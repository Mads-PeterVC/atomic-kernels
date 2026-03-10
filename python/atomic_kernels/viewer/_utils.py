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


def normalize_rgba(color) -> tuple[float, float, float, float]:
    array = np.asarray(color, dtype=np.float32).ravel()
    if array.shape not in ((3,), (4,)):
        raise ValueError("bond_color must have shape (3,) or (4,)")
    if array.shape == (3,):
        array = np.concatenate([array, np.array([1.0], dtype=np.float32)])
    return tuple(float(value) for value in array.tolist())


def _canonical_face_key(face: list[int]) -> tuple[int, ...]:
    rotations = [tuple(face[index:] + face[:index]) for index in range(len(face))]
    reversed_face = list(reversed(face))
    rotations.extend(
        tuple(reversed_face[index:] + reversed_face[:index])
        for index in range(len(reversed_face))
    )
    return min(rotations)


def normalize_bonds(bonds) -> list[tuple[int, int]]:
    array = np.asarray(bonds)
    if array.ndim == 2 and array.shape[0] == array.shape[1]:
        return bonds_from_adjacency(array)

    pairs = np.asarray(bonds, dtype=np.int64)
    if pairs.ndim != 2 or pairs.shape[1] != 2:
        raise ValueError("bonds must be an adjacency matrix or an array of shape (N, 2)")

    canonical = {
        (int(min(i, j)), int(max(i, j)))
        for i, j in pairs.tolist()
        if int(i) != int(j)
    }
    return sorted(canonical)


def bonds_from_adjacency(adjacency) -> list[tuple[int, int]]:
    matrix = np.asarray(adjacency)
    if matrix.ndim != 2 or matrix.shape[0] != matrix.shape[1]:
        raise ValueError("adjacency must be a square matrix")

    matrix = matrix.astype(bool)
    bonds = []
    for i in range(matrix.shape[0]):
        for j in range(i + 1, matrix.shape[1]):
            if matrix[i, j] or matrix[j, i]:
                bonds.append((int(i), int(j)))
    return bonds


def normalize_faces(faces) -> list[list[int]]:
    normalized = []
    seen = set()
    for face in faces:
        atoms = np.asarray(face, dtype=np.int64).ravel()
        if atoms.size < 3:
            raise ValueError("faces must contain at least three atom indices")

        resolved = [int(value) for value in atoms.tolist()]
        if len(set(resolved)) != len(resolved):
            raise ValueError("faces must not repeat atom indices")

        key = _canonical_face_key(resolved)
        if key in seen:
            continue
        seen.add(key)
        normalized.append(resolved)
    return normalized


def normalize_face_colors(face_count: int, color, face_colors=None):
    if face_colors is None:
        return [normalize_rgba(color)] * face_count

    if len(face_colors) != face_count:
        raise ValueError("face_colors must have the same length as faces")

    return [normalize_rgba(face_color) for face_color in face_colors]


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
