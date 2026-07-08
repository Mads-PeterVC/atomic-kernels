from __future__ import annotations

import time

import numpy as np
from ase import Atoms
from ase.build import molecule

from ak_viewer import viewer_session


def displaced_frames(base: Atoms, count: int) -> list[Atoms]:
    frames = []
    for idx in range(count):
        frame = base.copy()
        positions = frame.get_positions()
        positions[:, 2] += 0.05 * idx * np.sin(np.linspace(0.0, np.pi, len(frame)))
        frame.set_positions(positions)
        frames.append(frame)
    return frames


if __name__ == "__main__":
    initial = molecule("H2O")
    initial.cell = (8.0, 8.0, 8.0)
    initial.center()

    session = viewer_session(initial)
    session.follow_tail(True)

    for frame in displaced_frames(initial, 25):
        session.append_frame(frame)
        time.sleep(1.0)
