from __future__ import annotations

import numpy as np
from ase.build import bulk

from atomic_kernels import viewer_session


def build_structure():
    atoms = bulk("Cu", "fcc", a=3.615).repeat((4, 4, 4))
    atoms.center(vacuum=6.0)
    return atoms


def radial_metallic_weights(atoms) -> np.ndarray:
    positions = atoms.get_positions()
    center = positions.mean(axis=0, keepdims=True)
    distances = np.linalg.norm(positions - center, axis=1)
    span = float(distances.max() - distances.min())
    if span <= 0.0:
        return np.zeros(len(atoms), dtype=np.float32)
    return ((distances - distances.min()) / span).astype(np.float32)


if __name__ == "__main__":
    atoms = build_structure()
    session = viewer_session(atoms)
    camera = session.camera()
    materials = session.materials()

    camera.frame_all()

    metallic = radial_metallic_weights(atoms)
    materials.set_atom_scalars("radial_metallic", metallic)
    materials.by_scalar(
        "radial_metallic",
        channel="metallic",
        min=0.0,
        max=1.0,
    )
