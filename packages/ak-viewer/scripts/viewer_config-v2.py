from ak_viewer.viewer import bevy_viewer
from ak_viewer import ViewerConfig, RenderConfig
import numpy as np

from ase.build import molecule, bulk
from ase import Atoms
from ase.collections import g2


def build() -> Atoms:
    base = bulk("Cu", "fcc", a=3.615).repeat((21, 21, 21))
    base.center(vacuum=0.0)

    base_positions = base.get_positions()
    base_center = base_positions.mean(axis=0)
    mins = base_positions.min(axis=0)
    maxs = base_positions.max(axis=0)
    radius = 0.9 * ((maxs - mins) / 2.0).min()
    mask = np.linalg.norm(base_positions - base_center, axis=1) <= radius
    nanoparticle = base[mask]

    np_positions = nanoparticle.get_positions()
    distances = np.linalg.norm(np_positions - base_center, axis=1)
    r_core = radius * 0.6
    r_mid = radius * 0.85
    symbols = np.where(distances < r_core, "Cu", np.where(distances < r_mid, "Ag", "Au"))
    nanoparticle.set_chemical_symbols(symbols.tolist())

    nanoparticle.center(vacuum=8.0)
    np_positions = nanoparticle.get_positions()
    mins = np_positions.min(axis=0)
    maxs = np_positions.max(axis=0)
    center = (mins + maxs) / 2.0

    molecule_names = list(g2.names)
    grid_n = 4
    offset = 3.0
    molecules = Atoms()
    mol_cache = {}

    faces = ((0, 1), (0, -1), (1, 1), (1, -1), (2, 1), (2, -1))
    for face_idx, (face_axis, sign) in enumerate(faces):
        normal = np.zeros(3)
        normal[face_axis] = float(sign)

        face_pos = center.copy()
        face_pos[face_axis] = maxs[face_axis] if sign > 0 else mins[face_axis]
        face_pos[face_axis] += sign * offset

        other_axes = [0, 1, 2]
        other_axes.remove(face_axis)
        span = (maxs[other_axes] - mins[other_axes]).min()
        step = span / (grid_n + 1)

        for i in range(grid_n):
            for j in range(grid_n):
                shift = np.zeros(3)
                shift[other_axes[0]] = (i - (grid_n - 1) / 2) * step
                shift[other_axes[1]] = (j - (grid_n - 1) / 2) * step

                name_idx = (face_idx * grid_n * grid_n + i * grid_n + j) % len(molecule_names)
                mol_name = molecule_names[name_idx]
                if mol_name not in mol_cache:
                    mol_cache[mol_name] = molecule(mol_name)
                placed = mol_cache[mol_name].copy()
                placed.rotate([0, 0, 1], normal, center=(0, 0, 0))
                placed.translate(face_pos + shift)
                molecules += placed

    atoms = nanoparticle + molecules
    atoms.center(vacuum=10.0)

    return atoms


atoms = build()

trajectory = [atoms]
for mol_name in g2.names:
    mol = molecule(mol_name)
    # mol.center(vacuum=5.0)
    mol.cell = (10.0, 10.0, 10.0)
    mol.center()
    trajectory.append(mol)


config = ViewerConfig(
    render=RenderConfig(ico_subdiv=4),
)

bevy_viewer(trajectory, config=config)
