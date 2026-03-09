from __future__ import annotations

import multiprocessing as mp
import sys
import threading
from typing import Optional

from ase import Atoms

from atomic_kernels._atomic_kernels import (
    ColorConfig,
    LightingConfig,
    PreparedViewerSession,
    RenderConfig,
    ViewerConfig,
    ViewerSession,
    launch_viewer as _launch_viewer,
    prepare_viewer_session as _prepare_viewer_session,
    run_viewer_session as _run_viewer_session,
    trajectory_viewer,
)


def _normalize_atoms(atoms: Atoms | list[Atoms]) -> list[Atoms]:
    if isinstance(atoms, Atoms):
        return [atoms]
    return atoms


def _serialize_config(config: Optional[ViewerConfig]) -> Optional[dict]:
    if config is None:
        return None

    return {
        "color": {
            "background": config.color.background,
            "cell_color": config.color.cell_color,
        },
        "lighting": {
            "ambient_brightness": config.lighting.ambient_brightness,
            "key_illuminance": config.lighting.key_illuminance,
            "fill_illuminance": config.lighting.fill_illuminance,
            "back_illuminance": config.lighting.back_illuminance,
            "camera_illuminance": config.lighting.camera_illuminance,
            "enable_fog": config.lighting.enable_fog,
        },
        "render": {
            "show_cell": config.render.show_cell,
            "show_axes": config.render.show_axes,
            "show_ui": config.render.show_ui,
            "ico_subdiv": config.render.ico_subdiv,
        },
        "initial_frame": config.initial_frame,
    }


def _deserialize_config(payload: Optional[dict]) -> Optional[ViewerConfig]:
    if payload is None:
        return None

    return ViewerConfig(
        color=ColorConfig(
            background=payload["color"]["background"],
            cell_color=payload["color"]["cell_color"],
        ),
        lighting=LightingConfig(**payload["lighting"]),
        render=RenderConfig(**payload["render"]),
        initial_frame=payload["initial_frame"],
    )


def _viewer_process_main(
    atoms: list[Atoms], config_payload: Optional[dict], connection
) -> None:
    config = _deserialize_config(config_payload)
    prepared = _prepare_viewer_session(atoms, config)
    session = prepared.session

    def command_loop() -> None:
        while True:
            try:
                command, payload = connection.recv()
            except EOFError:
                break
            except OSError:
                break

            if command == "append_frame":
                session.append_frame(payload)
            elif command == "set_frame":
                session.set_frame(payload)
            elif command == "follow_tail":
                session.follow_tail(payload)
            elif command == "close":
                try:
                    session.close()
                except RuntimeError:
                    pass
                break

    worker = threading.Thread(target=command_loop)
    worker.start()
    prepared.run()
    connection.close()
    worker.join()


class _ViewerSessionProxy:
    def __init__(self, connection, process: mp.Process):
        self._connection = connection
        self._process = process

    def _send(self, command: str, payload=None) -> None:
        if not self._process.is_alive():
            raise RuntimeError("viewer session process is no longer running")
        self._connection.send((command, payload))

    def append_frame(self, frame: Atoms) -> None:
        self._send("append_frame", frame)

    def set_frame(self, index: int) -> None:
        self._send("set_frame", index)

    def follow_tail(self, enabled: bool = True) -> None:
        self._send("follow_tail", enabled)

    def close(self) -> None:
        if self._connection.closed:
            return
        try:
            self._send("close")
        finally:
            self._connection.close()


def bevy_viewer(atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None) -> None:
    trajectory_viewer(_normalize_atoms(atoms), config)


def launch_viewer(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> ViewerSession:
    return _launch_viewer(_normalize_atoms(atoms), config)


def run_viewer_session(
    atoms: Atoms | list[Atoms], callback, config: Optional[ViewerConfig] = None
) -> None:
    _run_viewer_session(_normalize_atoms(atoms), callback, config)


def prepare_viewer_session(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> PreparedViewerSession:
    return _prepare_viewer_session(_normalize_atoms(atoms), config)


def viewer_session(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> ViewerSession:
    atoms = _normalize_atoms(atoms)

    if sys.platform != "darwin":
        return _launch_viewer(atoms, config)

    ctx = mp.get_context("spawn")
    parent, child = ctx.Pipe()
    process = ctx.Process(
        target=_viewer_process_main,
        args=(atoms, _serialize_config(config), child),
        daemon=False,
    )
    process.start()
    child.close()

    return _ViewerSessionProxy(parent, process)
