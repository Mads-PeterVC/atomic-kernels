# ak-vis Developer Guide

`ak-vis` is the Bevy-backed visualization crate for `atomic-kernels`. It takes
`ak-core` structures and trajectories, turns them into renderable visual
descriptions, and runs them in an interactive, scriptable, or headless viewer.

This README is for developers and curious readers who want to know where to
look before changing code.

## Quick Map

| Path | Purpose |
| --- | --- |
| `src/lib.rs` | Public crate surface. Re-exports the viewer, visuals, color palettes, and selected render helpers. |
| `src/viewer.rs` | Viewer facade. Keeps the public viewer API stable while delegating to smaller modules. |
| `src/viewer/app.rs` | Builds and runs the interactive Bevy app, including plugins, UI, controls, picking, and session lifecycle. |
| `src/viewer/runtime.rs` | Shared app setup used by interactive, scripted, wasm, and headless paths. Installs core resources and systems. |
| `src/viewer/config.rs` | Viewer, render, color, and lighting configuration structs. |
| `src/viewer/session/` | Scriptable viewer state, commands, selections, bonds, faces, camera state, snapshots, and session handles. |
| `src/viewer/systems/` | Bevy ECS systems for rendering, command application, selection, camera/light sync, and snapshot sync. |
| `src/viewer/controls/` | Keyboard, camera, navigation, screenshot, and UI visibility controls. |
| `src/viewer/headless/` | Off-screen rendering and PNG export internals. |
| `src/viewer/orientation_widget.rs` | Orientation widget setup and synchronization. |
| `src/viewer/picking.rs` | Picking-related helpers and public picking surface. |
| `src/visuals/` | Visual data types and conversion from structures/cells/axes into renderable descriptions. |
| `src/render/` | Bevy entity spawning from visual descriptions. |
| `src/color_palette/` | Atom palettes, material defaults, and scalar color maps. |
| `src/ui/` | Inspector, playback panel, shortcut hints, and UI state synchronization. |
| `src/components.rs` | Bevy marker components used to identify viewer entities. |
| `examples/` | Small executable examples for interactive, configured, trajectory, relaxation, headless, and wasm usage. |
| `tests/` | Public-surface and render-helper coverage. |

## How The Viewer Works

The usual input is an `ak_core::Structure` or `ak_core::Trajectory`. Public entry
points such as `run`, `launch`, `run_with_session`, and the headless export
functions are re-exported from `src/lib.rs` and implemented under `src/viewer/`.

Interactive and scripted viewers start in `viewer::app`. That module creates a
Bevy `App`, adds Bevy plugins, registers fonts, installs input/picking/UI
systems, and delegates shared viewer setup to `viewer::runtime::configure_shared_app`.

`configure_shared_app` creates the core resources:

- `ViewerState`: trajectory, current frame, scalar fields, appearance rules,
  bonds, faces, selections, supercell settings, and render dirty flags.
- `CameraState`: camera focus, radius, yaw/pitch, pending apply flag, and
  optional camera motion.
- `ViewerConfig`: user-facing render, color, window, and lighting options.
- `CommandReceiver`: optional channel used by scriptable sessions.
- `SharedViewerSnapshot`: state mirrored back to `ViewerSessionHandle`.

At startup, `viewer::systems::rendering::render_current_frame` renders the first
frame. On later updates, `viewer::systems::commands::apply_viewer_commands`
drains queued `ViewerCommand`s, applies camera commands to `CameraState`, applies
state commands to `ViewerState`, and marks the viewer dirty when needed.
`viewer::systems::rendering::rerender_if_dirty` then despawns frame entities and
rebuilds the scene for the current state.

Rendering is split into two layers:

- `visuals` computes plain visual descriptions such as atoms, bonds, cells,
  axes, and faces.
- `render` turns those visual descriptions into Bevy meshes/materials/entities.

