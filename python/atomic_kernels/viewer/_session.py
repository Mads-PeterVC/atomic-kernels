from __future__ import annotations

from ase import Atoms

from ._camera import CameraController
from ._color import MaterialController, ViewerSelection
from ._render import RenderController
from ._utils import normalized_selection_mask


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
        self._materials = MaterialController(self)
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

    def selected_atoms(self, frame_index: int | None = None) -> list[int]:
        """Return the currently selected atom indices for a frame."""
        return list(self._backend.selected_atoms(self._resolve_frame_index(frame_index)))

    def selected_images(self, frame_index: int | None = None) -> list[dict]:
        """Return the currently selected image-aware atoms for a frame."""
        return [
            {"atom_index": atom_index, "image_offset": tuple(image_offset)}
            for atom_index, image_offset in self._backend.selected_images(
                self._resolve_frame_index(frame_index)
            )
        ]

    def set_selection(self, selection, frame_index: int | None = None) -> None:
        """Replace the shared live selection for a frame."""
        resolved = self._resolve_frame_index(frame_index)
        self._backend.set_selection(
            normalized_selection_mask(self._frame(resolved), selection),
            frame_index=resolved,
        )

    def add_selection(self, selection, frame_index: int | None = None) -> None:
        """Add atoms to the shared live selection for a frame."""
        resolved = self._resolve_frame_index(frame_index)
        self._backend.add_selection(
            normalized_selection_mask(self._frame(resolved), selection),
            frame_index=resolved,
        )

    def remove_selection(self, selection, frame_index: int | None = None) -> None:
        """Remove atoms from the shared live selection for a frame."""
        resolved = self._resolve_frame_index(frame_index)
        self._backend.remove_selection(
            normalized_selection_mask(self._frame(resolved), selection),
            frame_index=resolved,
        )

    def clear_selection(self, frame_index: int | None = None) -> None:
        """Clear the shared live selection for a frame."""
        self._backend.clear_selection(frame_index=self._resolve_frame_index(frame_index))

    def set_image_selection(
        self, selection: list[tuple[int, tuple[int, int, int]]], frame_index: int | None = None
    ) -> None:
        """Replace the image-aware selection for a frame."""
        self._backend.set_image_selection(
            list(selection), frame_index=self._resolve_frame_index(frame_index)
        )

    def add_image_selection(
        self, selection: list[tuple[int, tuple[int, int, int]]], frame_index: int | None = None
    ) -> None:
        """Add image-aware atoms to the selection for a frame."""
        self._backend.add_image_selection(
            list(selection), frame_index=self._resolve_frame_index(frame_index)
        )

    def remove_image_selection(
        self, selection: list[tuple[int, tuple[int, int, int]]], frame_index: int | None = None
    ) -> None:
        """Remove image-aware atoms from the selection for a frame."""
        self._backend.remove_image_selection(
            list(selection), frame_index=self._resolve_frame_index(frame_index)
        )

    def clear_image_selection(self, frame_index: int | None = None) -> None:
        """Clear the image-aware selection for a frame."""
        self._backend.clear_image_selection(
            frame_index=self._resolve_frame_index(frame_index)
        )

    def set_supercell(self, repeats: tuple[int, int, int]) -> None:
        """Set symmetric per-axis repeat extents for displayed periodic images."""
        self._backend.set_supercell(tuple(int(value) for value in repeats))

    def increment_supercell_axis(self, axis: int) -> None:
        """Increase the symmetric repeat extent for one lattice axis."""
        self._backend.increment_supercell_axis(int(axis))

    def decrement_supercell_axis(self, axis: int) -> None:
        """Decrease the symmetric repeat extent for one lattice axis."""
        self._backend.decrement_supercell_axis(int(axis))

    def reset_supercell(self) -> None:
        """Reset the viewer to the base cell without repeated images."""
        self._backend.reset_supercell()

    def set_ghost_repeated_images(self, enabled: bool = True) -> None:
        """Set whether repeated-only images should be ghosted."""
        self._backend.set_ghost_repeated_images(enabled)

    def toggle_supercell_distinction(self) -> None:
        """Toggle ghosting of repeated-only images."""
        self._backend.toggle_ghost_repeated_images()

    def supercell(self) -> dict:
        """Return the current supercell display settings."""
        repeats, ghosted = self._backend.supercell()
        return {"repeats": tuple(repeats), "ghost_repeated_images": bool(ghosted)}

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

    def materials(self) -> MaterialController:
        """Return the appearance controller for this session.

        Returns
        -------
        MaterialController
            Appearance control facade bound to this session.
        """
        return self._materials

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
            Selection helper that scopes appearance and render changes to a subset of atoms.
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
