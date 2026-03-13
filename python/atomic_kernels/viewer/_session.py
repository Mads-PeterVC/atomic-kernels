from __future__ import annotations

from ase import Atoms

from ._camera import CameraController
from ._color import ColorController, ViewerSelection
from ._render import RenderController


class ViewerSessionFacade:
    """High-level handle for a running viewer session.

    This facade keeps a Python-side copy of the loaded frames and exposes
    controller objects for camera, color, render, and selection operations.
    """

    def __init__(self, backend, atoms: list[Atoms]):
        self._backend = backend
        self._frames = [frame.copy() for frame in atoms]
        self._current_frame = 0
        self._camera = CameraController(self)
        self._colors = ColorController(self)
        self._render = RenderController(self)

    def _resolve_frame_index(self, frame_index: int | None = None) -> int:
        return self._current_frame if frame_index is None else frame_index

    def _frame(self, frame_index: int | None = None) -> Atoms:
        return self._frames[self._resolve_frame_index(frame_index)]

    def append_frame(self, frame: Atoms) -> None:
        """Append a new frame to the live trajectory.

        Parameters
        ----------
        frame : ase.Atoms
            Structure appended to the currently loaded trajectory.
        """
        self._backend.append_frame(frame)
        self._frames.append(frame.copy())

    def set_frame(self, index: int) -> None:
        """Switch the viewer to a specific frame index.

        Parameters
        ----------
        index : int
            Zero-based frame index to display.
        """
        self._backend.set_frame(index)
        self._current_frame = index

    def follow_tail(self, enabled: bool = True) -> None:
        """Keep the viewer pinned to the newest frame as frames are appended.

        Parameters
        ----------
        enabled : bool, default=True
            Whether follow-tail behavior should be enabled.
        """
        self._backend.follow_tail(enabled)

    def close(self) -> None:
        """Close the running viewer session."""
        self._backend.close()

    def wait_until_ready(self, timeout: float | None = None) -> bool:
        """Wait until the viewer reports readiness or the timeout elapses.

        Parameters
        ----------
        timeout : float or None, optional
            Timeout in seconds. ``None`` waits indefinitely.

        Returns
        -------
        bool
            ``True`` if the viewer became ready before the timeout expired.
        """
        return self._backend.wait_until_ready(timeout)

    def camera(self) -> CameraController:
        """Return the camera controller for this session.

        Returns
        -------
        CameraController
            Camera control facade bound to this session.
        """
        return self._camera

    def colors(self) -> ColorController:
        """Return the color controller for this session.

        Returns
        -------
        ColorController
            Color control facade bound to this session.
        """
        return self._colors

    def render(self) -> RenderController:
        """Return the rendering controller for this session.

        Returns
        -------
        RenderController
            Render control facade bound to this session.
        """
        return self._render

    def select(
        self,
        selection,
        frame_index: int | None = None,
    ) -> ViewerSelection:
        """Create a selection object for subset-aware operations.

        Parameters
        ----------
        selection
            Selection expression understood by :func:`selection_mask`.
        frame_index : int or None, optional
            Frame index used when resolving the selection. ``None`` uses the current frame.

        Returns
        -------
        ViewerSelection
            Selection helper that scopes color and render changes to a subset of atoms.
        """
        return ViewerSelection(self, selection, frame_index=frame_index)


class PreparedHeadlessRenderFacade(ViewerSessionFacade):
    """High-level facade for a prepared headless render session."""

    def __init__(self, prepared_backend, atoms: list[Atoms]):
        self._prepared_backend = prepared_backend
        super().__init__(prepared_backend.session, atoms)

    def save(self) -> None:
        """Render the scripted scene offscreen and write the PNG."""
        self._prepared_backend.save()
