# Viewer Configuration

Use `ViewerConfig` when you want to change the default viewer appearance before opening
the window.

## Configure the viewer up front

Pass a `ViewerConfig` into `bevy_viewer(...)` or `viewer_session(...)` to control colors,
lighting, and render toggles.

```python
from ase.build import bulk

from atomic_kernels import ColorConfig, LightingConfig, RenderConfig, ViewerConfig
from atomic_kernels.viewer import bevy_viewer

atoms = bulk("Cu", "fcc", a=3.615).repeat((3, 3, 3))

config = ViewerConfig(
    color=ColorConfig(
        background=(0.1, 0.1, 0.1),
        cell_color=(0.8, 0.8, 0.8),
    ),
    lighting=LightingConfig(
        ambient_brightness=150.0,
        fill_illuminance=0.0,
        key_illuminance=0.0,
        enable_fog=True,
    ),
    render=RenderConfig(
        show_cell=True,
        show_axes=True,
        show_ui=True,
    ),
)

bevy_viewer(atoms, config=config)
```

## Modify a config incrementally

The nested config objects can also be updated step by step before launching the viewer.

```python
from atomic_kernels import ViewerConfig

config = ViewerConfig()

color = config.color
color.background = (0.95, 0.95, 1.0)
config.color = color

lighting = config.lighting
lighting.ambient_brightness = 200.0
config.lighting = lighting

render = config.render
render.show_ui = True
config.render = render
```

This pattern is useful when the starting point is mostly default settings and only a
few fields need to change.
