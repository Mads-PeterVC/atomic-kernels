from __future__ import annotations

from pathlib import Path

import pytest
from ase import Atoms
from click.testing import CliRunner

from atomic_kernels.cli import main
from atomic_kernels.cli.view import (
    DEFAULT_WINDOW_HEIGHT,
    DEFAULT_WINDOW_WIDTH,
    build_viewer_config,
)


def test_ak_help_lists_view_command():
    runner = CliRunner()

    result = runner.invoke(main, ["--help"])

    assert result.exit_code == 0
    assert "view" in result.output


def test_ak_view_help_shows_grouped_flags():
    runner = CliRunner()

    result = runner.invoke(main, ["view", "--help"])

    assert result.exit_code == 0
    assert "Display Options" in result.output
    assert "Toggles" in result.output
    assert "Display Size" not in result.output
    assert "--width" in result.output
    assert "--height" in result.output
    assert "--theme" in result.output
    assert "--quality" in result.output
    assert "--atom-palette" in result.output
    assert "low (l)" in result.output
    assert "medium (m)" in result.output
    assert "high (h)" in result.output
    assert "very_high (vh)" in result.output
    assert "[low|medium|high|very_high|l|m|h|vh]" not in result.output
    assert "--ui" in result.output
    assert "--no-ui" in result.output
    assert "--cell" in result.output
    assert "--no-cell" in result.output
    assert "-w" in result.output
    assert "-h" in result.output
    assert "-t" in result.output


def test_ak_view_reads_single_frame_and_launches_viewer(monkeypatch, tmp_path: Path):
    runner = CliRunner()
    structure_path = tmp_path / "single.xyz"
    structure_path.write_text("placeholder\n", encoding="utf-8")
    atoms = Atoms("H2")
    calls: list[tuple[object, object]] = []

    monkeypatch.setattr("atomic_kernels.cli.view.read", lambda path, index: [atoms])
    monkeypatch.setattr(
        "atomic_kernels.cli.view.bevy_viewer",
        lambda loaded, config=None: calls.append((loaded, config)),
    )

    result = runner.invoke(main, ["view", str(structure_path)])

    assert result.exit_code == 0
    assert len(calls) == 1
    loaded, config = calls[0]
    assert loaded is atoms
    assert config is not None
    assert config.window_width == DEFAULT_WINDOW_WIDTH
    assert config.window_height == DEFAULT_WINDOW_HEIGHT
    assert config.render.show_ui is True
    assert config.render.show_cell is True
    assert config.render.atom_palette == "jmol"


def test_ak_view_reads_all_frames_for_trajectory(monkeypatch, tmp_path: Path):
    runner = CliRunner()
    structure_path = tmp_path / "traj.xyz"
    structure_path.write_text("placeholder\n", encoding="utf-8")
    frames = [Atoms("H"), Atoms("He")]
    reads: list[tuple[str, str]] = []
    calls: list[tuple[object, object]] = []

    def fake_read(path: str, index: str):
        reads.append((path, index))
        return frames

    monkeypatch.setattr("atomic_kernels.cli.view.read", fake_read)
    monkeypatch.setattr(
        "atomic_kernels.cli.view.bevy_viewer",
        lambda loaded, config=None: calls.append((loaded, config)),
    )

    result = runner.invoke(main, ["view", str(structure_path)])

    assert result.exit_code == 0
    assert reads == [(str(structure_path), ":")]
    assert len(calls) == 1
    loaded, config = calls[0]
    assert loaded is frames
    assert config is not None
    assert config.window_width == DEFAULT_WINDOW_WIDTH
    assert config.window_height == DEFAULT_WINDOW_HEIGHT
    assert config.render.show_ui is True
    assert config.render.show_cell is True
    assert config.render.atom_palette == "jmol"


