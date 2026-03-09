from __future__ import annotations

import sys
from typing import Optional

from ase import Atoms

from atomic_kernels._atomic_kernels import (
    ColorConfig,
    LightingConfig,
    PreparedViewerSession,
    RenderConfig,
    ViewerConfig,
    ViewerSession,
    launch_viewer as _launch_viewer,
    prepare_viewer_session as _prepare_viewer_session,
    run_viewer_session as _run_viewer_session,
    trajectory_viewer,
)

from ._camera import CameraController
from ._color import ColorController, ScalarRangeTracker, ViewerSelection
from ._process import spawn_process_viewer_session
from ._session import ViewerSessionFacade
from ._utils import normalize_atoms


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

    if sys.platform != "darwin":
        return ViewerSessionFacade(_launch_viewer(frames, config), frames)

    return ViewerSessionFacade(spawn_process_viewer_session(frames, config), frames)


__all__ = [
    "ColorConfig",
    "ColorController",
    "CameraController",
    "LightingConfig",
    "PreparedViewerSession",
    "RenderConfig",
    "ScalarRangeTracker",
    "ViewerConfig",
    "ViewerSelection",
    "ViewerSession",
    "ViewerSessionFacade",
    "bevy_viewer",
    "launch_viewer",
    "prepare_viewer_session",
    "run_viewer_session",
    "viewer_session",
]
