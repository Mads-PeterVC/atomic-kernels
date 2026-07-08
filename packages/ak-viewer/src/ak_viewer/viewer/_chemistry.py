from __future__ import annotations

from collections import defaultdict

from ase import Atoms
from ase.neighborlist import natural_cutoffs, neighbor_list as ase_neighbor_list
import numpy as np
from scipy.spatial import ConvexHull, QhullError


from ._utils import bonds_from_neighbor_list, selection_mask
from ak_viewer.neighbor_list import NeighborList


def bonds_from_ase(
    atoms: Atoms,
    *,
    cutoff_multiplier: float = 1.2,
    selection=None,
) -> list[tuple[int, int]]:
    cutoffs = natural_cutoffs(atoms, mult=cutoff_multiplier)
    senders, receivers = ase_neighbor_list("ij", atoms, cutoffs)
    bonds = bonds_from_neighbor_list(NeighborList(i=senders, j=receivers, S=[]))
    if selection is None:
        return bonds

    mask = selection_mask(atoms, selection)
    return [(i, j) for i, j in bonds if mask[i] and mask[j]]


def polyhedra_from_neighbors(
    atoms: Atoms,
    neighbors,
    *,
    selection=None,
) -> list[list[int]]:
    center_mask = (
        np.ones(len(atoms), dtype=bool)
        if selection is None
        else selection_mask(atoms, selection)
    )
    adjacency: dict[int, set[int]] = defaultdict(set)
    for i, j in bonds_from_neighbor_list(neighbors):
        adjacency[i].add(j)
        adjacency[j].add(i)

    faces: list[list[int]] = []
    for center in np.flatnonzero(center_mask):
        ligands = sorted(adjacency.get(int(center), ()))
        if len(ligands) < 3:
            continue

        ligand_positions = np.asarray(atoms.positions[ligands], dtype=np.float64)
        relative = ligand_positions - np.asarray(atoms.positions[int(center)], dtype=np.float64)
        centered = relative - relative.mean(axis=0, keepdims=True)
        if np.linalg.matrix_rank(centered) < 2:
            continue
        if len(ligands) == 3:
            faces.append(sorted(ligands))
            continue

        try:
            hull = ConvexHull(relative)
        except QhullError:
            continue

        for simplex in hull.simplices:
            face = sorted(ligands[int(index)] for index in simplex)
            if len(face) >= 3:
                faces.append(face)

    return faces


def faces_from_coordination(
    atoms: Atoms,
    *,
    cutoff_multiplier: float = 1.2,
    selection=None,
) -> list[list[int]]:
    cutoffs = natural_cutoffs(atoms, mult=cutoff_multiplier)
    senders, receivers = ase_neighbor_list("ij", atoms, cutoffs)
    neighbors = NeighborList(i=senders, j=receivers, S=[])
    return polyhedra_from_neighbors(atoms, neighbors, selection=selection)
