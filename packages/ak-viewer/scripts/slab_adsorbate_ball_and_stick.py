from __future__ import annotations

import numpy as np
from ase.build import add_adsorbate, fcc111, molecule
from ase.neighborlist import natural_cutoffs, neighbor_list
from time import sleep

from ak_viewer import viewer_session, ViewerConfig


def adsorbate_bonds(atoms) -> np.ndarray:
    cutoffs = natural_cutoffs(atoms, mult=1.2)
    senders, receivers = neighbor_list("ij", atoms, cutoffs)
    return np.column_stack([senders, receivers])


def adsorption_bonds(structure, slab_atom_count: int) -> np.ndarray:
    adsorbate_positions = structure.positions[slab_atom_count:]
    slab_positions = structure.positions[:slab_atom_count]
    oxygen_indices = np.where(structure.numbers[slab_atom_count:] == 8)[0]
    if len(oxygen_indices) == 0:
        return np.empty((0, 2), dtype=np.int64)

    oxygen_index = slab_atom_count + int(oxygen_indices[0])
    oxygen_position = structure.positions[oxygen_index]
    nearest_slab_index = int(
        np.argmin(np.linalg.norm(slab_positions - oxygen_position, axis=1))
    )
    return np.array([[nearest_slab_index, oxygen_index]], dtype=np.int64)


if __name__ == "__main__":

    slab = fcc111("Cu", size=(4, 4, 3), vacuum=10.0)
    adsorbate = molecule("CH3OH")
    adsorbate.rotate(90.0, "x")
    adsorbate.rotate(25.0, "z")

    slab_atom_count = len(slab)
    molecule_bonds = adsorbate_bonds(adsorbate)

    add_adsorbate(slab, adsorbate, height=3.3, position="ontop", offset=(2, 2))
    slab.center(axis=2)

    molecule_mask = np.zeros(len(slab), dtype=bool)
    molecule_mask[slab_atom_count:] = True

    shifted_bonds = molecule_bonds + slab_atom_count
    shifted_bonds = np.vstack([shifted_bonds, adsorption_bonds(slab, slab_atom_count)])


    config = ViewerConfig(window_width=800, window_height=600)

    session = viewer_session(slab, config=config)
    camera = session.camera()
    render = session.render()

    render.set_bonds(shifted_bonds)
    session.select(molecule_mask).ball_and_stick(
        atom_scale=0.6,
        bond_radius=0.07,
        bond_color=(0.55, 0.55, 0.55),
        bond_scope="touch_selection",
    )

    camera.frame_all()
    # camera.set_rotation(yaw=-1.57, pitch=0.0)
    # sleep(1.5)
    # camera.start_orbit(yaw_rate=0.6)
    # sleep(30.0)
    # camera.stop()
