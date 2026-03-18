from __future__ import annotations

import os

import pytest
from ase import Atoms

from viewer_integration_helpers import (
    click_viewer_at,
    has_xdotool,
    launch_ready_viewer_session,
    single_center_atom_scene,
    ui_viewer_config,
    wait_for_selected_atoms,
)


pytestmark = pytest.mark.viewer_integration


def _has_windowing_session() -> bool:
    if os.name != "posix":
        return True
    return bool(os.environ.get("DISPLAY") or os.environ.get("WAYLAND_DISPLAY"))


def _viewer_ready_timeout() -> float:
    return 60.0 if os.environ.get("CI") == "true" else 20.0


def _viewer_start_attempts() -> int:
    return 3 if os.environ.get("CI") == "true" else 1


def test_ui_viewer_config_enables_ui():
    config = ui_viewer_config()

    assert config.render.show_ui is True


def test_single_center_atom_scene_declares_expected_click_target():
    scene = single_center_atom_scene()

    assert (scene.click_target.x, scene.click_target.y) == (0.5, 0.5)
    assert scene.click_target.normalized_to_window is True
    assert scene.expected_selection == [0]
    assert tuple(scene.atoms.cell.lengths()) == (12.0, 12.0, 12.0)
    assert tuple(scene.atoms.positions[0]) == scene.focus


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


@pytest.mark.skipif(
    os.environ.get("ATOMIC_KERNELS_RUN_VIEWER_TESTS") != "1",
    reason="set ATOMIC_KERNELS_RUN_VIEWER_TESTS=1 to run real viewer smoke tests",
)
@pytest.mark.skipif(
    not _has_windowing_session(),
    reason="viewer smoke tests require a windowing session",
)
def test_viewer_session_handles_live_trajectory_commands():
    from atomic_kernels.viewer import RenderConfig, ViewerConfig, viewer_session

    frames = [
        Atoms("H2", positions=[(0.0, 0.0, 0.0), (0.0, 0.0, 0.74)]),
        Atoms("H2", positions=[(0.1, 0.0, 0.0), (0.0, 0.0, 0.80)]),
    ]
    appended = Atoms("H2", positions=[(0.2, 0.0, 0.0), (0.0, 0.1, 0.88)])
    config = ViewerConfig(render=RenderConfig(show_ui=False))
    timeout = _viewer_ready_timeout()

    for attempt in range(_viewer_start_attempts()):
        session = viewer_session(frames, config=config)
        try:
            if not session.wait_until_ready(timeout=timeout):
                continue
            session.set_frame(1)
            session.follow_tail(True)
            session.append_frame(appended)
            session.set_frame(0)
            return
        finally:
            session.close()

    raise AssertionError(
        f"viewer trajectory session did not become ready within {timeout:.1f}s"
    )


@pytest.mark.skipif(
    os.environ.get("ATOMIC_KERNELS_RUN_VIEWER_TESTS") != "1",
    reason="set ATOMIC_KERNELS_RUN_VIEWER_TESTS=1 to run real viewer smoke tests",
)
@pytest.mark.skipif(
    not _has_windowing_session(),
    reason="viewer smoke tests require a windowing session",
)
@pytest.mark.skipif(
    not has_xdotool(),
    reason="viewer click smoke test requires xdotool",
)
def test_viewer_session_click_selection_works_with_ui_enabled():
    scene = single_center_atom_scene()
    timeout = _viewer_ready_timeout()

    for attempt in range(_viewer_start_attempts()):
        session = launch_ready_viewer_session(scene, timeout=timeout)
        try:
            assert session.selected_atoms() == []
            debug_info = click_viewer_at(session, scene.click_target)
            wait_for_selected_atoms(
                session,
                scene.expected_selection,
                timeout=timeout,
                debug_info=debug_info,
            )
            return
        finally:
            session.close()

    raise AssertionError(
        f"ui-enabled click did not select atoms {scene.expected_selection} at "
        f"({scene.click_target.x}, {scene.click_target.y}) for scene: {scene.description}"
    )
