# Viewer Examples

These examples collect small patterns you can copy into your own ASE workflows. Each
example focuses on one viewer feature.

## Start a live session

Use `viewer_session(...)` when you want a viewer that stays open for further updates.
When you run these examples as scripts, put the `viewer_session(...)` call inside
`if __name__ == "__main__":`.

```python
from atomic_kernels import viewer_session


def main() -> None:
    session = viewer_session(atoms)
    session.wait_until_ready(timeout=5.0)


if __name__ == "__main__":
    main()
```

## Ball-and-stick on a selected adsorbate

Use `select(...).ball_and_stick(...)` when only part of the structure should switch
away from the default space-filling view.

```python
import numpy as np
from ase.build import add_adsorbate, fcc111, molecule

from atomic_kernels import viewer_session


def main() -> None:
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


if __name__ == "__main__":
    main()
```

If you also need custom connectivity, add `render.set_bonds(...)` separately.

## Automatic bonds

Use `render.set_bonds(mode="default")` when the viewer should derive bonds from the
current frame using ASE-style natural cutoffs.

```python
from ase.build import add_adsorbate, fcc111, molecule

from atomic_kernels import viewer_session


def main() -> None:
    slab = fcc111("Cu", size=(4, 4, 3), vacuum=10.0)
    adsorbate = molecule("CH3OH")
    slab_atom_count = len(slab)

    add_adsorbate(slab, adsorbate, height=3.3, position="ontop", offset=(2, 2))
    slab.center(axis=2)

    session = viewer_session(slab)
    session.render().set_bonds(mode="default")


if __name__ == "__main__":
    main()
```

## Polyhedral faces

Use `render.set_faces(mode="default", ...)` when you want to derive best-effort
polyhedra from coordination environments instead of enumerating faces by hand.

```python
import numpy as np
from ase import Atoms

from atomic_kernels import viewer_session


def main() -> None:
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
        mode="default",
        selection=[0],
        face_colors=[
            (0.13, 0.52, 0.78, 0.34),
            (0.18, 0.65, 0.66, 0.30),
            (0.83, 0.56, 0.16, 0.28),
            (0.73, 0.31, 0.22, 0.30),
        ],
    )


if __name__ == "__main__":
    main()
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


def main() -> None:
    atoms = bulk("Cu", "fcc", a=3.615).repeat((4, 4, 4))
    atoms.center(vacuum=6.0)

    z = atoms.positions[:, 2]
    heights = ((z - z.min()) / (z.max() - z.min())).astype(np.float32)

    session = viewer_session(atoms)
    colors = session.colors()
    colors.set_atom_scalars("height", heights)
    colors.by_scalar("height", palette="inferno")


if __name__ == "__main__":
    main()
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


def main() -> None:
    atoms = molecule("H2O")
    atoms.cell = (8.0, 8.0, 8.0)
    atoms.center()

    session = viewer_session(atoms)
    session.follow_tail(True)

    for step in range(10):
        frame = atoms.copy()
        frame.positions[:, 2] += 0.05 * step * np.sin(np.linspace(0.0, np.pi, len(frame)))
        session.append_frame(frame)


if __name__ == "__main__":
    main()
```

## Camera controls

Use the camera controller when the script needs a specific view or a scripted camera
motion.

```python
from ase.build import fcc111

from atomic_kernels import viewer_session


def main() -> None:
    atoms = fcc111("Cu", size=(4, 4, 3), vacuum=8.0)
    atoms.center(axis=2)

    session = viewer_session(atoms)
    camera = session.camera()

    camera.frame_all()
    camera.set_rotation(yaw=-1.1, pitch=0.45)
    camera.pan((2.0, 0.0, 0.5))
    camera.zoom(factor=0.75)
    camera.look_at((0.0, 0.0, 0.0), radius=18.0, yaw=0.4, pitch=0.2)


if __name__ == "__main__":
    main()
```
