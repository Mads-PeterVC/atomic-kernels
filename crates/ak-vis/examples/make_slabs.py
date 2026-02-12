from pathlib import Path

from ase.build import (
    bulk,
    fcc111,
    fcc100,
    bcc110,
    hcp0001,
    diamond111,
    graphene,
    surface,
)
from ase.io import write


out_dir = Path("xyz")
out_dir.mkdir(parents=True, exist_ok=True)

slabs = []

# FCC metals
slabs.append(("cu_fcc111_3x3x4", fcc111("Cu", size=(3, 3, 4), vacuum=10.0)))
slabs.append(("al_fcc100_3x3x4", fcc100("Al", size=(3, 3, 4), vacuum=10.0)))

# BCC metal
slabs.append(("fe_bcc110_3x3x4", bcc110("Fe", size=(3, 3, 4), vacuum=10.0)))

# HCP metal
slabs.append(("ti_hcp0001_3x3x4", hcp0001("Ti", size=(3, 3, 4), vacuum=10.0)))

# Diamond / covalent
slabs.append(("si_diamond111_3x3x4", diamond111("Si", size=(3, 3, 4), vacuum=10.0)))


# Graphene
slabs.append(("graphene_4x4", graphene(size=(4, 4, 1), vacuum=10.0)))

# Generic surface from bulk (e.g., W(211))
w_bulk = bulk("W", "bcc", a=3.165)
slabs.append(("w_bcc211_3x3x4", surface(w_bulk, (2, 1, 1), layers=4, vacuum=10.0)))

for name, atoms in slabs:
    write(out_dir / f"{name}.xyz", atoms)

print(f"Wrote {len(slabs)} slabs to {out_dir}/")