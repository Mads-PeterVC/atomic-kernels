from ase.io import write
from ase.build import molecule
from ase.collections import g2

trajectory = []
for name in g2.names:
    atoms = molecule(name)
    atoms.set_cell([10, 10, 10])
    atoms.center()
    trajectory.append(atoms)

write('g2_trajectory.xyz', trajectory)