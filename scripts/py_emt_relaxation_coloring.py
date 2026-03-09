from __future__ import annotations

import numpy as np
from ase.build import fcc111
from ase.calculators.emt import EMT
from ase.constraints import FixAtoms
from ase.optimize import BFGS
from time import sleep

from atomic_kernels import viewer_session


def build_deformed_slab():
    slab = fcc111("Cu", size=(4, 4, 3), vacuum=8.0)
    slab.center(axis=2)
    rng = np.random.default_rng(7)

    positions = slab.get_positions()
    z = positions[:, 2]
    z_levels = np.unique(np.round(z, decimals=6))
    bottom_z = z_levels.min()
    top_z = z_levels.max()

    # Keep the bottom layer fixed so the relaxation looks like a slab optimization.
    slab.set_constraint(FixAtoms(mask=np.isclose(z, bottom_z)))

    # Deform only the top layer by pushing each atom upward by a random amount.
    top_mask = np.isclose(z, top_z)
    positions[top_mask, 2] += rng.uniform(0.15, 0.9, size=top_mask.sum())
    slab.set_positions(positions)

    slab.calc = EMT()
    return slab, top_mask


def atomic_energies(atoms) -> np.ndarray:
    return np.asarray(atoms.get_potential_energies(), dtype=np.float32)


if __name__ == "__main__":
    slab, top_layer = build_deformed_slab()
    session = viewer_session(slab)
    session.follow_tail(True)
    energy_scale = session.scalar_range_tracker()

    initial_energies = atomic_energies(slab)
    energy_min, energy_max = energy_scale.update(initial_energies[top_layer])
    session.select(top_layer, frame_index=0).color_by_scalar(
        "atomic_energy",
        initial_energies,
        palette="inferno",
        min=energy_min,
        max=energy_max,
        frame_index=0,
    )

    optimizer = BFGS(slab, logfile="-")
    next_frame_index = [1]

    def push_frame() -> None:
        frame = slab.copy()
        frame_energies = atomic_energies(slab)
        session.append_frame(frame)
        energy_min, energy_max = energy_scale.update(frame_energies[top_layer])
        session.select(top_layer, frame_index=next_frame_index[0]).color_by_scalar(
            "atomic_energy",
            frame_energies,
            palette="viridis",
            min=energy_min,
            max=energy_max,
            frame_index=next_frame_index[0],
        )
        next_frame_index[0] += 1
        sleep(0.5)

    optimizer.attach(push_frame, interval=1)
    optimizer.run(fmax=0.03, steps=25)
