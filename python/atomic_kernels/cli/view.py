"""Viewer-focused CLI commands."""

from __future__ import annotations

from pathlib import Path

import rich_click as click
from ase.io import read

from atomic_kernels.viewer import (
    ColorConfig,
    LightingConfig,
    QualityPreset,
    RenderConfig,
    ViewerConfig,
    bevy_viewer,
)

DEFAULT_WINDOW_WIDTH = 950
DEFAULT_WINDOW_HEIGHT = 650
QUALITY_OPTION_METAVAR = "[low (l)|medium (m)|high (h)|very_high (vh)]"

click.rich_click.OPTION_GROUPS = {
    "ak view": [
        {
            "name": "Display Options",
            "options": ["--width", "--height", "--theme", "--quality"],
        },
        {
            "name": "Toggles",
            "options": ["--ui", "--cell"],
        },
    ]
}


def positive_int(ctx: click.Context, param: click.Parameter, value: int | None) -> int | None:
    """Reject non-positive integer CLI values."""
    if value is not None and value <= 0:
        raise click.BadParameter("must be a positive integer")
    return value


def quality_value(
    ctx: click.Context, param: click.Parameter, value: str | None
) -> str | None:
    """Normalize and validate CLI quality values."""
    if value is None:
        return None

    try:
        return QualityPreset.from_name(value).name
    except ValueError as exc:
        raise click.BadParameter(str(exc)) from exc


def load_atoms(path: Path):
    """Load an ASE-readable file, returning one frame or a trajectory."""
    atoms = read(str(path), index=":")
    if isinstance(atoms, list) and len(atoms) == 1:
        return atoms[0]
    return atoms


def build_viewer_config(
    *,
    theme: str,
    quality: str,
    width: int | None,
    height: int | None,
    show_ui: bool,
    show_cell: bool,
) -> ViewerConfig:
    """Translate CLI display options into a ViewerConfig."""
    config = quality_preset_from_name(quality).apply_to_viewer(ViewerConfig())

    if theme == "dark":
        config.color = ColorConfig(
            background=(0.1, 0.1, 0.1),
            cell_color=(0.8, 0.8, 0.8),
        )
        config.lighting = LightingConfig(
            ambient_brightness=150.0,
            fill_illuminance=0.0,
            key_illuminance=0.0,
            enable_fog=True,
        )

    render = config.render
    render.show_ui = show_ui
    render.show_cell = show_cell
    config.render = render
    config.window_width = width if width is not None else DEFAULT_WINDOW_WIDTH
    config.window_height = height if height is not None else DEFAULT_WINDOW_HEIGHT
    return config


def quality_preset_from_name(name: str) -> QualityPreset:
    return QualityPreset.from_name(name)


@click.command("view")
@click.argument(
    "file",
    type=click.Path(exists=True, dir_okay=False, path_type=Path),
)
@click.option(
    "-w",
    "--width",
    type=int,
    callback=positive_int,
    default=None,
    help="Viewer window width in pixels.",
)
@click.option(
    "-h",
    "--height",
    type=int,
    callback=positive_int,
    default=None,
    help="Viewer window height in pixels.",
)
@click.option(
    "-t",
    "--theme",
    type=click.Choice(("light", "dark"), case_sensitive=False),
    default="light",
    show_default=True,
    help="Viewer appearance preset.",
)
@click.option(
    "-q",
    "--quality",
    type=str,
    callback=quality_value,
    metavar=QUALITY_OPTION_METAVAR,
    default="medium",
    show_default=True,
    help="Viewer visual fidelity preset: low (l), medium (m), high (h), very_high (vh).",
)
@click.option(
    "--ui/--no-ui",
    "show_ui",
    default=True,
    show_default=True,
    help="Show the viewer UI overlay.",
)
@click.option(
    "--cell/--no-cell",
    "show_cell",
    default=True,
    show_default=True,
    help="Show the unit cell outline.",
)
def view_command(
    file: Path,
    width: int | None,
    height: int | None,
    theme: str,
    quality: str,
    show_ui: bool,
    show_cell: bool,
) -> None:
    """Open an ASE-readable structure or trajectory in the viewer."""
    atoms = load_atoms(file)
    config = build_viewer_config(
        theme=theme.lower(),
        quality=quality.lower(),
        width=width,
        height=height,
        show_ui=show_ui,
        show_cell=show_cell,
    )
    bevy_viewer(atoms, config=config)
