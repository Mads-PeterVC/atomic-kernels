from __future__ import annotations

import numpy as np
from ase import Atoms
from ase.cluster import *

from atomic_kernels import viewer_session, ViewerConfig, RenderConfig


def large_cluster() -> Atoms:
    atoms = Icosahedron("Ti", 7, latticeconstant=4.0)

    # Make a random alloy with different metals
    possible_atomic_numbers = [77, 13, 47, 79, 29, 46, 78, 26]  # Ir, Al, Ag, Au, Cu, Pd, Pt, Fe
    new_numbers = np.random.choice(possible_atomic_numbers, size=len(atoms), replace=True)
    atoms.set_atomic_numbers(new_numbers)

    atoms.set_cell(np.eye(3) * 30)
    atoms.center()


    return atoms


if __name__ == "__main__":
    atoms = large_cluster()

    render_config = RenderConfig(show_cell=False, show_axes=False, show_ui=False)
    config = ViewerConfig(render=render_config, window_width=800, window_height=500)

    session = viewer_session(atoms, config=config)
    render = session.render()
    camera = session.camera()

