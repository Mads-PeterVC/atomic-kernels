from __future__ import annotations

from ._utils import (
    bonds_from_adjacency,
    normalize_bonds,
    normalize_face_colors,
    normalize_faces,
    normalize_rgba,
    selection_mask,
)


class RenderController:
    """Manage rendering styles, bonds, and polygon faces."""

    def __init__(self, session: "ViewerSessionFacade") -> None:
        self._session = session

    def set_bonds(self, bonds, frame_index: int | None = None) -> None:
        """Store explicit bond connectivity for one frame.

        Parameters
        ----------
        bonds
            Iterable of atom-index pairs.
        frame_index : int or None, optional
            Frame index to update. ``None`` uses the current frame.
        """
        self._session._backend.set_bonds(
            normalize_bonds(bonds), frame_index=frame_index
        )

    def set_bonds_from_adjacency(self, adjacency, frame_index: int | None = None) -> None:
        """Store bonds from a square adjacency matrix.

        Parameters
        ----------
        adjacency
            Square boolean or numeric adjacency matrix.
        frame_index : int or None, optional
            Frame index to update. ``None`` uses the current frame.
        """
        self._session._backend.set_bonds(
            bonds_from_adjacency(adjacency), frame_index=frame_index
        )

    def set_faces(
        self,
        faces,
        color=(0.2, 0.6, 0.9, 0.35),
        face_colors=None,
        frame_index: int | None = None,
    ) -> None:
        """Store explicit polygon faces for one frame.

        Parameters
        ----------
        faces
            Iterable of face definitions expressed as atom-index sequences.
        color : tuple, default=(0.2, 0.6, 0.9, 0.35)
            Default RGBA color for all faces.
        face_colors : sequence, optional
            Per-face RGBA colors.
        frame_index : int or None, optional
            Frame index to update. ``None`` uses the current frame.
        """
        frame_index = self._session._resolve_frame_index(frame_index)
        normalized_faces = normalize_faces(faces)
        normalized_colors = normalize_face_colors(
            len(normalized_faces), color, face_colors=face_colors
        )
        self._session._backend.set_faces(
            normalized_faces,
            color=normalize_rgba(color),
            face_colors=normalized_colors,
            frame_index=frame_index,
        )

    def ball_and_stick(
        self,
        selection=None,
        atom_scale: float = 0.45,
        bond_radius: float = 0.08,
        bond_color=(0.7, 0.7, 0.7),
        bond_scope: str = "both_selected",
        frame_index: int | None = None,
        append: bool = False,
    ) -> None:
        """Apply ball-and-stick rendering to a whole frame or a selection.

        Parameters
        ----------
        selection, optional
            Selection expression restricting the style to a subset of atoms.
        atom_scale : float, default=0.45
            Relative atom radius for styled atoms.
        bond_radius : float, default=0.08
            Radius of rendered bonds.
        bond_color : tuple, default=(0.7, 0.7, 0.7)
            Bond color as RGB values.
        bond_scope : str, default="both_selected"
            Bond-selection rule passed to the render backend.
        frame_index : int or None, optional
            Frame index to update. ``None`` uses the current frame.
        append : bool, default=False
            If ``True``, append the style instead of replacing existing styles.
        """
        frame_index = self._session._resolve_frame_index(frame_index)
        atoms = self._session._frame(frame_index)
        mask = (
            selection_mask(atoms, selection)
            if selection is not None
            else [True] * len(atoms)
        )
        self._session._backend.set_ball_and_stick_style(
            list(bool(value) for value in mask),
            atom_scale=atom_scale,
            bond_radius=bond_radius,
            bond_color=normalize_rgba(bond_color),
            bond_scope=bond_scope,
            frame_index=frame_index,
            append=append,
        )

    def reset(self) -> None:
        """Restore pure space-filling rendering."""
        self._session._backend.reset_render_style()
