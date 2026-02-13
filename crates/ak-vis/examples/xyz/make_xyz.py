from ase.io import read, write

atoms = read('/Users/au616397/Repositories/quail/paper/figures/optimization_frames/ase.traj', '-1')

atoms = atoms.repeat((10, 10, 1))

write('optimized_structure.xyz', atoms)