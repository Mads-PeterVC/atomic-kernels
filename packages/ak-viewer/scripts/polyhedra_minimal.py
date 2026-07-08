from __future__ import annotations

import numpy as np
from ase import Atoms

from ak_viewer import viewer_session

if __name__ == "__main__":


    atoms = Atoms(
        symbols=["Ti", "O", "O", "O", "O"],
        positions=np.array(
            [
                [0.0, 0.0, 0.0],
                [1.0, 1.0, 1.0],
                [1.0, -1.0, -1.0],
                [-1.0, 1.0, -1.0],
                [-1.0, -1.0, 1.0],
            ]
        ),
        cell=[12.0, 12.0, 12.0],
        pbc=False,
    )
    atoms.center()

    session = viewer_session(atoms)
    render = session.render()
    render.set_bonds(mode="default")
    render.ball_and_stick()
    render.set_faces(mode="default")
