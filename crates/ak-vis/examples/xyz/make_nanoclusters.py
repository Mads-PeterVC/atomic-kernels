from __future__ import annotations

from pathlib import Path
import random
from typing import Iterable, Sequence

import numpy as np
from ase.cluster.icosahedron import Icosahedron
from ase.cluster.octahedron import Octahedron
from ase.io import write


out_dir = Path("xyz")
out_dir.mkdir(parents=True, exist_ok=True)


def _assign_symbols_by_index(atoms, symbols_by_index: dict[int, str]) -> None:
    symbols = atoms.get_chemical_symbols()
    for idx, sym in symbols_by_index.items():
        symbols[idx] = sym
    atoms.set_chemical_symbols(symbols)


def make_core_shell(
    atoms,
    core_element: str,
    shell_element: str,
    shell_fraction: float,
) -> None:
    center = atoms.get_center_of_mass()
    dists = np.linalg.norm(atoms.positions - center, axis=1)
    order = np.argsort(dists)
    shell_count = int(round(shell_fraction * len(atoms)))
    shell_indices = set(order[-shell_count:]) if shell_count > 0 else set()

    symbols_by_index = {
        i: (shell_element if i in shell_indices else core_element)
        for i in range(len(atoms))
    }
    _assign_symbols_by_index(atoms, symbols_by_index)


def make_random_alloy(
    atoms,
    elements: Sequence[str],
    fractions: Sequence[float],
    seed: int,
) -> None:
    if len(elements) != len(fractions):
        raise ValueError("elements and fractions must be the same length")

    total = sum(fractions)
    if total <= 0.0:
        raise ValueError("fractions must sum to a positive value")

    norm = [f / total for f in fractions]

    n = len(atoms)
    rng = random.Random(seed)
    indices = list(range(n))
    rng.shuffle(indices)

    counts = [int(round(f * n)) for f in norm]
    diff = n - sum(counts)
    counts[-1] += diff

    symbols_by_index: dict[int, str] = {}
    offset = 0
    for element, count in zip(elements, counts):
        for idx in indices[offset : offset + count]:
            symbols_by_index[idx] = element
        offset += count

    _assign_symbols_by_index(atoms, symbols_by_index)


clusters: list[tuple[str, Iterable]] = []

ico_2 = Icosahedron("Cu", 2)
make_core_shell(ico_2, core_element="Cu", shell_element="Ag", shell_fraction=0.4)
clusters.append(("cluster_cuag_ico_2_core_shell", ico_2))

ico_3 = Icosahedron("Pt", 3)
make_core_shell(ico_3, core_element="Pt", shell_element="Au", shell_fraction=0.35)
clusters.append(("cluster_ptau_ico_3_core_shell", ico_3))

ico_2_random = Icosahedron("Ni", 2)
make_random_alloy(ico_2_random, ["Ni", "Al", "Cu"], [0.5, 0.3, 0.2], seed=42)
clusters.append(("cluster_nialcu_ico_2_random", ico_2_random))

octa_3 = Octahedron("Cu", 3)
make_core_shell(octa_3, core_element="Cu", shell_element="Pt", shell_fraction=0.3)
clusters.append(("cluster_cupt_octa_3_core_shell", octa_3))

ico_4 = Icosahedron("Au", 4)
make_core_shell(ico_4, core_element="Au", shell_element="Ag", shell_fraction=0.25)
clusters.append(("cluster_auag_ico_4_core_shell", ico_4))

octa_4 = Octahedron("Ni", 4)
make_random_alloy(octa_4, ["Ni", "Cu", "Pd"], [0.5, 0.3, 0.2], seed=11)
clusters.append(("cluster_nicupd_octa_4_random", octa_4))

for name, atoms in clusters:
    write(out_dir / f"{name}.xyz", atoms)

print(f"Wrote {len(clusters)} nanoclusters to {out_dir}/")
