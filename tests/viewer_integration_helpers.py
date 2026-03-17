from __future__ import annotations

import shutil
import subprocess
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Final

from ase import Atoms


REPO_ROOT: Final[Path] = Path(__file__).resolve().parents[1]
PYTHON_SRC: Final[Path] = REPO_ROOT / "python"
XVFB_SCREEN_WIDTH: Final[int] = 1280
XVFB_SCREEN_HEIGHT: Final[int] = 1024
VIEWER_WINDOW_CLICK_TARGET: Final[tuple[int, int]] = (
    XVFB_SCREEN_WIDTH // 2,
    XVFB_SCREEN_HEIGHT // 2,
)
VIEWER_SETTLE_DELAY_S: Final[float] = 0.75


@dataclass(frozen=True)
class ClickTarget:
    x: int
    y: int


@dataclass(frozen=True)
class ViewerClickScene:
    atoms: Atoms
    focus: tuple[float, float, float]
    radius: float
    click_target: ClickTarget
    expected_selection: list[int]
    description: str


def single_center_atom_scene() -> ViewerClickScene:
    focus = (4.0, 4.0, 4.0)
    return ViewerClickScene(
        atoms=Atoms(
            "He",
            positions=[focus],
            cell=[8.0, 8.0, 8.0],
            pbc=[False, False, False],
        ),
        focus=focus,
        radius=12.0,
        click_target=ClickTarget(*VIEWER_WINDOW_CLICK_TARGET),
        expected_selection=[0],
        description="single atom centered inside an 8x8x8 cell; click at the viewer-window center",
    )


def ui_viewer_config():
    from atomic_kernels.viewer import RenderConfig, ViewerConfig

    return ViewerConfig(
        render=RenderConfig(
            show_ui=True,
            show_orientation_widget=True,
        )
    )


def launch_ready_viewer_session(scene: ViewerClickScene, timeout: float):
    from atomic_kernels.viewer import viewer_session

    session = viewer_session(scene.atoms, config=ui_viewer_config())
    if not session.wait_until_ready(timeout=timeout):
        session.close()
        raise AssertionError(f"viewer did not become ready within {timeout:.1f}s")
    session.camera().look_at(scene.focus, radius=scene.radius, yaw=0.0, pitch=0.0)
    time.sleep(VIEWER_SETTLE_DELAY_S)
    return session


def wait_for_selected_atoms(session, expected: list[int], timeout: float) -> None:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        current = session.selected_atoms()
        if current == expected:
            return
        time.sleep(0.05)
    raise AssertionError(
        f"selection did not become {expected}; current selection is {session.selected_atoms()}"
    )


def has_xdotool() -> bool:
    return shutil.which("xdotool") is not None


def _viewer_process_id(session) -> int | None:
    backend = getattr(session, "_backend", None)
    process = getattr(backend, "_process", None)
    return getattr(process, "pid", None)


def _viewer_window_id(session) -> str:
    pid = _viewer_process_id(session)
    if pid is not None:
        result = subprocess.run(
            ["xdotool", "search", "--sync", "--onlyvisible", "--pid", str(pid)],
            check=True,
            capture_output=True,
            text=True,
        )
        window_ids = [line.strip() for line in result.stdout.splitlines() if line.strip()]
        if window_ids:
            return window_ids[-1]

    result = subprocess.run(
        ["xdotool", "search", "--sync", "--onlyvisible", "--name", "python3"],
        check=True,
        capture_output=True,
        text=True,
    )
    window_ids = [line.strip() for line in result.stdout.splitlines() if line.strip()]
    if not window_ids:
        raise AssertionError("could not find a visible viewer window for xdotool input")
    return window_ids[-1]


def click_viewer_at(session, target: ClickTarget) -> None:
    window_id = _viewer_window_id(session)
    subprocess.run(
        [
            "xdotool",
            "windowactivate",
            "--sync",
            window_id,
            "mousemove",
            "--window",
            window_id,
            str(target.x),
            str(target.y),
            "click",
            "--window",
            window_id,
            "1",
        ],
        check=True,
    )