def test_dark_theme_builds_expected_viewer_config():
    config = build_viewer_config(
        theme="dark",
        quality="medium",
        width=None,
        height=None,
        show_ui=True,
        show_cell=True,
    )

    assert config is not None
    assert config.color.background == pytest.approx((0.1, 0.1, 0.1))
    assert config.color.cell_color == pytest.approx((0.8, 0.8, 0.8))
    assert config.lighting.ambient_brightness == 150.0
    assert config.lighting.fill_illuminance == 0.0
    assert config.lighting.key_illuminance == 0.0
    assert config.lighting.enable_fog is True
    assert config.render.ico_subdiv == 4
    assert config.render.show_ui is True
    assert config.render.show_cell is True


def test_window_size_options_populate_viewer_config_defaults():
    config = build_viewer_config(
        theme="light",
        quality="medium",
        width=1600,
        height=None,
        show_ui=True,
        show_cell=True,
    )

    assert config is not None
    assert config.window_width == 1600
    assert config.window_height == DEFAULT_WINDOW_HEIGHT

    config = build_viewer_config(
        theme="light",
        quality="medium",
        width=None,
        height=900,
        show_ui=True,
        show_cell=True,
    )

    assert config is not None
    assert config.window_width == DEFAULT_WINDOW_WIDTH
    assert config.window_height == 900


def test_light_theme_still_builds_default_windowed_config():
    config = build_viewer_config(
        theme="light",
        quality="medium",
        width=None,
        height=None,
        show_ui=True,
        show_cell=True,
    )

    assert config is not None
    assert config.window_width == DEFAULT_WINDOW_WIDTH
    assert config.window_height == DEFAULT_WINDOW_HEIGHT
    assert config.render.ico_subdiv == 4
    assert config.render.show_ui is True
    assert config.render.show_cell is True


def test_display_toggles_populate_render_config():
    config = build_viewer_config(
        theme="light",
        quality="medium",
        width=None,
        height=None,
        show_ui=True,
        show_cell=False,
    )

    assert config.render.show_ui is True
    assert config.render.show_cell is False


def test_atom_palette_populates_render_config():
    config = build_viewer_config(
        theme="light",
        quality="medium",
        atom_palette="jmol-metallic",
        width=None,
        height=None,
        show_ui=True,
        show_cell=True,
    )

    assert config.render.atom_palette == "jmol-metallic"


def test_quality_preset_populates_render_config():
    low = build_viewer_config(
        theme="light",
        quality="low",
        width=None,
        height=None,
        show_ui=True,
        show_cell=True,
    )
    high = build_viewer_config(
        theme="light",
        quality="high",
        width=None,
        height=None,
        show_ui=True,
        show_cell=True,
    )
    very_high = build_viewer_config(
        theme="light",
        quality="very_high",
        width=None,
        height=None,
        show_ui=True,
        show_cell=True,
    )

    assert low.render.ico_subdiv == 3
    assert high.render.ico_subdiv == 5
    assert very_high.render.ico_subdiv == 7


def test_quality_short_aliases_populate_render_config():
    config = build_viewer_config(
        theme="light",
        quality="vh",
        width=None,
        height=None,
        show_ui=True,
        show_cell=True,
    )

    assert config.render.ico_subdiv == 7


def test_dark_theme_and_quality_are_independent():
    config = build_viewer_config(
        theme="dark",
        quality="high",
        width=None,
        height=None,
        show_ui=False,
        show_cell=True,
    )

    assert config.color.background == pytest.approx((0.1, 0.1, 0.1))
    assert config.lighting.enable_fog is True
    assert config.render.ico_subdiv == 5
    assert config.render.show_ui is False


def test_ak_view_accepts_short_quality_alias(monkeypatch, tmp_path: Path):
    runner = CliRunner()
    structure_path = tmp_path / "single.xyz"
    structure_path.write_text("placeholder\n", encoding="utf-8")
    atoms = Atoms("H2")
    calls: list[tuple[object, object]] = []

    monkeypatch.setattr("atomic_kernels.cli.view.read", lambda path, index: [atoms])
    monkeypatch.setattr(
        "atomic_kernels.cli.view.bevy_viewer",
        lambda loaded, config=None: calls.append((loaded, config)),
    )

    result = runner.invoke(main, ["view", str(structure_path), "-q", "vh"])

    assert result.exit_code == 0
    assert len(calls) == 1
    _, config = calls[0]
    assert config is not None
    assert config.render.ico_subdiv == 7


