import numpy as np
from ase import Atoms

def extract_arrays(atoms: Atoms, force_contiguous: bool = True):
    """
    Extracts the positions, atomic numbers, and cell information from an ASE Atoms object.

    Parameters:
    atoms (Atoms): An ASE Atoms object containing the atomic structure.

    Returns:
    tuple: A tuple containing four numpy arrays:
        - positions (numpy.ndarray): An array of shape (N, 3) with the atomic positions.
        - atomic_numbers (numpy.ndarray): An array of shape (N,) with the atomic numbers.
        - cell (numpy.ndarray): An array of shape (3, 3) representing the cell vectors.
        - pbc (numpy.ndarray): An array of shape (3,) representing the periodic boundary conditions.
    """
    positions = atoms.get_positions()
    atomic_numbers = atoms.get_atomic_numbers()
    cell = atoms.get_cell()
    pbc = atoms.get_pbc()

    if force_contiguous:
        positions = np.ascontiguousarray(positions)
        atomic_numbers = np.ascontiguousarray(atomic_numbers, dtype=np.int32)
        cell = np.ascontiguousarray(cell)
        pbc = np.ascontiguousarray(pbc)

    return positions, atomic_numbers, cell, pbc
