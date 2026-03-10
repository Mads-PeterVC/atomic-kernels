from __future__ import annotations

import os

import pytest
from ase import Atoms

from atomic_kernels.viewer import headless_viewer_session


def _asymmetric_atoms() -> Atoms:
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


def test_headless_viewer_session_supports_direct_scripting(tmp_path):
    atoms = Atoms("H2", positions=[(0.0, 0.0, 0.0), (0.0, 0.0, 0.74)])

    session = headless_viewer_session(atoms, str(tmp_path / "direct.png"))
    session.camera().frame_all()
    session.render().ball_and_stick()
    session.save()


@pytest.mark.viewer_integration
@pytest.mark.skipif(
    os.environ.get("ATOMIC_KERNELS_RUN_VIEWER_TESTS") != "1",
    reason="set ATOMIC_KERNELS_RUN_VIEWER_TESTS=1 to run real headless render tests",
)
def test_render_image_writes_png_with_real_extension(tmp_path):
    atoms = Atoms(
        "H2O",
        positions=[(0.0, 0.0, 0.0), (0.76, 0.0, 0.58), (-0.76, 0.0, 0.58)],
    )
    path = tmp_path / "headless.png"

    session = headless_viewer_session(atoms, str(path), width=320, height=240)
    session.camera().frame_all()
    session.save()

    assert path.exists()
    assert path.stat().st_size > 0


@pytest.mark.viewer_integration
@pytest.mark.skipif(
    os.environ.get("ATOMIC_KERNELS_RUN_VIEWER_TESTS") != "1",
    reason="set ATOMIC_KERNELS_RUN_VIEWER_TESTS=1 to run real headless render tests",
)
def test_headless_camera_changes_affect_rendered_image(tmp_path):
    atoms = _asymmetric_atoms()
    default_path = tmp_path / "default.png"
    moved_path = tmp_path / "moved.png"

    default_session = headless_viewer_session(atoms, str(default_path), width=320, height=240)
    default_session.camera().frame_all()
    default_session.save()

    moved_session = headless_viewer_session(atoms, str(moved_path), width=320, height=240)
    moved_session.camera().frame_all()
    moved_session.camera().set_rotation(yaw=1.8, pitch=0.95)
    moved_session.camera().pan((0.7, -0.4, 0.3))
    moved_session.camera().zoom(factor=0.35)
    moved_session.save()

    assert default_path.exists()
    assert moved_path.exists()
    assert default_path.read_bytes() != moved_path.read_bytes()
