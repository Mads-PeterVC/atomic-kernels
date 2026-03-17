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
VIEWER_SETTLE_DELAY_S: Final[float] = 0.75
VIEWER_POST_CLICK_DELAY_S: Final[float] = 0.2
CLICK_SEARCH_OFFSETS: Final[tuple[tuple[int, int], ...]] = (
    (0, 0),
    (-24, 0),
    (24, 0),
    (0, -24),
    (0, 24),
    (-16, -16),
    (16, -16),
    (-16, 16),
    (16, 16),
)


@dataclass(frozen=True)
class ClickTarget:
    x: float
    y: float
    normalized_to_window: bool = False


@dataclass(frozen=True)
class ViewerClickScene:
    atoms: Atoms
    focus: tuple[float, float, float]
    radius: float
    click_target: ClickTarget
    expected_selection: list[int]
    description: str


@dataclass(frozen=True)
class ClickDebugInfo:
    process_id: int | None
    window_id: str
    window_geometry: dict[str, int]
    resolved_points: list[tuple[int, int]]


def single_center_atom_scene() -> ViewerClickScene:
    focus = (6.0, 6.0, 6.0)
    return ViewerClickScene(
        atoms=Atoms(
            "Cs",
            positions=[focus],
            cell=[12.0, 12.0, 12.0],
            pbc=[False, False, False],
        ),
        focus=focus,
        radius=9.0,
        click_target=ClickTarget(0.5, 0.5, normalized_to_window=True),
        expected_selection=[0],
        description="single large Cs atom centered inside a 12x12x12 cell; click at the viewer-window center",
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
    time.sleep(1.5 if timeout >= 60.0 else VIEWER_SETTLE_DELAY_S)
    return session


def wait_for_selected_atoms(
    session, expected: list[int], timeout: float, debug_info: ClickDebugInfo | None = None
) -> None:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        current = session.selected_atoms()
        if current == expected:
            return
        time.sleep(0.05)
    debug_suffix = ""
    if debug_info is not None:
        debug_suffix = (
            f"; click debug pid={debug_info.process_id} window={debug_info.window_id} "
            f"geometry={debug_info.window_geometry} "
            f"resolved_clicks={debug_info.resolved_points}"
        )
    raise AssertionError(
        f"selection did not become {expected}; current selection is {session.selected_atoms()}"
        f"{debug_suffix}"
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


def _window_geometry(window_id: str) -> dict[str, int]:
    result = subprocess.run(
        ["xdotool", "getwindowgeometry", "--shell", window_id],
        check=True,
        capture_output=True,
        text=True,
    )
    geometry: dict[str, int] = {}
    for line in result.stdout.splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        try:
            geometry[key] = int(value)
        except ValueError:
            continue
    return geometry


def _resolve_click_target(window_id: str, target: ClickTarget) -> tuple[int, int]:
    if not target.normalized_to_window:
        return int(target.x), int(target.y)

    geometry = _window_geometry(window_id)
    width = geometry.get("WIDTH")
    height = geometry.get("HEIGHT")
    if width is None or height is None:
        raise AssertionError("xdotool did not report WIDTH/HEIGHT for the viewer window")
    return int(round(target.x * width)), int(round(target.y * height))


def _candidate_click_points(base_x: int, base_y: int, geometry: dict[str, int]) -> list[tuple[int, int]]:
    width = geometry.get("WIDTH")
    height = geometry.get("HEIGHT")
    if width is None or height is None:
        return [(base_x, base_y)]

    points: list[tuple[int, int]] = []
    for dx, dy in CLICK_SEARCH_OFFSETS:
        x = min(max(base_x + dx, 1), max(width - 2, 1))
        y = min(max(base_y + dy, 1), max(height - 2, 1))
        point = (x, y)
        if point not in points:
            points.append(point)
    return points


def click_viewer_at(session, target: ClickTarget) -> ClickDebugInfo:
    window_id = _viewer_window_id(session)
    process_id = _viewer_process_id(session)
    geometry = _window_geometry(window_id)
    resolved_x, resolved_y = _resolve_click_target(window_id, target)
    resolved_points = _candidate_click_points(resolved_x, resolved_y, geometry)
    for point_x, point_y in resolved_points:
        subprocess.run(
            [
                "xdotool",
                "mousemove",
                "--sync",
                "--window",
                window_id,
                str(point_x),
                str(point_y),
            ],
            check=True,
        )
        subprocess.run(
            [
                "xdotool",
                "click",
                "--window",
                window_id,
                "1",
            ],
            check=True,
        )
        time.sleep(VIEWER_POST_CLICK_DELAY_S)
    return ClickDebugInfo(
        process_id=process_id,
        window_id=window_id,
        window_geometry=geometry,
        resolved_points=resolved_points,
    )
