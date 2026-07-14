"""Neighbor-list helpers exposed by the Python package."""

from dataclasses import dataclass

import numpy as np
from ase import Atoms

from ._ak_neighborlist import neighborlist as rust_neighborlist


@dataclass
class NeighborList:
    """Neighbor-list arrays returned by :func:`neighbor_list`.

    Attributes
    ----------
    i : list[int]
        Source atom indices.
    j : list[int]
        Neighbor atom indices paired with ``i``.
    S : list[float]
        Periodic-image shift vectors for each edge.
    """

    i: list[int]
    j: list[int]
    S: list[float]


def neighbor_list(
    atoms: Atoms, cutoff: float, symmetrize: bool = True, sort: bool = False
) -> NeighborList:
    """Compute a neighbor list for an ASE structure."""
    i, j, S = rust_neighborlist(atoms, cutoff)

    if symmetrize:
        i_sym = np.concatenate([i, j])
        j_sym = np.concatenate([j, i])
        S_sym = np.concatenate([S, -S])
        i, j, S = i_sym, j_sym, S_sym

    if sort:
        sorted_indices = np.lexsort((j, i))
        i = i[sorted_indices]
        j = j[sorted_indices]
        S = S[sorted_indices]

    return NeighborList(i, j, S)
