from __future__ import annotations

from ase import Atoms
import numpy as np

from ._utils import selection_mask


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
        return frame_index, selection_mask(atoms, self._selection)

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
