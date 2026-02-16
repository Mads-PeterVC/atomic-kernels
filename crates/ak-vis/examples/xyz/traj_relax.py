import numpy as np
from ase.calculators.lj import LennardJones
from ase.optimize import BFGS
from ase import Atoms
from ase.io import write, read

def setup_atoms(n: int, cell_size: float) -> Atoms:
    positions = np.random.rand(n, 3) * (cell_size - 5)  # Random positions in a cell_size x cell_size x cell_size box
    atoms = Atoms('H' * n, positions=positions)
    atoms.cell = np.eye(3) * cell_size
    atoms.center()
    atoms.set_calculator(LennardJones())
    return atoms

def relax_trajectory(atoms: Atoms, traj_file: str):
    dyn = BFGS(atoms, trajectory=traj_file)
    dyn.run(fmax=0.01)  # Relax until forces are below 0.01 eV/Å

if __name__ == "__main__":
    n_atoms = 20
    cell_size = 10.0
    traj_file = 'relaxation.traj'

    atoms = setup_atoms(n_atoms, cell_size)
    relax_trajectory(atoms, traj_file)

    # Load the trajectory and resave as xyz for each frame
    traj = read(traj_file, index=':')
    print(len(traj))
    for i, frame in enumerate(traj):
        frame.write(f'relaxation/frame_{i:03d}.xyz')
