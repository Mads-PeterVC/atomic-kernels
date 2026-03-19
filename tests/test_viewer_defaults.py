from __future__ import annotations

from ase import Atoms


def test_bevy_viewer_enables_ui_by_default(monkeypatch):
    calls = []

    monkeypatch.setattr(
        "atomic_kernels.viewer.trajectory_viewer",
        lambda atoms, config=None: calls.append((atoms, config)),
    )

    from atomic_kernels.viewer import bevy_viewer

    atoms = Atoms("H2")
    bevy_viewer(atoms)

    assert len(calls) == 1
    _, config = calls[0]
    assert config is not None
    assert config.render.show_ui is True


def test_viewer_session_enables_ui_by_default(monkeypatch):
    calls = []

    monkeypatch.setattr("atomic_kernels.viewer._viewer_session_requires_process", lambda: False)
    monkeypatch.setattr(
        "atomic_kernels.viewer._launch_viewer",
        lambda atoms, config=None: calls.append((atoms, config)) or object(),
    )

    from atomic_kernels.viewer import viewer_session

    atoms = Atoms("H2")
    viewer_session(atoms)

    assert len(calls) == 1
    _, config = calls[0]
    assert config is not None
    assert config.render.show_ui is True


def test_viewer_session_preserves_explicit_ui_override(monkeypatch):
    calls = []

    monkeypatch.setattr("atomic_kernels.viewer._viewer_session_requires_process", lambda: False)
    monkeypatch.setattr(
        "atomic_kernels.viewer._launch_viewer",
        lambda atoms, config=None: calls.append((atoms, config)) or object(),
    )

    from atomic_kernels.viewer import RenderConfig, ViewerConfig, viewer_session

    config = ViewerConfig(render=RenderConfig(show_ui=False))
    viewer_session(Atoms("H2"), config=config)

    assert len(calls) == 1
    _, forwarded = calls[0]
    assert forwarded is config
    assert forwarded.render.show_ui is False


def test_render_config_accepts_atom_palette_argument():
    from atomic_kernels.viewer import RenderConfig

    config = RenderConfig(atom_palette="jmol-metallic")

    assert config.atom_palette == "jmol-metallic"
