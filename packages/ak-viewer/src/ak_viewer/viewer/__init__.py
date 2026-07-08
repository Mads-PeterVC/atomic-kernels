"""High-level Python viewer helpers and controller facades."""

from __future__ import annotations

import os
import sys
from typing import Optional

from ase import Atoms

from ak_viewer._ak_viewer import (
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
from ._chemistry import bonds_from_ase, faces_from_coordination, polyhedra_from_neighbors
from ._color import MaterialController, ScalarRangeTracker, ViewerSelection
from ._process import spawn_process_viewer_session
from ._quality import HIGH, LOW, MEDIUM, VERY_HIGH, QualityPreset
from ._render import RenderController
from ._session import PreparedHeadlessRenderFacade, ViewerSessionFacade
from ._utils import bonds_from_neighbor_list
from ._utils import normalize_atoms


def _viewer_session_requires_process() -> bool:
    if sys.platform == "darwin":
        return True
    return sys.platform.startswith("linux") and os.environ.get("CI") == "true"


def _interactive_default_config(config: Optional[ViewerConfig]) -> ViewerConfig:
    if config is not None:
        return config
    return ViewerConfig(render=RenderConfig(show_ui=True))


def bevy_viewer(atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None) -> None:
    """Open the interactive Bevy viewer and block until it closes.

    Parameters
    ----------
    atoms : ase.Atoms or list[ase.Atoms]
        Single structure or trajectory to display.
    config : ViewerConfig, optional
        Viewer configuration passed to the Rust backend.
    """
    trajectory_viewer(normalize_atoms(atoms), _interactive_default_config(config))


def launch_viewer(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> ViewerSession:
    """Launch a low-level live viewer session.

    Parameters
    ----------
    atoms : ase.Atoms or list[ase.Atoms]
        Single structure or trajectory to display.
    config : ViewerConfig, optional
        Viewer configuration passed to the Rust backend.

    Returns
    -------
    ViewerSession
        Low-level Rust-backed session object.
    """
    return _launch_viewer(normalize_atoms(atoms), _interactive_default_config(config))


def run_viewer_session(
    atoms: Atoms | list[Atoms], callback, config: Optional[ViewerConfig] = None
) -> None:
    """Run a viewer session on the main thread and invoke a callback.

    Parameters
    ----------
    atoms : ase.Atoms or list[ase.Atoms]
        Single structure or trajectory to display.
    callback : callable
        Callback that receives the low-level session handle while the viewer is running.
    config : ViewerConfig, optional
        Viewer configuration passed to the Rust backend.
    """
    _run_viewer_session(
        normalize_atoms(atoms), callback, _interactive_default_config(config)
    )


def prepare_viewer_session(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> PreparedViewerSession:
    """Prepare a low-level session that can be started later.

    Parameters
    ----------
    atoms : ase.Atoms or list[ase.Atoms]
        Single structure or trajectory to display.
    config : ViewerConfig, optional
        Viewer configuration passed to the Rust backend.

    Returns
    -------
    PreparedViewerSession
        Prepared session object that can be launched later.
    """
    return _prepare_viewer_session(
        normalize_atoms(atoms), _interactive_default_config(config)
    )


def viewer_session(
    atoms: Atoms | list[Atoms], config: Optional[ViewerConfig] = None
) -> ViewerSessionFacade:
    """Launch a high-level live viewer session.

    Parameters
    ----------
    atoms : ase.Atoms or list[ase.Atoms]
        Single structure or trajectory to display.
    config : ViewerConfig, optional
        Viewer configuration passed to the Rust backend.

    Returns
    -------
    ViewerSessionFacade
        Python facade exposing camera, appearance, render, and frame controls.
    """
    frames = normalize_atoms(atoms)
    config = _interactive_default_config(config)

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
    """Prepare a headless render session that can be scripted before saving.

    Parameters
    ----------
    atoms : ase.Atoms or list[ase.Atoms]
        Single structure or trajectory to render.
    path : str
        Output PNG path.
    width : int, default=800
        Output image width in pixels.
    height : int, default=600
        Output image height in pixels.
    config : ViewerConfig, optional
        Viewer configuration passed to the Rust backend.

    Returns
    -------
    PreparedHeadlessRenderFacade
        Scriptable facade whose :meth:`save` method performs the final render.
    """
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
    "CameraController",
    "LightingConfig",
    "MaterialController",
    "PreparedHeadlessRender",
    "PreparedViewerSession",
    "QualityPreset",
    "PreparedHeadlessRenderFacade",
    "RenderController",
    "RenderConfig",
    "ScalarRangeTracker",
    "ViewerConfig",
    "ViewerSelection",
    "ViewerSession",
    "ViewerSessionFacade",
    "bevy_viewer",
    "bonds_from_ase",
    "bonds_from_neighbor_list",
    "faces_from_coordination",
    "headless_viewer_session",
    "HIGH",
    "launch_viewer",
    "LOW",
    "MEDIUM",
    "VERY_HIGH",
    "polyhedra_from_neighbors",
    "prepare_viewer_session",
    "run_viewer_session",
    "viewer_session",
]
