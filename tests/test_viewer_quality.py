from __future__ import annotations

from atomic_kernels.viewer import (
    HIGH,
    LOW,
    MEDIUM,
    VERY_HIGH,
    QualityPreset,
    RenderConfig,
    ViewerConfig,
)


def test_quality_presets_map_to_expected_ico_subdiv():
    assert LOW.ico_subdiv == 3
    assert MEDIUM.ico_subdiv == 4
    assert HIGH.ico_subdiv == 5
    assert VERY_HIGH.ico_subdiv == 6


def test_quality_preset_from_name_is_case_insensitive():
    preset = QualityPreset.from_name("HiGh")

    assert preset.name == "high"
    assert preset.ico_subdiv == 5


def test_quality_preset_short_aliases_resolve_cleanly():
    assert QualityPreset.from_name("l").name == "low"
    assert QualityPreset.from_name("m").name == "medium"
    assert QualityPreset.from_name("h").name == "high"
    assert QualityPreset.from_name("vh").name == "very_high"


def test_quality_preset_apply_to_render_preserves_unrelated_fields():
    render = RenderConfig(show_cell=False, show_ui=True)

    updated = MEDIUM.apply_to_render(render)

    assert updated is render
    assert updated.ico_subdiv == 4
    assert updated.show_cell is False
    assert updated.show_ui is True


def test_quality_preset_apply_to_viewer_preserves_unrelated_fields():
    config = ViewerConfig(render=RenderConfig(show_cell=False, show_ui=True))
    config.window_width = 1234

    updated = HIGH.apply_to_viewer(config)

    assert updated is config
    assert updated.render.ico_subdiv == 5
    assert updated.render.show_cell is False
    assert updated.render.show_ui is True
    assert updated.window_width == 1234


def test_explicit_render_override_can_follow_quality_preset():
    config = LOW.apply_to_viewer(ViewerConfig())

    render = config.render
    render.ico_subdiv = 7
    config.render = render

    assert config.render.ico_subdiv == 7
