from __future__ import annotations

import time

import numpy as np
from ase.build import fcc111

from atomic_kernels import viewer_session


if __name__ == "__main__":
    atoms = fcc111("Cu", size=(4, 4, 3), vacuum=8.0)
    positions = atoms.get_positions()
    z = positions[:, 2]
    top_layer = np.isclose(z, z.max())
    positions[top_layer, 2] += np.linspace(0.1, 0.8, top_layer.sum())
    atoms.set_positions(positions)
    atoms.center(axis=2)

    session = viewer_session(atoms)
    camera = session.camera()
    materials = session.materials()

    z = atoms.get_positions()[:, 2]
    z_min = float(z.min())
    z_span = float(z.max() - z_min)
    heights = (z - z_min) / z_span if z_span > 0.0 else z * 0.0
    materials.set_atom_scalars("height", heights)
    materials.by_scalar("height", channel="color", palette="viridis")

    camera.frame_all()
    camera.set_rotation(yaw=-1.1, pitch=0.45)
    time.sleep(1.5)

    camera.start_orbit(yaw_rate=1.2, pitch_rate=0.15)
    time.sleep(4.0)

    camera.stop()
    camera.pan((2.0, 0.0, 0.5))
    camera.zoom(factor=0.75)
    time.sleep(1.5)

    camera.look_at((0.0, 0.0, 0.0), radius=18.0, yaw=0.4, pitch=0.2)
