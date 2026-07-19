from ase import Atoms
from ak_neighborlist import neighbor_list as ak_neighbor_list
import numpy as np
from timeit import default_timer as dt

from ase.neighborlist import neighbor_list as ase_neighbor_list

def make_atoms(n: int = 3, box_size=10.0, seed=None) -> Atoms:
    if seed is not None:
        np.random.seed(seed)
    positions = np.random.rand(n, 3) * box_size
    numbers = np.random.randint(1, 10, size=n)
    cell = np.eye(3) * box_size
    pbc = [True, False, False]
    atoms = Atoms(positions=positions, numbers=numbers, cell=cell, pbc=pbc)
    return atoms


atoms = make_atoms(n=2, box_size=2, seed=45)


nl = ak_neighbor_list(atoms, cutoff=3.0, symmetrize=False, sort=True)
ase_i, ase_j, ase_S = ase_neighbor_list("ijS", atoms, cutoff=3.0, self_interaction=False)

# Sort ASE neighbor list by i, then j
sorted_indices = np.lexsort((ase_j, ase_i))
ase_i = ase_i[sorted_indices]
ase_j = ase_j[sorted_indices]
ase_S = ase_S[sorted_indices]


print(f'Atomic Kernels Neighbor List: {nl.S.shape}')
for idx in range(len(nl.S)):

    d = atoms.positions[nl.i[idx]] - (atoms.positions[nl.j[idx]] + nl.S[idx] @ atoms.cell)
    dist = np.linalg.norm(d)

    print(f'{nl.i[idx]} {nl.j[idx]} {nl.S[idx]} {dist}')


print(f'ASE Neighbor List: {ase_S.shape}')
for idx in range(len(ase_S)):

    d = atoms.positions[ase_i[idx]] - (atoms.positions[ase_j[idx]] + ase_S[idx] @ atoms.cell)
    dist = np.linalg.norm(d)

    print(f'{ase_i[idx]} {ase_j[idx]} {ase_S[idx]} {dist}')









