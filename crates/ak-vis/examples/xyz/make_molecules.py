from ase.io import write
from ase.build import molecule
from ase.collections import g2


best_name = max(g2.names, key=lambda name: len(molecule(name)))
print(f"The largest molecule in the G2 set is: {best_name}")

for name in g2.names:
    atoms = molecule(name)
    atoms.cell = [10, 10, 10]
    atoms.center()
    write(f"{name}.xyz", atoms)