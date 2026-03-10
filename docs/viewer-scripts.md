# Viewer Scripts

This page collects the larger script-based viewer workflows separately from the
distilled documentation examples.

## Focused feature scripts

- [`py_ball_and_stick_selection.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_ball_and_stick_selection.py):
  selection-scoped ball-and-stick with explicit bonds
- [`py_polyhedra_faces.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_polyhedra_faces.py):
  explicit face overlays for a tetrahedral cluster
- [`py_property_coloring.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_property_coloring.py):
  scalar coloring and palette changes
- [`py_camera_controls.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_camera_controls.py):
  camera framing, orbit, pan, zoom, and look-at helpers
- [`py_live_viewer_session.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_live_viewer_session.py):
  appending frames and tail-following
- [`py_viewer_config.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_viewer_config.py):
  viewer appearance configuration

## Compound workflow scripts

- [`py_slab_adsorbate_ball_and_stick.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_slab_adsorbate_ball_and_stick.py):
  slab setup, adsorbate selection, explicit bonds, and styling in one workflow
- [`py_random_alloy_coloring.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_random_alloy_coloring.py):
  mixed-species coloring, explicit bonds, and selection-specific render layers
- [`py_emt_relaxation_coloring.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_emt_relaxation_coloring.py):
  live trajectory updates during relaxation with consistent scalar ranges
- [`py_viewer.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_viewer.py):
  larger viewer demo
- [`py_viewer_config-v2.py`](/Users/au616397/Repositories/atomic-kernels/scripts/py_viewer_config-v2.py):
  extended viewer configuration examples

## Running a script

```bash
uv run python scripts/py_slab_adsorbate_ball_and_stick.py
```
