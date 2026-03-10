# Viewer Examples

These examples distill the usage-oriented scripts in
[`scripts/`](/Users/au616397/Repositories/atomic-kernels/scripts) into smaller patterns
you can copy into your own ASE workflows.

The examples below are adapted from:

- [`py_slab_adsorbate_ball_and_stick.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_slab_adsorbate_ball_and_stick.py)
- [`py_polyhedra_faces.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_polyhedra_faces.py)

## Launch a reusable viewer session

Use `viewer_session(...)` when you want a live session that can be updated after the
window opens. The returned object keeps camera, color, and render controls together.

```python
from atomic_kernels import viewer_session

session = viewer_session(atoms)
session.wait_until_ready(timeout=5.0)

camera = session.camera()
render = session.render()
colors = session.colors()

camera.frame_all()
```

This is the base pattern used by the more specific examples below.

## Highlight an adsorbate with explicit ball-and-stick bonds

This pattern is useful when you want a slab to stay in space-filling mode while the
adsorbate is rendered in a smaller ball-and-stick style with manually controlled bond
connectivity.

```python
from __future__ import annotations

import numpy as np
from ase.build import add_adsorbate, fcc111, molecule
from ase.neighborlist import natural_cutoffs, neighbor_list

from atomic_kernels import viewer_session


def adsorbate_bonds(atoms) -> np.ndarray:
    cutoffs = natural_cutoffs(atoms, mult=1.2)
    senders, receivers = neighbor_list("ij", atoms, cutoffs)
    return np.column_stack([senders, receivers])


def adsorption_bond(structure, slab_atom_count: int) -> np.ndarray:
    adsorbate_indices = np.arange(slab_atom_count, len(structure))
    oxygen_indices = adsorbate_indices[structure.numbers[slab_atom_count:] == 8]
    if len(oxygen_indices) == 0:
        return np.empty((0, 2), dtype=np.int64)

    oxygen_index = int(oxygen_indices[0])
    oxygen_position = structure.positions[oxygen_index]
    nearest_slab_index = int(
        np.argmin(
            np.linalg.norm(structure.positions[:slab_atom_count] - oxygen_position, axis=1)
        )
    )
    return np.array([[nearest_slab_index, oxygen_index]], dtype=np.int64)


slab = fcc111("Cu", size=(4, 4, 3), vacuum=10.0)
adsorbate = molecule("CH3OH")
adsorbate.rotate(90.0, "x")
adsorbate.rotate(25.0, "z")

slab_atom_count = len(slab)
adsorbate_internal_bonds = adsorbate_bonds(adsorbate) + slab_atom_count

add_adsorbate(slab, adsorbate, height=3.3, position="ontop", offset=(2, 2))
slab.center(axis=2)

adsorbate_mask = np.zeros(len(slab), dtype=bool)
adsorbate_mask[slab_atom_count:] = True

all_adsorbate_bonds = np.vstack(
    [adsorbate_internal_bonds, adsorption_bond(slab, slab_atom_count)]
)

session = viewer_session(slab)
render = session.render()
camera = session.camera()

render.set_bonds(all_adsorbate_bonds)
session.select(adsorbate_mask).ball_and_stick(
    atom_scale=0.6,
    bond_radius=0.07,
    bond_color=(0.55, 0.55, 0.55),
    bond_scope="touch_selection",
)

camera.frame_all()
camera.set_rotation(yaw=-1.57, pitch=0.0)
```

Why this pattern is reusable:

- `render.set_bonds(...)` lets you define exactly which edges should be shown.
- `session.select(mask).ball_and_stick(...)` scopes the style to the adsorbate only.
- `bond_scope="touch_selection"` keeps slab-adsorbate bridge bonds visible even though
  only the adsorbate atoms are selected.

## Overlay polyhedral faces on explicit bonds

Use `render.set_faces(...)` when you already know the atom indices that define a
polyhedron and want a semi-transparent surface on top of the atom-and-bond view.

```python
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


atoms = tetrahedral_cluster()
session = viewer_session(atoms)
render = session.render()
camera = session.camera()

render.set_bonds([(0, 1), (0, 2), (0, 3), (0, 4)])
session.select([0, 1, 2, 3, 4]).ball_and_stick(
    atom_scale=0.4,
    bond_radius=0.08,
    bond_color=(0.55, 0.55, 0.58),
)

render.set_faces(
    [
        [1, 2, 3],
        [1, 4, 2],
        [1, 3, 4],
        [2, 4, 3],
    ],
    face_colors=[
        (0.13, 0.52, 0.78, 0.34),
        (0.18, 0.65, 0.66, 0.30),
        (0.83, 0.56, 0.16, 0.28),
        (0.73, 0.31, 0.22, 0.30),
    ],
)

camera.frame_all()
camera.set_rotation(yaw=-0.75, pitch=0.45)
```

Notes:

- Each face is a list of atom indices for one polygon.
- `face_colors` must match the number of faces.
- Faces and explicit bonds are independent overlays, so you can combine them freely.

## Practical guidance

- Prefer explicit helper functions for bond or face generation when the structure logic
  matters. That keeps the visualization call site short.
- Use boolean masks for long-lived selections such as “adsorbate atoms” or “top layer”.
- Keep camera setup at the end of the script so the structure styling is complete before
  framing and orbiting.
- Use `session.wait_until_ready(timeout=...)` before automated camera changes when the
  script is part of a test or a larger workflow.
