from __future__ import annotations

import os

import pytest
from ase import Atoms


pytestmark = pytest.mark.viewer_integration


def _has_windowing_session() -> bool:
    if os.name != "posix":
        return True
    return bool(os.environ.get("DISPLAY") or os.environ.get("WAYLAND_DISPLAY"))


def _viewer_ready_timeout() -> float:
    return 60.0 if os.environ.get("CI") == "true" else 20.0


def _viewer_start_attempts() -> int:
    return 3 if os.environ.get("CI") == "true" else 1


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
    timeout = _viewer_ready_timeout()

    for attempt in range(_viewer_start_attempts()):
        session = viewer_session(atoms, config=config)
        try:
            if session.wait_until_ready(timeout=timeout):
                session.camera().frame_all()
                session.set_frame(0)
                return
        finally:
            session.close()

    raise AssertionError(
        f"viewer session did not become ready within {timeout:.1f}s"
    )
