"""
Example demonstrating ViewerConfig usage with atomic-kernels viewer.

This shows how to customize the viewer appearance using the configuration classes.
"""

from ase.build import bulk
from atomic_kernels import ViewerConfig, LightingConfig, ColorConfig, RenderConfig
from atomic_kernels.viewer import bevy_viewer

# Build a simple structure
atoms = bulk('Cu', 'fcc', a=3.615).repeat((3, 3, 3))

# Example 1: Use default configuration
print("Example 1: Default configuration")
# bevy_viewer(atoms)  # Uncomment to run

# Example 2: Dark theme with custom lighting
print("\nExample 2: Dark theme")
dark_config = ViewerConfig(
    color=ColorConfig(
        background=(0.1, 0.1, 0.1),  # Dark background
        cell_color=(0.8, 0.8, 0.8)   # Light gray cell
    ),
    lighting=LightingConfig(
        ambient_brightness=150.0,
        fill_illuminance=0.0,        
        key_illuminance=0.0,    
        enable_fog=True
    ),
    render=RenderConfig(
        show_cell=True,
        show_axes=True,
        show_ui=True  # Enable UI
    )
)
bevy_viewer(atoms, config=dark_config)  # Uncomment to run

# Example 3: Modify configuration step by step
print("\nExample 3: Step-by-step modification")
config = ViewerConfig()

# Get nested config, modify, and reassign
color = config.color
color.background = (0.95, 0.95, 1.0)  # Light blue background
config.color = color

lighting = config.lighting
lighting.ambient_brightness = 200.0
config.lighting = lighting

render = config.render
render.show_ui = True
config.render = render

# bevy_viewer(atoms, config=config)  # Uncomment to run

print("\n✓ All examples configured successfully")
print("  Uncomment the bevy_viewer() calls to run the viewer")
