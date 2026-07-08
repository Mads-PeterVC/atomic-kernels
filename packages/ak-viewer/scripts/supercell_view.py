from __future__ import annotations

import time

from ase.build import bulk

from ak_viewer import RenderConfig, ViewerConfig, viewer_session


if __name__ == "__main__":
    atoms = bulk("Cu", "fcc", a=3.615).repeat((2, 2, 2))

    config = ViewerConfig(
        render=RenderConfig(
            show_ui=True,
            show_cell=True,
            show_axes=True,
            supercell_repeat_a=1,
            supercell_repeat_b=1,
            supercell_repeat_c=0,
            ghost_repeated_images=True,
        )
    )

    session = viewer_session(atoms, config=config)
    session.wait_until_ready(timeout=10.0)

    camera = session.camera()
    camera.frame_all()
    camera.set_rotation(yaw=0.7, pitch=0.45)

    print("Initial supercell:", session.supercell())
    time.sleep(1.0)

    session.increment_supercell_axis(2)
    print("After repeating c:", session.supercell())
    time.sleep(1.0)

    session.toggle_supercell_distinction()
    print("After toggling ghosting:", session.supercell())

