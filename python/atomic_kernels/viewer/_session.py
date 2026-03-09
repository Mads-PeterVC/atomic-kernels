from __future__ import annotations

from ase import Atoms

from ._camera import CameraController
from ._color import ColorController, ViewerSelection


class ViewerSessionFacade:
    """High-level handle for a running viewer session."""

    def __init__(self, backend, atoms: list[Atoms]):
        self._backend = backend
        self._frames = [frame.copy() for frame in atoms]
        self._current_frame = 0
        self._camera = CameraController(self)
        self._colors = ColorController(self)

    def _resolve_frame_index(self, frame_index: int | None = None) -> int:
        return self._current_frame if frame_index is None else frame_index

    def _frame(self, frame_index: int | None = None) -> Atoms:
        return self._frames[self._resolve_frame_index(frame_index)]

    def append_frame(self, frame: Atoms) -> None:
        """Append a new frame to the live trajectory."""
        self._backend.append_frame(frame)
        self._frames.append(frame.copy())

    def set_frame(self, index: int) -> None:
        """Switch the viewer to a specific frame index."""
        self._backend.set_frame(index)
        self._current_frame = index

    def follow_tail(self, enabled: bool = True) -> None:
        """Keep the viewer pinned to the newest frame as frames are appended."""
        self._backend.follow_tail(enabled)

    def close(self) -> None:
        """Close the running viewer session."""
        self._backend.close()

    def camera(self) -> CameraController:
        """Return the camera controller for this session."""
        return self._camera

    def colors(self) -> ColorController:
        """Return the color controller for this session."""
        return self._colors

    def select(
        self,
        selection,
        frame_index: int | None = None,
    ) -> ViewerSelection:
        """Create a selection object for subset-aware color operations."""
        return ViewerSelection(self, selection, frame_index=frame_index)