The UI and controls do not bypass this model. Playback, keyboard navigation,
marquee selection, atom picking, screenshot handling, scripted sessions, and
headless export all ultimately update the same `ViewerState`/`CameraState` and
use the same render systems.

## Where To Look

| Task | Start here |
| --- | --- |
| Add or change public API | `src/lib.rs`, `src/viewer.rs`, then the relevant facade/module. |
| Add a new viewer command | `src/viewer/session/command.rs`, `src/viewer/session/handle.rs`, `src/viewer/session/state_commands.rs`. |
| Change persistent viewer state | `src/viewer/session/state.rs`, plus the focused session modules for selections, bonds, faces, appearance, or snapshots. |
| Change camera behavior | `src/viewer/session/camera.rs`, `src/viewer/systems/camera.rs`, `src/viewer/controls/camera.rs`. |
| Change frame rendering | `src/viewer/systems/rendering.rs`, then `src/visuals/` or `src/render/` depending on whether the change is data conversion or Bevy spawning. |
| Change atom colors/materials | `src/color_palette/`, `src/viewer/session/appearance.rs`, `src/viewer/systems/rendering.rs`. |
| Change selection behavior | `src/viewer/session/selection.rs`, `src/viewer/systems/selection.rs`, and selection-related tests under `src/viewer/session/tests/`. |
| Change bonds or faces | `src/viewer/session/bonds.rs`, `src/viewer/session/faces.rs`, `src/viewer/systems/rendering.rs`, `src/render/`. |
| Change UI panels or text | `src/ui/`, especially `build_inspector.rs`, `build_playback_panel.rs`, `state.rs`, and `playback/`. |
| Change keyboard/navigation controls | `src/viewer/controls/`. |
| Change headless image export | `src/viewer/headless.rs` and `src/viewer/headless/`. |
| Change wasm canvas behavior | `src/viewer/app.rs` and `ViewerAppOptions`; also check the `ak-wasm` crate. |

## Module Boundary Notes

The crate intentionally keeps a few facade modules:

- `src/lib.rs` is the crate-level public surface.
- `src/viewer.rs` is the viewer public surface.
- `src/viewer/session.rs`, `src/viewer/systems.rs`, and
  `src/viewer/headless.rs` collect smaller implementation modules and re-export
  the items callers need.

Prefer preserving those facades when adding code. That keeps external imports
stable while allowing the internals to stay split by responsibility.

As a rule of thumb:

- Put pure visual data and coordinate conversion in `visuals`.
- Put Bevy entity spawning from visual data in `render`.
- Put durable viewer/session state in `viewer/session`.
- Put Bevy ECS systems in `viewer/systems`.
- Put user input handling in `viewer/controls` or `viewer/systems/selection`.
- Put UI layout and UI state synchronization in `ui`.

Avoid making `visuals` depend on viewer state or UI concepts. Avoid making
public exports from deep implementation modules unless they are intentionally
part of the crate API and re-exported through `lib.rs` or `viewer.rs`.

## Examples

| Example | Demonstrates |
| --- | --- |
| `examples/basic_scene.rs` | Minimal interactive viewer launch. |
| `examples/config_scene.rs` | Viewer configuration such as colors, lighting, and render options. |
| `examples/trajectory_scene.rs` | Multi-frame trajectory viewing. |
| `examples/relaxation_scene.rs` | Relaxation trajectory playback using bundled example data. |
| `examples/headless_scene.rs` | Off-screen export to an image file. |
| `examples/wasm_scene.rs` | Setup intended for wasm/browser embedding. |

Most examples use data from `examples/xyz/`.

## Development Commands

Useful checks while working on this crate:

```sh
cargo test -p ak-vis --lib
cargo test -p ak-vis --tests
cargo check -p ak-vis --examples
```

When changing public API or behavior used by downstream crates, also run:

```sh
cargo check -p ak-viewer-py
cargo check -p ak-wasm
```

