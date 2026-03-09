import numpy as np
from numpy.typing import NDArray


class LightingConfig:
    ambient_brightness: float
    key_illuminance: float
    fill_illuminance: float
    back_illuminance: float
    camera_illuminance: float
    enable_fog: bool
    def __init__(
        self,
        ambient_brightness: float = 100.0,
        key_illuminance: float = 5000.0,
        fill_illuminance: float = 1000.0,
        back_illuminance: float = 800.0,
        camera_illuminance: float = 2000.0,
        enable_fog: bool = False,
    ) -> None: ...


class ColorConfig:
    @property
    def background(self) -> tuple[float, float, float]: ...
    @background.setter
    def background(self, value: tuple[float, float, float]) -> None: ...
    @property
    def cell_color(self) -> tuple[float, float, float]: ...
    @cell_color.setter
    def cell_color(self, value: tuple[float, float, float]) -> None: ...
    def __init__(
        self,
        background: tuple[float, float, float] = (0.98, 0.98, 0.98),
        cell_color: tuple[float, float, float] = (0.0, 0.0, 0.0),
    ) -> None: ...


class RenderConfig:
    show_cell: bool
    show_axes: bool
    show_ui: bool
    ico_subdiv: int
    def __init__(
        self,
        show_cell: bool = True,
        show_axes: bool = True,
        show_ui: bool = False,
        ico_subdiv: int = 5,
    ) -> None: ...


class ViewerConfig:
    @property
    def color(self) -> ColorConfig: ...
    @color.setter
    def color(self, value: ColorConfig) -> None: ...
    @property
    def lighting(self) -> LightingConfig: ...
    @lighting.setter
    def lighting(self, value: LightingConfig) -> None: ...
    @property
    def render(self) -> RenderConfig: ...
    @render.setter
    def render(self, value: RenderConfig) -> None: ...
    @property
    def initial_frame(self) -> int: ...
    @initial_frame.setter
    def initial_frame(self, value: int) -> None: ...
    def __init__(
        self,
        color: ColorConfig | None = None,
        lighting: LightingConfig | None = None,
        render: RenderConfig | None = None,
        initial_frame: int = 0,
    ) -> None: ...


class ViewerSession:
    def append_frame(self, frame: object) -> None: ...
    def set_frame(self, index: int) -> None: ...
    def follow_tail(self, enabled: bool = True) -> None: ...
    def close(self) -> None: ...


class PreparedViewerSession:
    @property
    def session(self) -> ViewerSession: ...
    def run(self) -> None: ...


def distance_matrix(
    positions: NDArray[np.float64],
    numbers: NDArray[np.int32],
    cell: NDArray[np.float64],
    pbc: NDArray[np.bool_],
) -> NDArray[np.float64]: ...


def launch_viewer(trajectory: list[object], config: ViewerConfig | None = None) -> ViewerSession: ...
def prepare_viewer_session(
    trajectory: list[object], config: ViewerConfig | None = None
) -> PreparedViewerSession: ...
def run_viewer_session(
    trajectory: list[object],
    callback: object,
    config: ViewerConfig | None = None,
) -> None: ...
def viewer_session(trajectory: list[object], config: ViewerConfig | None = None) -> ViewerSession: ...
def trajectory_viewer(trajectory: list[object], config: ViewerConfig | None = None) -> None: ...
def viewer(structure: object, config: ViewerConfig | None = None) -> None: ...
