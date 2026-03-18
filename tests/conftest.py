from __future__ import annotations

import os
import sys
import types
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PYTHON_SRC = ROOT / "python"
USE_REAL_EXTENSION = os.environ.get("ATOMIC_KERNELS_USE_REAL_EXTENSION") == "1"
if not USE_REAL_EXTENSION and str(PYTHON_SRC) not in sys.path:
    sys.path.insert(0, str(PYTHON_SRC))


class _ColorConfig:
    def __init__(self, background=(0.98, 0.98, 0.98), cell_color=(0.0, 0.0, 0.0)):
        self.background = background
        self.cell_color = cell_color


class _LightingConfig:
    def __init__(
        self,
        ambient_brightness=100.0,
        key_illuminance=5000.0,
        fill_illuminance=1000.0,
        back_illuminance=800.0,
        camera_illuminance=2000.0,
        enable_fog=False,
    ):
        self.ambient_brightness = ambient_brightness
        self.key_illuminance = key_illuminance
        self.fill_illuminance = fill_illuminance
        self.back_illuminance = back_illuminance
        self.camera_illuminance = camera_illuminance
        self.enable_fog = enable_fog


class _RenderConfig:
    def __init__(
        self,
        show_cell=True,
        show_axes=True,
        show_ui=False,
        ico_subdiv=5,
        show_orientation_widget=True,
        orientation_widget_size_px=100,
        orientation_widget_margin_px=0,
        orientation_widget_offset_x_px=0,
        orientation_widget_offset_y_px=0,
        orientation_widget_camera_scale=0.065,
    ):
        self.show_cell = show_cell
        self.show_axes = show_axes
        self.show_ui = show_ui
        self.ico_subdiv = ico_subdiv
        self.show_orientation_widget = show_orientation_widget
        self.orientation_widget_size_px = orientation_widget_size_px
        self.orientation_widget_margin_px = orientation_widget_margin_px
        self.orientation_widget_offset_x_px = orientation_widget_offset_x_px
        self.orientation_widget_offset_y_px = orientation_widget_offset_y_px
        self.orientation_widget_camera_scale = orientation_widget_camera_scale


class _ViewerConfig:
    def __init__(
        self,
        color=None,
        lighting=None,
        render=None,
        initial_frame=0,
        window_width=None,
        window_height=None,
    ):
        self.color = color or _ColorConfig()
        self.lighting = lighting or _LightingConfig()
        self.render = render or _RenderConfig()
        self.initial_frame = initial_frame
        self.window_width = window_width
        self.window_height = window_height


class _PreparedViewerSession:
    def __init__(self, session=None):
        self.session = session

    def run(self):
        return None


class _PreparedHeadlessRender:
    def __init__(self, session=None):
        self.session = session

    def save(self):
        return None


class _ViewerSession:
    def wait_until_ready(self, timeout=None):
        return True

    def close(self):
        return None

    def append_frame(self, frame):
        return None

    def set_frame(self, index):
        return None

    def follow_tail(self, enabled=True):
        return None

    def set_atom_scalars(self, name, values, frame_index=None):
        return None

    def color_by_scalar(
        self, name, palette="viridis", colors=None, min=None, max=None, append=False
    ):
        return None

    def reset_atom_colors(self):
        return None

    def set_bonds(self, bonds, frame_index=None):
        return None

    def set_faces(self, faces, color=(0.2, 0.6, 0.9, 0.35), face_colors=None, frame_index=None):
        return None

    def set_ball_and_stick_style(
        self,
        selection,
        atom_scale=0.45,
        bond_radius=0.08,
        bond_color=(0.7, 0.7, 0.7, 1.0),
        bond_scope="both_selected",
        frame_index=None,
        append=False,
    ):
        return None

    def reset_render_style(self):
        return None

    def selected_atoms(self, frame_index=None):
        return []

    def set_selection(self, selection, frame_index=None):
        return None

    def add_selection(self, selection, frame_index=None):
        return None

    def remove_selection(self, selection, frame_index=None):
        return None

    def clear_selection(self, frame_index=None):
        return None

    def set_camera_view(self, focus=None, radius=None, yaw=None, pitch=None):
        return None

    def pan_camera(self, delta):
        return None

    def zoom_camera(self, factor=None, delta=None):
        return None

    def orbit_camera(self, yaw_delta=0.0, pitch_delta=0.0):
        return None

    def frame_all(self):
        return None

    def start_orbit(self, yaw_rate=0.5, pitch_rate=0.0):
        return None

    def stop_camera_motion(self):
        return None


def _install_extension_stub() -> None:
    if USE_REAL_EXTENSION:
        return
    if "atomic_kernels._atomic_kernels" in sys.modules:
        return

    module = types.ModuleType("atomic_kernels._atomic_kernels")
    module.ColorConfig = _ColorConfig
    module.LightingConfig = _LightingConfig
    module.RenderConfig = _RenderConfig
    module.ViewerConfig = _ViewerConfig
    module.PreparedViewerSession = _PreparedViewerSession
    module.PreparedHeadlessRender = _PreparedHeadlessRender
    module.ViewerSession = _ViewerSession
    module.launch_viewer = lambda atoms, config=None: _ViewerSession()
    module.prepare_viewer_session = (
        lambda atoms, config=None: _PreparedViewerSession(_ViewerSession())
    )
    module.prepare_render_viewer_image = (
        lambda atoms, path, width=800, height=600, config=None: _PreparedHeadlessRender(
            _ViewerSession()
        )
    )
    module.run_viewer_session = lambda atoms, callback, config=None: callback(
        _ViewerSession()
    )
    module.trajectory_viewer = lambda atoms, config=None: None
    module.viewer = lambda atoms, config=None: None
    module.neighborlist = lambda atoms, cutoff: ([], [], [])
    sys.modules[module.__name__] = module


_install_extension_stub()
