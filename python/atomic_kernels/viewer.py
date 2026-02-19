from ase import Atoms

from atomic_kernels import trajectory_viewer

def bevy_viewer(atoms: Atoms | list[Atoms]) -> None:

    if isinstance(atoms, Atoms):
        atoms = [atoms]

    trajectory_viewer(atoms)