def test_ak_view_passes_ui_and_cell_flags(monkeypatch, tmp_path: Path):
    runner = CliRunner()
    structure_path = tmp_path / "single.xyz"
    structure_path.write_text("placeholder\n", encoding="utf-8")
    atoms = Atoms("H2")
    calls: list[tuple[object, object]] = []

    monkeypatch.setattr("atomic_kernels.cli.view.read", lambda path, index: [atoms])
    monkeypatch.setattr(
        "atomic_kernels.cli.view.bevy_viewer",
        lambda loaded, config=None: calls.append((loaded, config)),
    )

    result = runner.invoke(
        main,
        ["view", str(structure_path), "--quality", "high", "--ui", "--no-cell"],
    )

    assert result.exit_code == 0
    assert len(calls) == 1
    _, config = calls[0]
    assert config is not None
    assert config.render.ico_subdiv == 5
    assert config.render.show_ui is True
    assert config.render.show_cell is False


def test_ak_view_passes_atom_palette(monkeypatch, tmp_path: Path):
    runner = CliRunner()
    structure_path = tmp_path / "single.xyz"
    structure_path.write_text("placeholder\n", encoding="utf-8")
    atoms = Atoms("H2")
    calls: list[tuple[object, object]] = []

    monkeypatch.setattr("atomic_kernels.cli.view.read", lambda path, index: [atoms])
    monkeypatch.setattr(
        "atomic_kernels.cli.view.bevy_viewer",
        lambda loaded, config=None: calls.append((loaded, config)),
    )

    result = runner.invoke(
        main,
        ["view", str(structure_path), "--atom-palette", "jmol-metallic"],
    )

    assert result.exit_code == 0
    assert len(calls) == 1
    _, config = calls[0]
    assert config is not None
    assert config.render.atom_palette == "jmol-metallic"


def test_ak_view_defaults_enable_ui_and_cell(monkeypatch, tmp_path: Path):
    runner = CliRunner()
    structure_path = tmp_path / "single.xyz"
    structure_path.write_text("placeholder\n", encoding="utf-8")
    atoms = Atoms("H2")
    calls: list[tuple[object, object]] = []

    monkeypatch.setattr("atomic_kernels.cli.view.read", lambda path, index: [atoms])
    monkeypatch.setattr(
        "atomic_kernels.cli.view.bevy_viewer",
        lambda loaded, config=None: calls.append((loaded, config)),
    )

    result = runner.invoke(main, ["view", str(structure_path)])

    assert result.exit_code == 0
    assert len(calls) == 1
    _, config = calls[0]
    assert config is not None
    assert config.render.show_ui is True
    assert config.render.show_cell is True


def test_ak_view_rejects_invalid_width(tmp_path: Path):
    runner = CliRunner()
    structure_path = tmp_path / "single.xyz"
    structure_path.write_text("placeholder\n", encoding="utf-8")

    result = runner.invoke(main, ["view", str(structure_path), "--width", "0"])

    assert result.exit_code != 0
    assert "must be a positive integer" in result.output


def test_ak_view_rejects_invalid_quality(tmp_path: Path):
    runner = CliRunner()
    structure_path = tmp_path / "single.xyz"
    structure_path.write_text("placeholder\n", encoding="utf-8")

    result = runner.invoke(main, ["view", str(structure_path), "-q", "nope"])

    assert result.exit_code != 0
    assert "unsupported quality preset" in result.output


def test_ak_view_missing_file_reports_click_error(tmp_path: Path):
    runner = CliRunner()
    missing_path = tmp_path / "missing.xyz"

    result = runner.invoke(main, ["view", str(missing_path)])

    assert result.exit_code != 0
    assert "does not exist" in result.output
