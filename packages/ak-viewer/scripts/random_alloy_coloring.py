from __future__ import annotations

import numpy as np
from ase.build import bulk
from ase.calculators.emt import EMT
from ase.neighborlist import neighbor_list

from ak_viewer import viewer_session


def build_random_alloy():
    rng = np.random.default_rng(11)

    # Start from a relatively large FCC supercell and randomly assign Cu/Ni to
    # the lattice sites so the structure stays physically reasonable for EMT.
    atoms = bulk("Cu", "fcc", a=3.62, cubic=True).repeat((5, 5, 4))
    copper_mask = rng.random(len(atoms)) < 0.6
    symbols = np.where(copper_mask, "Cu", "Ni")
    atoms.set_chemical_symbols(symbols.tolist())

    # Add a small positional disorder so the local environments are not
    # perfectly symmetric from the outset.
    atoms.rattle(stdev=0.05, seed=11)
    atoms.calc = EMT()
    return atoms, copper_mask


def atomic_energies(atoms) -> np.ndarray:
    return np.asarray(atoms.get_potential_energies(), dtype=np.float32)


if __name__ == "__main__":
    atoms, copper_mask = build_random_alloy()
    nickel_mask = ~copper_mask
    session = viewer_session(atoms)
    camera = session.camera()
    render = session.render()
    camera.frame_all()

    energies = atomic_energies(atoms)
    copper_energies = energies[copper_mask]
    nickel_energies = energies[nickel_mask]
    senders, receivers = neighbor_list("ij", atoms, 2.8)
    render.set_bonds(np.column_stack([senders, receivers]))
    session.select(copper_mask).ball_and_stick(atom_scale=0.4, bond_radius=0.06)

    # Layer two independent coloring rules onto the same frame: Cu atoms use
    # inferno while Ni atoms use plasma.
    session.select(copper_mask).material_by_scalar(
        "copper_atomic_energy",
        energies,
        channel="color",
        palette="inferno",
        min=float(copper_energies.min()),
        max=float(copper_energies.max()),
    )
    session.select(nickel_mask).material_by_scalar(
        "nickel_atomic_energy",
        energies,
        channel="color",
        palette="plasma",
        min=float(nickel_energies.min()),
        max=float(nickel_energies.max()),
    )
