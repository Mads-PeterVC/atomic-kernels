from __future__ import annotations

from ase import Atoms
import numpy as np


def normalize_atoms(atoms: Atoms | list[Atoms]) -> list[Atoms]:
    if isinstance(atoms, Atoms):
        return [atoms]
    return atoms


def normalize_colormap(colors) -> list[tuple[float, float, float, float]]:
    array = np.asarray(colors, dtype=np.float32)
    if array.ndim != 2 or array.shape[1] not in (3, 4):
        raise ValueError("colors must have shape (N, 3) or (N, 4)")
    if array.shape[0] < 2:
        raise ValueError("colors must contain at least two samples")

    if array.shape[1] == 3:
        alpha = np.ones((array.shape[0], 1), dtype=np.float32)
        array = np.concatenate([array, alpha], axis=1)

    return [tuple(row) for row in array.tolist()]


def selection_mask(atoms: Atoms, selection) -> np.ndarray:
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
