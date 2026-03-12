"""Neighbor-list helpers exposed by the Python package."""

import numpy as np
from ase import Atoms
from dataclasses import dataclass

from ._atomic_kernels import neighborlist as rust_neighborlist


@dataclass
class NeighborList:
    """Neighbor-list arrays returned by :func:`neighbor_list`.

    Attributes:
        i: Source atom indices.
        j: Neighbor atom indices.
        S: Cell-shift vectors for periodic images.
    """

    i: list[int]
    j: list[int]
    S: list[float]


def neighbor_list(atoms: Atoms, cutoff: float, symmetrize: bool = True, sort: bool = False) -> NeighborList:
    """Compute a neighbor list for an ASE structure.

    Args:
        atoms: Input structure.
        cutoff: Pair cutoff distance in angstrom.
        symmetrize: Duplicate each edge in reverse order.
        sort: Sort the result lexicographically by ``(i, j)``.
    """
    i, j, S = rust_neighborlist(atoms, cutoff)

    if symmetrize:
        i_sym = np.concatenate([i, j])
        j_sym = np.concatenate([j, i])
        S_sym = np.concatenate([S, -S])
        i, j, S = i_sym, j_sym, S_sym
    
    if sort:
        # Sort by i, then j
        sorted_indices = np.lexsort((j, i))
        i = i[sorted_indices]
        j = j[sorted_indices]
        S = S[sorted_indices]


    return NeighborList(i, j, S)
