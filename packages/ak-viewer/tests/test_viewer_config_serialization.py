from __future__ import annotations

from ak_viewer.viewer import RenderConfig, ViewerConfig
from ak_viewer.viewer._config import deserialize_config, serialize_config


def test_render_atom_palette_round_trips_through_process_config_payload():
    config = ViewerConfig(render=RenderConfig(atom_palette="jmol-metallic"))

    payload = serialize_config(config)
    restored = deserialize_config(payload)

    assert payload is not None
    assert payload["render"]["atom_palette"] == "jmol-metallic"
    assert restored is not None
    assert restored.render.atom_palette == "jmol-metallic"
