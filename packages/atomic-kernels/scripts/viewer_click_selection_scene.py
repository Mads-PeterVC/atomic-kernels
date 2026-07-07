from __future__ import annotations

import sys
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PYTHON_SRC = ROOT / "python"
TESTS_DIR = ROOT / "tests"
for path in (PYTHON_SRC, TESTS_DIR):
    if str(path) not in sys.path:
        sys.path.insert(0, str(path))

from viewer_integration_helpers import (  # noqa: E402
    XVFB_SCREEN_HEIGHT,
    XVFB_SCREEN_WIDTH,
    launch_ready_viewer_session,
    single_center_atom_scene,
)


def main() -> None:
    scene = single_center_atom_scene()
    print(f"Scene: {scene.description}")
    print(f"Screen assumption: {XVFB_SCREEN_WIDTH}x{XVFB_SCREEN_HEIGHT}")
    print(f"Expected click target: ({scene.click_target.x}, {scene.click_target.y})")
    print("Close the viewer window to exit.")

    session = launch_ready_viewer_session(scene, timeout=20.0)
    try:
        while True:
            print(f"Selected atoms: {session.selected_atoms()}")
            time.sleep(1.0)
    except KeyboardInterrupt:
        pass
    finally:
        session.close()


if __name__ == "__main__":
    main()
