from pathlib import Path

from ase import Atoms
import anywidget
import numpy as np
import traitlets

DIRECTORY = Path(__file__).parent


def _normalize_atoms(atoms: Atoms | list[Atoms]) -> list[Atoms]:
    if isinstance(atoms, Atoms):
        frames = [atoms]
    else:
        frames = list(atoms)

    if not frames:
        raise ValueError("ViewerWidget requires at least one Atoms frame")

    for frame in frames:
        if not isinstance(frame, Atoms):
            raise TypeError("ViewerWidget frames must be ase.Atoms objects")

    return frames


def _serialize_atoms(atoms: Atoms) -> dict:
    positions = np.asarray(atoms.positions, dtype=np.float64)
    numbers = np.asarray(atoms.numbers, dtype=np.int32)
    cell = np.asarray(atoms.cell.array, dtype=np.float64)
    pbc = np.asarray(atoms.pbc, dtype=bool)

    if positions.ndim != 2 or positions.shape[1] != 3:
        raise ValueError("Atoms positions must have shape (N, 3)")
    if numbers.shape != (positions.shape[0],):
        raise ValueError("Atoms numbers must have shape (N,)")
    if cell.shape != (3, 3):
        raise ValueError("Atoms cell must have shape (3, 3)")
    if pbc.shape != (3,):
        raise ValueError("Atoms pbc must have shape (3,)")

    return {
        "positions": positions.reshape(-1).tolist(),
        "numbers": numbers.tolist(),
        "cell": cell.reshape(-1).tolist(),
        "pbc": pbc.tolist(),
    }


class ViewerWidget(anywidget.AnyWidget):
    _esm = DIRECTORY / "index.js"
    _css = """
    """
    frames = traitlets.List(trait=traitlets.Dict()).tag(sync=True)

    def __init__(self, atoms: Atoms | list[Atoms], **kwargs):
        kwargs["frames"] = [_serialize_atoms(frame) for frame in _normalize_atoms(atoms)]
        super().__init__(**kwargs)
