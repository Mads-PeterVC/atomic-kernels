from __future__ import annotations

import os

import pytest
from ase import Atoms


pytestmark = pytest.mark.viewer_integration


def _has_windowing_session() -> bool:
    if os.name != "posix":
        return True
    return bool(os.environ.get("DISPLAY") or os.environ.get("WAYLAND_DISPLAY"))


@pytest.mark.skipif(
    os.environ.get("ATOMIC_KERNELS_RUN_VIEWER_TESTS") != "1",
    reason="set ATOMIC_KERNELS_RUN_VIEWER_TESTS=1 to run real viewer smoke tests",
)
@pytest.mark.skipif(
    not _has_windowing_session(),
    reason="viewer smoke tests require a windowing session",
)
def test_viewer_session_reports_ready_and_accepts_commands():
    from atomic_kernels.viewer import RenderConfig, ViewerConfig, viewer_session

    atoms = Atoms("H2", positions=[(0.0, 0.0, 0.0), (0.0, 0.0, 0.74)])
    config = ViewerConfig(render=RenderConfig(show_ui=False))
    session = viewer_session(atoms, config=config)

    try:
        assert session.wait_until_ready(timeout=20.0)
        session.camera().frame_all()
        session.set_frame(0)
    finally:
        session.close()
