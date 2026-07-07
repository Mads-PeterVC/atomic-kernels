from __future__ import annotations

import numpy as np
from ase import Atoms

from atomic_kernels import viewer_session


def tetrahedral_cluster() -> Atoms:
    center = np.array([[0.0, 0.0, 0.0]])
    ligands = 1.85 * np.array(
        [
            [1.0, 1.0, 1.0],
            [1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
        ],
        dtype=float,
    ) / np.sqrt(3.0)

    atoms = Atoms(
        symbols=["Ti", "O", "O", "O", "O"],
        positions=np.vstack([center, ligands]),
        cell=[12.0, 12.0, 12.0],
        pbc=False,
    )
    atoms.center()
    return atoms


if __name__ == "__main__":
    atoms = tetrahedral_cluster()

    session = viewer_session(atoms)
    render = session.render()
    camera = session.camera()

    render.set_bonds(mode="default")
    session.select([0, 1, 2, 3, 4]).ball_and_stick(
        atom_scale=0.4,
        bond_radius=0.08,
        bond_color=(0.55, 0.55, 0.58),
    )

    render.set_faces(
        mode="default",
        selection=[0],
        face_colors=[
            (0.13, 0.52, 0.78, 0.34),
            (0.18, 0.65, 0.66, 0.30),
            (0.83, 0.56, 0.16, 0.28),
            (0.73, 0.31, 0.22, 0.30),
        ],
    )

    camera.frame_all()
    camera.set_rotation(yaw=-0.75, pitch=0.45)
