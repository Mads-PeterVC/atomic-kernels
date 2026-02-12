from ase.io import write
from ase.build import molecule

atoms = molecule('H2O')
# atoms.cell = [10, 10, 10]
atoms.center()
write('h2o.xyz', atoms)