from __future__ import annotations

import numpy as np

from ._utils import selection_mask


class ScalarRangeTracker:
    """Track a running scalar range across multiple arrays or frames."""

    def __init__(self) -> None:
        self._min: float | None = None
        self._max: float | None = None

    def update(self, values) -> tuple[float, float]:
        """Expand the tracked range with the finite values in ``values``."""
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
        """Return the current ``(min, max)`` limits."""
        if self._min is None or self._max is None:
            raise ValueError("scalar range has not been initialized")
        return self._min, self._max


class ColorController:
    """Manage scalar fields and color mappings for the current viewer session."""

    def __init__(self, session: "ViewerSessionFacade") -> None:
        self._session = session

    def set_atom_scalars(
        self, name: str, values, frame_index: int | None = None
    ) -> None:
        """Store a per-atom scalar field for a specific frame or the current frame."""
        values = np.asarray(values, dtype=np.float32)
        self._session._backend.set_atom_scalars(
            name, values.tolist(), frame_index=frame_index
        )

    def by_scalar(
        self,
        name: str,
        palette: str = "viridis",
        colors=None,
        min: float | None = None,
        max: float | None = None,
        append: bool = False,
    ) -> None:
        """Color atoms by a named scalar field using a built-in or sampled colormap."""
        self._session._backend.color_by_scalar(
            name,
            palette=palette,
            colors=colors,
            min=min,
            max=max,
            append=append,
        )

    def reset(self) -> None:
        """Restore default element-based coloring."""
        self._session._backend.reset_atom_colors()

    def range_tracker(self) -> ScalarRangeTracker:
        """Create a helper for keeping a fixed scalar range across frames."""
        return ScalarRangeTracker()


class ViewerSelection:
    """Apply scalar and color operations to a subset of atoms."""

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
        return frame_index, selection_mask(atoms, self._selection)

    def set_atom_scalars(
        self, name: str, values, frame_index: int | None = None
    ) -> "ViewerSessionFacade":
        """Store a scalar field for the selection and mask all other atoms with ``NaN``."""
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

        self._session.colors().set_atom_scalars(
            name, scalar_values, frame_index=frame_index
        )
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
        """Color the selection by scalar values while leaving unselected atoms unchanged."""
        self.set_atom_scalars(name, values, frame_index=frame_index)
        self._session.colors().by_scalar(
            name,
            palette=palette,
            colors=colors,
            min=min,
            max=max,
            append=True,
        )
        return self._session

    def ball_and_stick(
        self,
        atom_scale: float = 0.45,
        bond_radius: float = 0.08,
        bond_color=(0.7, 0.7, 0.7),
        bond_scope: str = "both_selected",
        frame_index: int | None = None,
    ) -> "ViewerSessionFacade":
        """Render the selection in ball-and-stick style and leave other atoms unchanged."""
        self._session.render().ball_and_stick(
            selection=self._selection,
            atom_scale=atom_scale,
            bond_radius=bond_radius,
            bond_color=bond_color,
            bond_scope=bond_scope,
            frame_index=self._frame_index if frame_index is None else frame_index,
            append=True,
        )
        return self._session
