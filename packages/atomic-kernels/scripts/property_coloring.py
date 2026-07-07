from __future__ import annotations

import time

import matplotlib.pyplot as plt
import numpy as np
from ase.build import bulk

from atomic_kernels import viewer_session


def build_structure():
    atoms = bulk("Cu", "fcc", a=3.615).repeat((4, 4, 4))
    atoms.center(vacuum=6.0)
    return atoms


def normalized_heights(atoms) -> np.ndarray:
    z = atoms.get_positions()[:, 2]
    z_min = float(z.min())
    z_span = float(z.max() - z_min)
    if z_span == 0.0:
        return np.zeros(len(atoms), dtype=np.float32)
    return ((z - z_min) / z_span).astype(np.float32)


if __name__ == "__main__":
    atoms = build_structure()
    session = viewer_session(atoms)
    materials = session.materials()
    camera = session.camera()
    camera.frame_all()

    heights = normalized_heights(atoms)
    materials.set_atom_scalars("height", heights)
    materials.by_scalar("height", channel="color", palette="inferno")

    time.sleep(2.0)

    # Switch to a matplotlib colormap by sampling it into RGBA values.
    magma = plt.get_cmap("magma")(np.linspace(0.0, 1.0, 256)).astype(np.float32)
    materials.by_scalar("height", channel="color", colors=magma)

    time.sleep(2.0)

    shifted = atoms.copy()
    shifted.positions[:, 2] += np.sin(np.linspace(0.0, np.pi, len(shifted)))
    session.append_frame(shifted)

    shifted_heights = normalized_heights(shifted)
    materials.set_atom_scalars("height", shifted_heights, frame_index=1)
    materials.by_scalar("height", channel="color", palette="plasma")
    session.set_frame(1)
