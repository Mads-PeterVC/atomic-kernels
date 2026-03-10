from __future__ import annotations

from ._utils import bonds_from_adjacency, normalize_bonds, normalize_rgba, selection_mask


class RenderController:
    """Manage rendering styles and explicit bond connectivity."""

    def __init__(self, session: "ViewerSessionFacade") -> None:
        self._session = session

    def set_bonds(self, bonds, frame_index: int | None = None) -> None:
        """Store explicit bond connectivity for one frame."""
        self._session._backend.set_bonds(
            normalize_bonds(bonds), frame_index=frame_index
        )

    def set_bonds_from_adjacency(self, adjacency, frame_index: int | None = None) -> None:
        """Store bonds from a square adjacency matrix."""
        self._session._backend.set_bonds(
            bonds_from_adjacency(adjacency), frame_index=frame_index
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
        """Apply ball-and-stick rendering to a whole frame or a selection."""
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
