from __future__ import annotations

import numpy as np

from ._utils import selection_mask


class ScalarRangeTracker:
    """Track a scalar range across multiple arrays or frames."""

    def __init__(self) -> None:
        self._min: float | None = None
        self._max: float | None = None

    def update(self, values) -> tuple[float, float]:
        """Expand the tracked range with finite values from an array.

        Parameters
        ----------
        values
            Array-like scalar values.

        Returns
        -------
        tuple[float, float]
            Updated ``(min, max)`` limits.
        """
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


class MaterialController:
    """Manage scalar fields and appearance mappings for the current viewer session."""

    def __init__(self, session: "ViewerSessionFacade") -> None:
        self._session = session

    def set_atom_scalars(
        self, name: str, values, frame_index: int | None = None
    ) -> None:
        """Store a per-atom scalar field.

        Parameters
        ----------
        name : str
            Scalar field name.
        values
            Per-atom scalar values for the target frame.
        frame_index : int or None, optional
            Frame index to update. ``None`` uses the current frame.
        """
        values = np.asarray(values, dtype=np.float32)
        self._session._backend.set_atom_scalars(
            name, values.tolist(), frame_index=frame_index
        )

    def by_scalar(
        self,
        name: str,
        channel: str,
        palette: str = "viridis",
        colors=None,
        min: float | None = None,
        max: float | None = None,
        append: bool = False,
    ) -> None:
        """Map a named scalar field into an appearance channel.

        Parameters
        ----------
        name : str
            Scalar field name to visualize.
        channel : {"color", "metallic", "perceptual_roughness"}
            Appearance channel to drive from the scalar field.
        palette : str, default="viridis"
            Built-in colormap name. Only valid for ``channel="color"``.
        colors : sequence, optional
            Explicit sampled colors used instead of a built-in palette for ``channel="color"``.
        min : float or None, optional
            Lower bound for scalar normalization.
        max : float or None, optional
            Upper bound for scalar normalization.
        append : bool, default=False
            If ``True``, append the rule instead of replacing existing color rules.
        """
        if channel != "color" and colors is not None:
            raise ValueError("colors can only be provided when channel='color'")
        if channel != "color" and palette != "viridis":
            raise ValueError("palette can only be customized when channel='color'")

        self._session._backend.material_by_scalar(
            name,
            channel=channel,
            palette=palette,
            colors=colors,
            min=min,
            max=max,
            append=append,
        )

    def reset(self, channel: str | None = None) -> None:
        """Restore palette-based appearance defaults for one channel or all channels."""
        self._session._backend.reset_atom_materials(channel=channel)

    def range_tracker(self) -> ScalarRangeTracker:
        """Create a helper for keeping a fixed scalar range across frames.

        Returns
        -------
        ScalarRangeTracker
            Tracker for accumulating global scalar limits.
        """
        return ScalarRangeTracker()


class ViewerSelection:
    """Apply scalar and appearance operations to a subset of atoms."""

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
        """Store a scalar field for the selection.

        Unselected atoms are masked with ``NaN`` so later scalar coloring can
        leave them visually unchanged.

        Parameters
        ----------
        name : str
            Scalar field name.
        values
            Scalar value, full-length array, or selection-length array.
        frame_index : int or None, optional
            Frame index to update. ``None`` uses the selection's default frame.

        Returns
        -------
        ViewerSessionFacade
            Session facade to support fluent scripting.
        """
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

        self._session.materials().set_atom_scalars(
            name, scalar_values, frame_index=frame_index
        )
        return self._session

    def material_by_scalar(
        self,
        name: str,
        values,
        channel: str,
        palette: str = "viridis",
        colors=None,
        min: float | None = None,
        max: float | None = None,
        frame_index: int | None = None,
    ) -> "ViewerSessionFacade":
        """Map selection scalars into one appearance channel while leaving other atoms unchanged.

        Parameters
        ----------
        name : str
            Scalar field name.
        values
            Scalar value, full-length array, or selection-length array.
        channel : {"color", "metallic", "perceptual_roughness"}
            Appearance channel to drive from the scalar field.
        palette : str, default="viridis"
            Built-in colormap name when ``channel="color"``.
        colors : sequence, optional
            Explicit sampled colors used instead of a built-in palette for ``channel="color"``.
        min : float or None, optional
            Lower bound for scalar normalization.
        max : float or None, optional
            Upper bound for scalar normalization.
        frame_index : int or None, optional
            Frame index to update. ``None`` uses the selection's default frame.

        Returns
        -------
        ViewerSessionFacade
            Session facade to support fluent scripting.
        """
        self.set_atom_scalars(name, values, frame_index=frame_index)
        self._session.materials().by_scalar(
            name,
            channel=channel,
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
        """Apply ball-and-stick rendering to the selection.

        Parameters
        ----------
        atom_scale : float, default=0.45
            Relative atom radius for selected atoms.
        bond_radius : float, default=0.08
            Radius of rendered bonds.
        bond_color : tuple, default=(0.7, 0.7, 0.7)
            Bond color as RGB values.
        bond_scope : str, default="both_selected"
            Bond-selection rule passed to the render backend.
        frame_index : int or None, optional
            Frame index to update. ``None`` uses the selection's default frame.

        Returns
        -------
        ViewerSessionFacade
            Session facade to support fluent scripting.
        """
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
