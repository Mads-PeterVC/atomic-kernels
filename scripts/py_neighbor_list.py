from ase import Atoms
from atomic_kernels import neighborlist
from ase.neighborlist import neighbor_list, NeighborList
import numpy as np
from timeit import default_timer as dt
from matscipy.neighbours import neighbour_list as matscipy_neighbor_list

def make_atoms(n: int = 3, box_size=10.0) -> Atoms:
    positions = np.random.rand(n, 3) * box_size
    numbers = np.random.randint(1, 10, size=n)
    cell = np.eye(3) * box_size
    pbc = [False, False, False]
    atoms = Atoms(positions=positions, numbers=numbers, cell=cell, pbc=pbc)
    return atoms


atoms = make_atoms(n=20000, box_size=100)

t0 = dt()
P = np.ascontiguousarray(atoms.positions)
numbers = np.ascontiguousarray(atoms.numbers, dtype=np.int32)
cell = np.ascontiguousarray(atoms.cell)
pbc = np.array(atoms.pbc)
i, j, shifts = neighborlist(P, numbers, cell, pbc, cutoff=2.0)

## Symmetrize the neighbor list
i_sym = np.concatenate([i, j])
j_sym = np.concatenate([j, i])
shifts_sym = np.concatenate([shifts, -shifts])
t1 = dt()

## Compare with ASE's neighbor list
t2 = dt()
# i_ase, j_ase, shifts_ase = neighbor_list("ijS", atoms, cutoff=2.0)
nl = NeighborList([2.0 / 2] * len(atoms), self_interaction=False, bothways=True)
nl.update(atoms)
# i_ase, j_ase, shifts_ase = nl.get_neighbors(list(range(len(atoms)))
t3 = dt()

## Compare with matscipy neighbor list
t4 = dt()
i_matscipy, j_matscipy, shifts_matscipy = matscipy_neighbor_list("ijS", atoms, cutoff=2.0)
t5 = dt()

# Sort the neighbor lists for comparison
# def sort_neighbors(i, j, shifts):
#     idx = np.lexsort((shifts[:, 2], shifts[:, 1], shifts[:, 0], j, i))
#     return i[idx], j[idx], shifts[idx]

# i_sym, j_sym, shifts_sym = sort_neighbors(i_sym, j_sym, shifts_sym)
# i_ase, j_ase, shifts_ase = sort_neighbors(i_ase, j_ase, shifts_ase)

# assert np.array_equal(i_sym, i_ase)
# assert np.array_equal(j_sym, j_ase)
# assert np.array_equal(shifts_sym, shifts_ase)
# print("Neighbor lists from atomic_kernels and ASE match successfully.")
print(f"atomic_kernels neighbor list time: {t1 - t0:.4f} seconds")
print(f"ASE neighbor list time: {t3 - t2:.4f} seconds")
print(f"matscipy neighbor list time: {t5 - t4:.4f} seconds")    