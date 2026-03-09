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

from ._process import spawn_process_viewer_session
from ._session import ScalarRangeTracker, ViewerSelection, ViewerSessionFacade
from ._utils import normalize_atoms


def bevy_viewer(atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None) -> None:
    trajectory_viewer(normalize_atoms(atoms), config)


def launch_viewer(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> ViewerSession:
    return _launch_viewer(normalize_atoms(atoms), config)


def run_viewer_session(
    atoms: Atoms | list[Atoms], callback, config: Optional[ViewerConfig] = None
) -> None:
    _run_viewer_session(normalize_atoms(atoms), callback, config)


def prepare_viewer_session(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> PreparedViewerSession:
    return _prepare_viewer_session(normalize_atoms(atoms), config)


def viewer_session(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> ViewerSessionFacade:
    frames = normalize_atoms(atoms)

    if sys.platform != "darwin":
        return ViewerSessionFacade(_launch_viewer(frames, config), frames)

    return ViewerSessionFacade(spawn_process_viewer_session(frames, config), frames)


__all__ = [
    "ColorConfig",
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
