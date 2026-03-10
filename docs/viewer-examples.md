# Viewer Examples

These examples collect small patterns you can copy into your own ASE workflows. Each
example focuses on one viewer feature.

## Start a live session

Use `viewer_session(...)` when you want a viewer that stays open for further updates.

```python
from atomic_kernels import viewer_session

session = viewer_session(atoms)
session.wait_until_ready(timeout=5.0)
```

## Ball-and-stick on a selected adsorbate

Use `select(...).ball_and_stick(...)` when only part of the structure should switch
away from the default space-filling view.

```python
import numpy as np
from ase.build import add_adsorbate, fcc111, molecule

from atomic_kernels import viewer_session

slab = fcc111("Cu", size=(4, 4, 3), vacuum=10.0)
adsorbate = molecule("CH3OH")
slab_atom_count = len(slab)

add_adsorbate(slab, adsorbate, height=3.3, position="ontop", offset=(2, 2))
slab.center(axis=2)

adsorbate_mask = np.zeros(len(slab), dtype=bool)
adsorbate_mask[slab_atom_count:] = True

session = viewer_session(slab)
session.select(adsorbate_mask).ball_and_stick(
    atom_scale=0.6,
    bond_radius=0.07,
    bond_color=(0.55, 0.55, 0.55),
)
```

If you also need custom connectivity, add `render.set_bonds(...)` separately.

## Explicit bonds

Use `render.set_bonds(...)` when the viewer should draw only the bond pairs you supply.

```python
import numpy as np
from ase.build import add_adsorbate, fcc111, molecule
from ase.neighborlist import natural_cutoffs, neighbor_list

from atomic_kernels import viewer_session


def adsorbate_bonds(atoms) -> np.ndarray:
    cutoffs = natural_cutoffs(atoms, mult=1.2)
    senders, receivers = neighbor_list("ij", atoms, cutoffs)
    return np.column_stack([senders, receivers])


slab = fcc111("Cu", size=(4, 4, 3), vacuum=10.0)
adsorbate = molecule("CH3OH")
slab_atom_count = len(slab)

add_adsorbate(slab, adsorbate, height=3.3, position="ontop", offset=(2, 2))
slab.center(axis=2)

session = viewer_session(slab)
session.render().set_bonds(adsorbate_bonds(adsorbate) + slab_atom_count)
```

## Polyhedral faces

Use `render.set_faces(...)` when you want to overlay polygon faces defined by atom
indices.

```python
import numpy as np
from ase import Atoms

from atomic_kernels import viewer_session

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
session.render().set_faces(
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
```

Notes:

- Each face is a list of atom indices for one polygon.
- `face_colors` must match the number of faces.
- Faces are independent of bonds and ball-and-stick styling.

## Scalar coloring

Use `colors.set_atom_scalars(...)` and `colors.by_scalar(...)` to color atoms from a
per-atom property array.

```python
import numpy as np
from ase.build import bulk

from atomic_kernels import viewer_session

atoms = bulk("Cu", "fcc", a=3.615).repeat((4, 4, 4))
atoms.center(vacuum=6.0)

z = atoms.positions[:, 2]
heights = ((z - z.min()) / (z.max() - z.min())).astype(np.float32)

session = viewer_session(atoms)
colors = session.colors()
colors.set_atom_scalars("height", heights)
colors.by_scalar("height", palette="inferno")
```

This example isolates scalar coloring on a single frame. For subset-only coloring, use
`session.select(...).color_by_scalar(...)` instead.

## Append frames and follow the newest one

Use `append_frame(...)` and `follow_tail(True)` when your structure changes over time
and the viewer should keep following the latest frame.

```python
import numpy as np
from ase.build import molecule

from atomic_kernels import viewer_session

atoms = molecule("H2O")
atoms.cell = (8.0, 8.0, 8.0)
atoms.center()

session = viewer_session(atoms)
session.follow_tail(True)

for step in range(10):
    frame = atoms.copy()
    frame.positions[:, 2] += 0.05 * step * np.sin(np.linspace(0.0, np.pi, len(frame)))
    session.append_frame(frame)
```

## Camera controls

Use the camera controller when the script needs a specific view or a scripted camera
motion.

```python
from ase.build import fcc111

from atomic_kernels import viewer_session

atoms = fcc111("Cu", size=(4, 4, 3), vacuum=8.0)
atoms.center(axis=2)

session = viewer_session(atoms)
camera = session.camera()

camera.frame_all()
camera.set_rotation(yaw=-1.1, pitch=0.45)
camera.pan((2.0, 0.0, 0.5))
camera.zoom(factor=0.75)
camera.look_at((0.0, 0.0, 0.0), radius=18.0, yaw=0.4, pitch=0.2)
```
