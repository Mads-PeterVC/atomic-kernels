# Viewer Scripts

This page collects the larger script-based viewer workflows separately from the
distilled documentation examples.

## Focused feature scripts

- [`ball_and_stick_selection.py`](/Users/au616397/Repositories/atomic-kernels/scripts/ball_and_stick_selection.py):
  selection-scoped ball-and-stick with explicit bonds
- [`polyhedra_faces.py`](/Users/au616397/Repositories/atomic-kernels/scripts/polyhedra_faces.py):
  explicit face overlays for a tetrahedral cluster
- [`polyhedra_minimal.py`](/Users/au616397/Repositories/atomic-kernels/scripts/polyhedra_minimal.py):
  minimal auto-polyhedra example from an `Atoms` object
- [`property_coloring.py`](/Users/au616397/Repositories/atomic-kernels/scripts/property_coloring.py):
  scalar coloring and palette changes
- [`camera_controls.py`](/Users/au616397/Repositories/atomic-kernels/scripts/camera_controls.py):
  camera framing, orbit, pan, zoom, and look-at helpers
- [`live_viewer_session.py`](/Users/au616397/Repositories/atomic-kernels/scripts/live_viewer_session.py):
  appending frames and tail-following
- [`viewer_config.py`](/Users/au616397/Repositories/atomic-kernels/scripts/viewer_config.py):
  viewer appearance configuration

## Compound workflow scripts

- [`slab_adsorbate_ball_and_stick.py`](/Users/au616397/Repositories/atomic-kernels/scripts/slab_adsorbate_ball_and_stick.py):
  slab setup, adsorbate selection, explicit bonds, and styling in one workflow
- [`random_alloy_coloring.py`](/Users/au616397/Repositories/atomic-kernels/scripts/random_alloy_coloring.py):
  mixed-species coloring, explicit bonds, and selection-specific render layers
- [`emt_relaxation_coloring.py`](/Users/au616397/Repositories/atomic-kernels/scripts/emt_relaxation_coloring.py):
  live trajectory updates during relaxation with consistent scalar ranges
- [`viewer.py`](/Users/au616397/Repositories/atomic-kernels/scripts/viewer.py):
  larger viewer demo
- [`viewer_config-v2.py`](/Users/au616397/Repositories/atomic-kernels/scripts/viewer_config-v2.py):
  extended viewer configuration examples

## Running a script

```bash
uv run python scripts/slab_adsorbate_ball_and_stick.py
```
