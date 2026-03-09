from __future__ import annotations

import multiprocessing as mp
import sys
import threading
from typing import Optional

from ase import Atoms
import numpy as np

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


def _normalize_colormap(colors) -> list[tuple[float, float, float, float]]:
    array = np.asarray(colors, dtype=np.float32)
    if array.ndim != 2 or array.shape[1] not in (3, 4):
        raise ValueError("colors must have shape (N, 3) or (N, 4)")
    if array.shape[0] < 2:
        raise ValueError("colors must contain at least two samples")

    if array.shape[1] == 3:
        alpha = np.ones((array.shape[0], 1), dtype=np.float32)
        array = np.concatenate([array, alpha], axis=1)

    return [tuple(row) for row in array.tolist()]


def _selection_mask(atoms: Atoms, selection) -> np.ndarray:
    raw = selection(atoms) if callable(selection) else selection
    mask = np.asarray(raw)

    if mask.dtype == bool:
        if mask.shape != (len(atoms),):
            raise ValueError(f"boolean selection must have shape ({len(atoms)},)")
        return mask

    indices = np.asarray(mask, dtype=np.int64).ravel()
    resolved = np.zeros(len(atoms), dtype=bool)
    resolved[indices] = True
    return resolved


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
            elif command == "set_atom_scalars":
                session.set_atom_scalars(*payload)
            elif command == "color_by_scalar":
                session.color_by_scalar(*payload)
            elif command == "reset_atom_colors":
                session.reset_atom_colors()
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
        normalized = None if colors is None else _normalize_colormap(colors)
        self._send("color_by_scalar", (name, palette, normalized, min, max, append))

    def reset_atom_colors(self) -> None:
        self._send("reset_atom_colors")

    def close(self) -> None:
        if self._connection.closed:
            return
        try:
            self._send("close")
        finally:
            self._connection.close()


class ViewerSelection:
    def __init__(
        self,
        session: "ViewerSessionFacade",
        selection,
        frame_index: int | None = None,
    ) -> None:
        self._session = session
        self._selection = selection
        self._frame_index = frame_index

    def _resolve(self, frame_index: int | None = None) -> tuple[int, np.ndarray]:
        frame_index = self._session._resolve_frame_index(
            self._frame_index if frame_index is None else frame_index
        )
        atoms = self._session._frame(frame_index)
        return frame_index, _selection_mask(atoms, self._selection)

    def set_atom_scalars(
        self, name: str, values, frame_index: int | None = None
    ) -> "ViewerSessionFacade":
        frame_index, mask = self._resolve(frame_index)
        atoms = self._session._frame(frame_index)
        scalar_values = np.full(len(atoms), np.nan, dtype=np.float32)
        values = np.asarray(values, dtype=np.float32)

        if values.ndim == 0:
            scalar_values[mask] = float(values)
        elif values.shape == (len(atoms),):
            scalar_values[mask] = values[mask]
        elif values.shape == (int(mask.sum()),):
            scalar_values[mask] = values
        else:
            raise ValueError(
                "values must be a scalar, a full-length array, or an array matching the selection size"
            )

        self._session.set_atom_scalars(name, scalar_values, frame_index=frame_index)
        return self._session

    def color_by_scalar(
        self,
        name: str,
        values,
        palette: str = "viridis",
        colors=None,
        min: float | None = None,
        max: float | None = None,
        frame_index: int | None = None,
    ) -> "ViewerSessionFacade":
        self.set_atom_scalars(name, values, frame_index=frame_index)
        self._session.color_by_scalar(
            name,
            palette=palette,
            colors=colors,
            min=min,
            max=max,
            append=True,
        )
        return self._session


class ScalarRangeTracker:
    def __init__(self) -> None:
        self._min: float | None = None
        self._max: float | None = None

    def update(self, values) -> tuple[float, float]:
        array = np.asarray(values, dtype=np.float32)
        finite = array[np.isfinite(array)]
        if finite.size == 0:
            raise ValueError("values must contain at least one finite scalar")

        current_min = float(finite.min())
        current_max = float(finite.max())
        self._min = current_min if self._min is None else min(self._min, current_min)
        self._max = current_max if self._max is None else max(self._max, current_max)
        return self.limits

    @property
    def limits(self) -> tuple[float, float]:
        if self._min is None or self._max is None:
            raise ValueError("scalar range has not been initialized")
        return self._min, self._max


class ViewerSessionFacade:
    def __init__(self, backend, atoms: list[Atoms]):
        self._backend = backend
        self._frames = [frame.copy() for frame in atoms]
        self._current_frame = 0

    def _resolve_frame_index(self, frame_index: int | None = None) -> int:
        return self._current_frame if frame_index is None else frame_index

    def _frame(self, frame_index: int | None = None) -> Atoms:
        return self._frames[self._resolve_frame_index(frame_index)]

    def append_frame(self, frame: Atoms) -> None:
        self._backend.append_frame(frame)
        self._frames.append(frame.copy())

    def set_frame(self, index: int) -> None:
        self._backend.set_frame(index)
        self._current_frame = index

    def follow_tail(self, enabled: bool = True) -> None:
        self._backend.follow_tail(enabled)

    def set_atom_scalars(
        self, name: str, values, frame_index: int | None = None
    ) -> None:
        values = np.asarray(values, dtype=np.float32)
        self._backend.set_atom_scalars(name, values.tolist(), frame_index=frame_index)

    def color_by_scalar(
        self,
        name: str,
        palette: str = "viridis",
        colors=None,
        min: float | None = None,
        max: float | None = None,
        append: bool = False,
    ) -> None:
        self._backend.color_by_scalar(
            name,
            palette=palette,
            colors=colors,
            min=min,
            max=max,
            append=append,
        )

    def reset_atom_colors(self) -> None:
        self._backend.reset_atom_colors()

    def close(self) -> None:
        self._backend.close()

    def select(
        self,
        selection,
        frame_index: int | None = None,
    ) -> ViewerSelection:
        return ViewerSelection(self, selection, frame_index=frame_index)

    def scalar_range_tracker(self) -> ScalarRangeTracker:
        return ScalarRangeTracker()


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
) -> ViewerSessionFacade:
    atoms = _normalize_atoms(atoms)

    if sys.platform != "darwin":
        return ViewerSessionFacade(_launch_viewer(atoms, config), atoms)

    ctx = mp.get_context("spawn")
    parent, child = ctx.Pipe()
    process = ctx.Process(
        target=_viewer_process_main,
        args=(atoms, _serialize_config(config), child),
        daemon=False,
    )
    process.start()
    child.close()

    return ViewerSessionFacade(_ViewerSessionProxy(parent, process), atoms)
