from ase import Atoms
from typing import Optional

from atomic_kernels._atomic_kernels import trajectory_viewer, ViewerConfig

def bevy_viewer(atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None) -> None:

    if isinstance(atoms, Atoms):
        atoms = [atoms]

    trajectory_viewer(atoms, config)
