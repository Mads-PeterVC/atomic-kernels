from __future__ import annotations

import multiprocessing as mp
import threading
import time
from typing import Optional

from ase import Atoms

from atomic_kernels._atomic_kernels import (
    PreparedViewerSession,
    ViewerConfig,
    ViewerSession,
    prepare_viewer_session as _prepare_viewer_session,
)

from ._config import deserialize_config, serialize_config
from ._utils import normalize_colormap


def viewer_process_main(
    atoms: list[Atoms], config_payload: Optional[dict], connection
) -> None:
    config = deserialize_config(config_payload)
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
            elif command == "set_atom_scalars":
                session.set_atom_scalars(*payload)
            elif command == "color_by_scalar":
                session.color_by_scalar(*payload)
            elif command == "reset_atom_colors":
                session.reset_atom_colors()
            elif command == "set_bonds":
                session.set_bonds(*payload)
            elif command == "set_ball_and_stick_style":
                session.set_ball_and_stick_style(*payload)
            elif command == "reset_render_style":
                session.reset_render_style()
            elif command == "set_camera_view":
                session.set_camera_view(*payload)
            elif command == "pan_camera":
                session.pan_camera(payload)
            elif command == "zoom_camera":
                session.zoom_camera(*payload)
            elif command == "orbit_camera":
                session.orbit_camera(*payload)
            elif command == "frame_all":
                session.frame_all()
            elif command == "start_orbit":
                session.start_orbit(*payload)
            elif command == "stop_camera_motion":
                session.stop_camera_motion()
            elif command == "wait_until_ready":
                try:
                    ready = session.wait_until_ready(payload)
                except RuntimeError:
                    ready = False
                try:
                    connection.send(("wait_until_ready", ready))
                except (BrokenPipeError, EOFError, OSError):
                    break
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


class ViewerSessionProxy:
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

    def set_atom_scalars(
        self, name: str, values, frame_index: int | None = None
    ) -> None:
        self._send("set_atom_scalars", (name, list(values), frame_index))

    def color_by_scalar(
        self,
        name: str,
        palette: str = "viridis",
        colors=None,
        min: float | None = None,
        max: float | None = None,
        append: bool = False,
    ) -> None:
        normalized = None if colors is None else normalize_colormap(colors)
        self._send("color_by_scalar", (name, palette, normalized, min, max, append))

    def reset_atom_colors(self) -> None:
        self._send("reset_atom_colors")

    def set_bonds(self, bonds, frame_index: int | None = None) -> None:
        self._send("set_bonds", (list(bonds), frame_index))

    def set_ball_and_stick_style(
        self,
        selection,
        atom_scale: float = 0.45,
        bond_radius: float = 0.08,
        bond_color=(0.7, 0.7, 0.7, 1.0),
        bond_scope: str = "both_selected",
        frame_index: int | None = None,
        append: bool = False,
    ) -> None:
        self._send(
            "set_ball_and_stick_style",
            (
                list(selection),
                atom_scale,
                bond_radius,
                tuple(bond_color),
                bond_scope,
                frame_index,
                append,
            ),
        )

    def reset_render_style(self) -> None:
        self._send("reset_render_style")

    def set_camera_view(
        self,
        focus=None,
        radius: float | None = None,
        yaw: float | None = None,
        pitch: float | None = None,
    ) -> None:
        self._send("set_camera_view", (focus, radius, yaw, pitch))

    def pan_camera(self, delta) -> None:
        self._send("pan_camera", tuple(delta))

    def zoom_camera(
        self, factor: float | None = None, delta: float | None = None
    ) -> None:
        self._send("zoom_camera", (factor, delta))

    def orbit_camera(self, yaw_delta: float = 0.0, pitch_delta: float = 0.0) -> None:
        self._send("orbit_camera", (yaw_delta, pitch_delta))

    def frame_all(self) -> None:
        self._send("frame_all")

    def start_orbit(self, yaw_rate: float = 0.5, pitch_rate: float = 0.0) -> None:
        self._send("start_orbit", (yaw_rate, pitch_rate))

    def stop_camera_motion(self) -> None:
        self._send("stop_camera_motion")

    def close(self) -> None:
        if self._connection.closed:
            return
        try:
            self._send("close")
        finally:
            self._connection.close()

    def wait_until_ready(self, timeout: float | None = None) -> bool:
        if timeout is not None and timeout < 0:
            raise ValueError("timeout must be non-negative")

        self._send("wait_until_ready", timeout)
        deadline = None if timeout is None else time.monotonic() + timeout

        while True:
            if not self._process.is_alive() and not self._connection.poll():
                return False

            wait_time = 0.1
            if deadline is not None:
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    return False
                wait_time = min(wait_time, remaining)

            if not self._connection.poll(wait_time):
                continue

            message, payload = self._connection.recv()
            if message == "wait_until_ready":
                return bool(payload)


def spawn_process_viewer_session(
    atoms: list[Atoms],
    config: Optional[ViewerConfig],
) -> ViewerSessionProxy:
    ctx = mp.get_context("spawn")
    parent, child = ctx.Pipe()
    process = ctx.Process(
        target=viewer_process_main,
        args=(atoms, serialize_config(config), child),
        daemon=False,
    )
    process.start()
    child.close()
    return ViewerSessionProxy(parent, process)
