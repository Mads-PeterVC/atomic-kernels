from ase import Atoms

from atomic_kernels import viewer
from atomic_kernels.atoms_to_arr import extract_arrays

def bevy_viewer(atoms: Atoms) -> None:
    P, numbers, cell, pbc = extract_arrays(atoms)
    viewer(P, numbers, cell, pbc)
