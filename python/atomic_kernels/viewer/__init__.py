from __future__ import annotations

import os
import sys
from typing import Optional

from ase import Atoms

from atomic_kernels._atomic_kernels import (
    ColorConfig,
    LightingConfig,
    PreparedHeadlessRender,
    PreparedViewerSession,
    RenderConfig,
    ViewerConfig,
    launch_viewer as _launch_viewer,
    prepare_render_viewer_image as _prepare_render_viewer_image,
    prepare_viewer_session as _prepare_viewer_session,
    run_viewer_session as _run_viewer_session,
    trajectory_viewer,
)

from ._camera import CameraController
from ._color import ColorController, ScalarRangeTracker, ViewerSelection
from ._process import spawn_process_viewer_session
from ._render import RenderController
from ._session import PreparedHeadlessRenderFacade, ViewerSessionFacade
from ._utils import normalize_atoms


def _viewer_session_requires_process() -> bool:
    if sys.platform == "darwin":
        return True
    return sys.platform.startswith("linux") and os.environ.get("CI") == "true"


def bevy_viewer(atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None) -> None:
    """Open the Bevy viewer in blocking mode for one structure or trajectory."""
    trajectory_viewer(normalize_atoms(atoms), config)


def launch_viewer(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> ViewerSession:
    """Launch a low-level live viewer session backed directly by Rust bindings."""
    return _launch_viewer(normalize_atoms(atoms), config)


def run_viewer_session(
    atoms: Atoms | list[Atoms], callback, config: Optional[ViewerConfig] = None
) -> None:
    """Run the viewer on the main thread and invoke ``callback`` with a low-level session."""
    _run_viewer_session(normalize_atoms(atoms), callback, config)


def prepare_viewer_session(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> PreparedViewerSession:
    """Prepare a low-level session object that can be started later."""
    return _prepare_viewer_session(normalize_atoms(atoms), config)


def viewer_session(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> ViewerSessionFacade:
    """Launch a high-level live viewer session with camera and color controllers."""
    frames = normalize_atoms(atoms)

    if _viewer_session_requires_process():
        return ViewerSessionFacade(spawn_process_viewer_session(frames, config), frames)

    return ViewerSessionFacade(_launch_viewer(frames, config), frames)


def headless_viewer_session(
    atoms: Atoms | list[Atoms],
    path: str,
    width: int = 800,
    height: int = 600,
    config: Optional[ViewerConfig] = None,
) -> PreparedHeadlessRenderFacade:
    """Prepare a scriptable headless session; call ``save()`` when ready."""
    frames = normalize_atoms(atoms)
    prepared = _prepare_render_viewer_image(
        frames,
        path,
        width=width,
        height=height,
        config=config,
    )
    return PreparedHeadlessRenderFacade(prepared, frames)


__all__ = [
    "ColorConfig",
    "ColorController",
    "CameraController",
    "LightingConfig",
    "PreparedHeadlessRender",
    "PreparedViewerSession",
    "PreparedHeadlessRenderFacade",
    "RenderController",
    "RenderConfig",
    "ScalarRangeTracker",
    "ViewerConfig",
    "ViewerSelection",
    "ViewerSession",
    "ViewerSessionFacade",
    "bevy_viewer",
    "headless_viewer_session",
    "launch_viewer",
    "prepare_viewer_session",
    "run_viewer_session",
    "viewer_session",
]
