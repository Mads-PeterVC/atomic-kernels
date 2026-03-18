# Agentic Development Features

This page tracks notable feature ideas that should shape future agent-driven work.
It is intentionally forward-looking, unlike
[`docs/src/agent-development-notes.md`](/Users/au616397/Repositories/atomic-kernels/docs/src/agent-development-notes.md),
which records completed implementation work.

## Candidate features

### Web rendering support

Add a browser-capable rendering path for structure visualization, likely through
WebGL or WebGPU.

- Decide whether this should be a separate frontend target, a portability layer over
  the current viewer stack, or a deliberately smaller web-specific viewer.
- Define the minimum supported feature set for a first version rather than assuming the
  desktop viewer should map over directly.

### Rust and Python API documentation

~~Add proper API reference documentation for both the Rust crates and the Python package.~~

- ~~Rust docs should make crate boundaries and intended public entry points clearer.~~
- ~~Python docs should cover the exposed viewer, structure, and workflow APIs with
  examples where signatures alone would be too thin.~~
- ~~If generated docs are introduced, keep the docs build explicit about whether it must
  build the local extension module.~~

Progress

- Feature completed in `9a78507`.

### Viewer UI feature expansion

Expand the interactive viewer UI beyond the current camera and rendering controls.

- ~~Add distance and angle measurement tools for common inspection workflows.~~
- Show which image in a trajectory is currently displayed.
- Expect additional viewer affordances to emerge once selection and measurement exist,
  so this area should be treated as a broader UI roadmap rather than a closed list.

Progress

- Shared live atom selection and Shift-drag marquee selection landed in `bd816a4`.
- A structured selection/measurement inspector, keybindings panel, 2-atom distance
  readout, 3-atom angle readout, 3-atom in-scene angle cue, and the follow-up UI
  module split landed in `fb94de0`, `8e2d2f7`, `fd041e5`, and `24496e7`.
- The next clean slice in this area is a dedicated trajectory status/playback surface,
  since the current inspector intentionally stays focused on selection and measurement.

### UI-enabled viewer input smoke test

~~Add one real viewer integration smoke test that proves atom click-picking still works
when the UI is enabled.~~

- ~~The test should run in the existing Xvfb-based viewer integration lane rather than
  relying only on in-process unit tests.~~
- ~~The scene setup should be reusable and manually launchable so the click target can
  be inspected visually before CI green is trusted.~~

Progress

- Feature completed in `294787b`.

### Interactive picking and selection state

~~Add a proper viewer-side picking and selection model that is shared between the UI
and the Python API.~~

- ~~Picking should identify atoms in the live viewer and expose that state in a durable
  way rather than only as a transient visual highlight.~~
- ~~Python should be able to query, replace, extend, and clear the current
  selection.~~
- ~~The design should support future selection-driven workflows such as measurements,
  annotations, and subset-specific rendering without duplicating selection logic across
  the Rust viewer and Python facade layers.~~

Progress

- Feature completed in `bd816a4`.

### Supercell visualization and selection semantics

~~Add the ability to display repeated periodic images while preserving a clear notion of
the original cell.~~

- ~~Users should be able to repeat the structure in one or more periodic directions.~~
- ~~The viewer should visually distinguish atoms in the main cell from atoms shown only
  because of repetition.~~
- ~~Python-side selection should remain convenient for both the main cell and repeated
  images instead of flattening everything into a single ambiguous atom set.~~
- ~~The design should avoid forcing the displayed supercell to replace the canonical
  original-cell representation in the API.~~

Progress

- Feature completed in `c24f53c`.
- The current viewer keeps initial framing and `X`/`Y`/`Z` snap views cell-based even
  when repeated images are displayed, so future camera work should treat supercell
  display fit and default/snap framing as separate decisions rather than one shared
  camera policy.

### Higher-level chemistry helpers for bonds and polyhedra

Add higher-level Python helpers that derive viewer-ready topology and overlays from
common chemistry inputs.

- Bond generation should be easy to drive from neighbor lists, ASE inputs, or other
  common structure-analysis results.
- Polyhedra helpers should derive face definitions from coordination environments or
  similar chemistry concepts instead of requiring users to enumerate polygon faces by
  hand.
- These helpers should remain optional conveniences on top of the explicit Rust-side
  bond and face data model rather than weakening the typed scene representation.

### Persistent headless sequence rendering

Extend the headless viewer so it can render sequences efficiently from a persistent
session rather than only one-shot images.

- Batch rendering should reuse a live app/session state instead of rebuilding the whole
  viewer for every output frame.
- The API should support scripted trajectory playback, camera motion, and repeated image
  export for animations or datasets.
- This should be treated as a native and automation-friendly feature first, while
  keeping the command/state model compatible with future web backends.

### CI-built wheels for PyPI publishing

Extend CI so it builds distributable wheels that can be published to PyPI as
precompiled packages.

- The workflow should produce wheels for the target platforms worth supporting rather
  than assuming source builds are acceptable for all users.
- Artifact production and release/publish steps should be designed deliberately on top
  of the current validation-focused CI rather than by reviving the old autogenerated
  release matrix wholesale.
- The publishing flow should make it straightforward to ship precompiled releases to
  PyPI once versioning and release triggers are defined.

Progress

- Wheels for MacOS Arm, MacOS x86 and Many linux added in ``01b7d6f`, `b84b0a4`, `3ffc69b`, `87c72c8`, `e164e0f`, `5c55fa2`,`9b5929a`. 

### Dedicated viewer integration CI

~~Add a dedicated CI lane for real viewer and headless-render integration coverage with
explicit runtime assumptions.~~

- ~~GUI and headless viewer tests should not be folded blindly into the default unit-test
  path because they depend on graphics/runtime details that differ from normal library
  tests.~~
- ~~The job should define and document the display, renderer, or software-rendering
  environment it expects so failures are actionable rather than flaky.~~
- ~~Once this exists, it should cover the real viewer lifecycle more directly than the
  current stub-heavy Python tests.~~

Progress

- Feature completed in `e043738`, `56b64dd`, `5c07633`, `bf15327`, `9a9a986`,
  `2221f78`, `9d6b5da`.

### Camera tuning as public configuration

Expose camera behavior more deliberately through user-facing configuration and API
controls.

- Camera smoothing and related interaction feel should not remain buried in hard-coded
  viewer defaults.
- Python and Rust users should be able to configure relevant camera behavior without
  patching internal viewer setup.
- The feature should clarify which camera semantics are stable public API versus which
  remain implementation details of the current backend.

### Trajectory playback controls

Add trajectory playback controls as a first-class viewer feature rather than only basic
frame switching.

- The viewer UI should support play/pause, frame stepping, and playback speed control.
- Live and appended trajectories should integrate cleanly with follow-tail behavior.
- Playback state should be consistent with any trajectory index indicator so users can
  tell both where they are and how the viewer is advancing through frames.
