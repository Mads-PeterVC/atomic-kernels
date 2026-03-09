import numpy as np
from ._atomic_kernels import neighborlist as rust_neighborlist
from ase import Atoms
from dataclasses import dataclass

@dataclass
class NeighborList:
    i: list[int]
    j: list[int]
    S: list[float]


def neighbor_list(atoms: Atoms, cutoff: float, symmetrize: bool = True, sort: bool = False) -> NeighborList:
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





