from __future__ import annotations

from typing import Optional

from atomic_kernels._atomic_kernels import (
    ColorConfig,
    LightingConfig,
    RenderConfig,
    ViewerConfig,
)


def serialize_config(config: Optional[ViewerConfig]) -> Optional[dict]:
    if config is None:
        return None

    return {
        "color": {
            "background": config.color.background,
            "cell_color": config.color.cell_color,
        },
        "lighting": {
            "ambient_brightness": config.lighting.ambient_brightness,
            "key_illuminance": config.lighting.key_illuminance,
            "fill_illuminance": config.lighting.fill_illuminance,
            "back_illuminance": config.lighting.back_illuminance,
            "camera_illuminance": config.lighting.camera_illuminance,
            "enable_fog": config.lighting.enable_fog,
        },
        "render": {
            "show_cell": config.render.show_cell,
            "show_axes": config.render.show_axes,
            "show_ui": config.render.show_ui,
            "ico_subdiv": config.render.ico_subdiv,
        },
        "initial_frame": config.initial_frame,
    }


def deserialize_config(payload: Optional[dict]) -> Optional[ViewerConfig]:
    if payload is None:
        return None

    return ViewerConfig(
        color=ColorConfig(
            background=payload["color"]["background"],
            cell_color=payload["color"]["cell_color"],
        ),
        lighting=LightingConfig(**payload["lighting"]),
        render=RenderConfig(**payload["render"]),
        initial_frame=payload["initial_frame"],
    )
