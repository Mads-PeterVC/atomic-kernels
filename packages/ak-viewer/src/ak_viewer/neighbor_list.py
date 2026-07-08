"""Neighbor-list helpers exposed by the Python package."""

import numpy as np
from ase import Atoms
from dataclasses import dataclass

from ._ak_viewer import neighborlist as rust_neighborlist


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


def neighbor_list(atoms: Atoms, cutoff: float, symmetrize: bool = True, sort: bool = False) -> NeighborList:
    """Compute a neighbor list for an ASE structure.

    Parameters
    ----------
    atoms : ase.Atoms
        Input structure.
    cutoff : float
        Pair cutoff distance in angstrom.
    symmetrize : bool, default=True
        If ``True``, return both ``i -> j`` and ``j -> i`` entries for each pair.
    sort : bool, default=False
        If ``True``, sort the output lexicographically by ``(i, j)``.

    Returns
    -------
    NeighborList
        Neighbor indices and periodic-image shifts for all pairs within the cutoff.
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
