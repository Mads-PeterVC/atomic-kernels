from pathlib import Path

from ase import Atoms

from ak_viewer import (
    RenderConfig,
    ViewerConfig,
    headless_viewer_session,
)


def asymmetric_cluster() -> Atoms:
    atoms = Atoms(
        "CuONH3",
        positions=[
            (0.0, 0.0, 0.0),
            (1.8, 0.2, 0.1),
            (-0.9, 1.4, 0.6),
            (0.4, -1.7, 1.1),
            (-1.3, -0.4, -0.9),
            (0.7, 0.9, -1.4),
        ],
        cell=(10.0, 10.0, 10.0),
        pbc=False,
    )
    atoms.center()
    return atoms


def style(session) -> None:
    render = session.render()
    render.set_bonds([(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (2, 5)])
    render.ball_and_stick(
        atom_scale=0.42,
        bond_radius=0.07,
        bond_color=(0.55, 0.55, 0.55),
    )


def main() -> None:
    atoms = asymmetric_cluster()
    config = ViewerConfig(
        render=RenderConfig(
            show_ui=False,
            show_orientation_widget=False,
            show_axes=False,
            show_cell=False,
        )
    )

    default_path = Path("headless-default.png")
    moved_path = Path("headless-moved.png")

    default_session = headless_viewer_session(
        atoms,
        default_path.as_posix(),
        width=1200,
        height=900,
        config=config,
    )
    style(default_session)
    default_session.camera().frame_all()
    default_session.save()

    moved_session = headless_viewer_session(
        atoms,
        moved_path.as_posix(),
        width=1200,
        height=900,
        config=config,
    )
    style(moved_session)

    camera = moved_session.camera()

    # camera.frame_all()
    # camera.set_rotation(yaw=1.8, pitch=0.95)
    # camera.pan((0.7, -0.4, 0.3))
    # camera.zoom(factor=0.35)
    camera.look_at(
        focus=atoms.get_center_of_mass(),
        radius=7.0,
        yaw=1.8,
        pitch=0.95,
    )
    moved_session.save()

    print(f"saved {default_path.resolve()}")
    print(f"saved {moved_path.resolve()}")
    print("compare the two images to verify the camera path")


if __name__ == "__main__":
    main()
