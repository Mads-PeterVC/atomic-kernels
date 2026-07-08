from __future__ import annotations

import numpy as np
from ase.build import molecule
from ase.neighborlist import natural_cutoffs, neighbor_list

from ak_viewer import viewer_session


def explicit_bonds(atoms):
    cutoffs = natural_cutoffs(atoms, 1.5)
    senders, receivers = neighbor_list("ij", atoms, cutoffs)
    return np.column_stack([senders, receivers])


if __name__ == "__main__":
    atoms = molecule("C6H6")
    atoms.center(vacuum=6.0)

    session = viewer_session(atoms)
    camera = session.camera()
    render = session.render()
    materials = session.materials()

    render.set_bonds(explicit_bonds(atoms))

    heavy_atoms = atoms.numbers >= 1
    render.ball_and_stick(
        selection=heavy_atoms,
        atom_scale=0.45,
        bond_radius=0.08,
        bond_color=(0.6, 0.6, 0.6),
    )

    z = atoms.positions[:, 2]
    z_norm = (z - z.min()) / max(float(z.max() - z.min()), 1e-6)
    # materials.set_atom_scalars("z", z_norm)
    # materials.by_scalar("z", channel="color", palette="plasma")

    camera.frame_all()
    camera.set_rotation(yaw=-0.9, pitch=0.35)
