# Agentic Development Notes

This page is the running log for agent-driven development updates. Follow the workflow
and format defined in
[`docs/src/agent-development.md`](/Users/au616397/Repositories/atomic-kernels/docs/src/agent-development.md).

## Existing notes

Entries are listed newest first.

## 2026-03-11 - Branch-scoped CI cache proof of concept

- Commits: `662b848`, `4c02ab9`, `ea9e247`, `5517065`, `f93a691`, `dadb642`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The repository needed a low-risk way to start exercising the Rust workspace in
  GitHub Actions without paying a full cold compile cost on every run. The goal was to
  validate a branch-only CI shape first, then extend it later into the main workflow
  once caching behavior and test environment constraints were better understood.
- Implementation: Added `.github/workflows/rust-build-poc.yml` scoped to the
  `codex/cache-ci` branch, plus the reusable
  `.github/actions/install-linux-build-deps/action.yml` composite action so Linux build
  packages are defined once and reused across jobs. The workflow now uses
  `Swatinem/rust-cache@v2` with a shared cache key for a Rust build job, a Rust test
  job, and a Python integration job that syncs dependencies with
  `uv sync --group test --no-install-project`, installs `maturin`, runs
  `maturin develop`, and then executes the pytest suite against the real extension. The
  `target`-artifact experiment was tried and then removed after measuring that upload
  and download time was worse than relying on the Rust cache alone. Supporting fixes
  also made the Rust `xyz` parser test self-contained in
  `crates/ak-core/src/io/xyz.rs`.
- Difficulty: The hard part was not wiring the action syntax but sorting out which
  reuse layer was actually worth keeping. A naive "build once, upload `target`, reuse it
  everywhere" design looked clean on paper but was slower in practice. The branch also
  exposed that headless Bevy rendering remains flaky in CI even when compilation and
  dependency caching work, so several iterations were spent separating cache behavior
  from runtime/test-environment failures.
- Constraints: This workflow is intentionally a branch-only proof of concept and should
  not be treated as the final production CI shape yet. The current note only records
  the caching and job-structure decision; headless viewer stability in CI is still an
  open problem, and ALSA/audio-related stderr noise was not fully eliminated by simply
  disabling Bevy audio.
- Follow-up: Once the headless viewer behavior is settled, add a separate note for the
  CI/runtime constraints discovered there and then decide how much of
  `rust-build-poc.yml` should migrate into the real CI workflow. If Python jobs expand,
  preserve the `uv sync --no-install-project` plus explicit `maturin develop` pattern
  so dependency installation and local package build remain distinct.

## 2026-03-10 - Documentation build workflow in GitHub Actions

- Commits: `e3ad597`, `adf8e27`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The documentation setup needed CI coverage so docs configuration and content
  changes are validated automatically instead of only when someone runs the site
  locally.
- Implementation: Added `.github/workflows/docs.yml` to install the docs-only Python
  dependency group with `uv`, run `zensical build --config-file docs/zensical.toml`,
  upload a GitHub Pages artifact, and
  deploy the built site through a dedicated Pages job on pushes to `development`. The
  job is scoped to docs-related path changes and uses `--no-install-project` so the
  Python package itself is not installed.
- Difficulty: The main point of care was avoiding an accidental Rust build. A naive
  `uv sync --group docs` would still install the local `maturin` project, which can
  trigger compilation of the Rust extension even though the current docs are pure
  Markdown. After that, the workflow also needed the GitHub Pages-specific artifact and
  deploy actions rather than only uploading a generic CI artifact.
- Constraints: This workflow validates the static docs site only. It does not exercise
  the Rust crates, Python bindings, or any future docs feature that imports the local
  package during site generation. Deployment is currently tied to pushes on the
  `development` branch.
- Follow-up: If the docs later gain generated API pages or other build-time imports of
  `atomic-kernels`, revisit the workflow and decide whether a separate heavier docs CI
  job is justified. If the repository’s publishing branch changes, update the deploy
  condition to match it.

## 2026-03-10 - Documentation system introduced

- Commit: `TBD`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The repository had reached a size where architecture and workflow knowledge
  was no longer recoverable from source layout alone, and the top-level README did not
  provide a usable entry point.
- Implementation: Added a Markdown-first docs site via `Zensical`, introduced
  `docs/src/index.md`, `docs/src/getting-started.md`,
  `docs/src/architecture.md`, and the agent development pages, and linked the docs
  entry points from the repository README.
- Difficulty: The setup itself was straightforward, but it exposed that project-level
  context had been spread across source layout, scripts, and recent memory rather than
  in durable documentation.
- Constraints: This is intentionally light on generated API reference for now. The
  priority is maintainable engineering documentation, not fully automated reference
  extraction.
- Follow-up: Replace `TBD` with the actual feature commit hash once the docs workflow is
  being followed from the start, and append future entries with concrete commit ids.

## 2026-03-10 - Selection-aware ball-and-stick rendering

- Commit: `7d5cd23`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The Python viewer API needed a second rendering mode beyond space-filling so
  scripts could highlight subsets such as adsorbates, alloy species, or coordination
  environments without replacing the full-scene representation.
- Implementation: Added bond and render-style domain types in
  `crates/ak-vis/src/viewer/session.rs`, bond rendering in
  `crates/ak-vis/src/render/render_bonds.rs` and
  `crates/ak-vis/src/visuals/bond_visual.rs`, and a Python render controller in
  `python/atomic_kernels/viewer/_render.py` wired through the PyO3 bindings in
  `crates/ak-py/src/pyfunctions/py_viewer.rs`. Example usage lives in
  `scripts/ball_and_stick_selection.py` and
  `scripts/slab_adsorbate_ball_and_stick.py`.
- Difficulty: The useful abstraction was not "ball-and-stick for the whole frame" but
  ordered selection-scoped style rules. That required explicit bond storage,
  bond-scope semantics (`both_selected` vs `touch_selection`), and keeping the viewer
  agnostic to bond discovery so ASE or other Python-side tooling can own connectivity.
- Constraints: The viewer consumes canonical edge pairs, not bond heuristics or
  adjacency matrices directly. Python may normalize adjacency input for convenience,
  but Rust-side state should stay typed and validated. v1 bonds use a single neutral
  bond color and only support ball-and-stick overlays on top of the existing atom
  renderer.
- Follow-up: Document a recommended bond-generation workflow for scripts, consider a
  higher-level helper for common ASE neighbor-list conversions, and decide whether
  endpoint-colored bonds or additional render styles are worth adding.

## 2026-03-10 - Structure-space camera semantics and orientation widget

- Commit: `acf843f`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: Camera control and orientation cues initially felt wrong for chemistry
  workflows because the viewer internals followed Bevy/world conventions while scripts
  and structures assume structure-space `z` is up.
- Implementation: Added structure-space camera conversion in
  `python/atomic_kernels/viewer/_camera.py` and the matching Rust-side world transform
  export in `crates/ak-vis/src/visuals/convert.rs`. Added a viewport-fixed orientation
  widget in `crates/ak-vis/src/viewer/orientation_widget.rs`, wired through
  `crates/ak-vis/src/viewer/app.rs` and configured from
  `crates/ak-vis/src/viewer/config.rs` plus the Python config wrappers.
- Difficulty: The widget took several iterations because the second camera rendered
  correctly while the render-layer and viewport behavior were misleading in practice.
  The final labels use mesh geometry rather than Bevy text/UI because the geometry path
  was the reliable one inside the overlay pass.
- Constraints: The widget is a viewer config feature, not a live Python controller
  feature. It represents structure-space axes, should stay anchored to the lower-left
  viewport corner, and currently uses mesh-stroke letters that face the camera rather
  than dynamic text rendering.
- Follow-up: If the camera still feels constrained after the coordinate-system fix, the
  next investigation should be whether `PanOrbitCamera` is the right long-term camera
  backend. The orientation widget may also need future polish around DPI scaling and
  theming.

## 2026-03-10 - Python test harness and viewer readiness handshake

- Commit: `1d65f52`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The Python package had wrapper code for neighbor-list utilities and live
  viewer control, but almost no automated coverage and no reliable way to assert that a
  spawned Bevy viewer session had actually reached a usable state.
- Implementation: Added a pytest dependency group and marker configuration in
  `pyproject.toml`, a stub-backed `tests/` suite for pure-Python viewer helpers and
  session facades, and `just test` / `just viewer-test` entry points via `justfile` and
  the README. Added `wait_until_ready()` through the Rust viewer session handle in
  `crates/ak-vis/src/viewer/session.rs`, signaled readiness from the Bevy app loop in
  `crates/ak-vis/src/viewer/app.rs`, exposed it through the PyO3 bindings in
  `crates/ak-py/src/pyfunctions/py_viewer.rs`, and bridged it on macOS through the
  subprocess proxy in `python/atomic_kernels/viewer/_process.py`.
- Difficulty: The first macOS approach tried to send readiness as an out-of-band message
  over the same multiprocessing pipe used for viewer commands, which was race-prone and
  failed even when the viewer itself launched correctly. The stable design was to make
  readiness an explicit request/response command and to delay the Bevy-side ready signal
  until the app had entered its update loop with a primary window available.
- Constraints: The default Python tests still run against a stubbed
  `atomic_kernels._atomic_kernels` module and intentionally avoid launching the real
  viewer. The real GUI smoke test is opt-in behind the `viewer_integration` marker and
  `ATOMIC_KERNELS_RUN_VIEWER_TESTS=1`, because it depends on a usable display
  environment and the compiled extension.
- Follow-up: If viewer integration tests become part of CI, give them a dedicated job
  with explicit display/runtime support rather than folding them into the default Python
  test path. Consider whether future viewer lifecycle checks should distinguish between
  "window created" and "first frame rendered" if startup assertions need to become
  stricter.

## 2026-03-10 - Explicit face overlays for polyhedra-style viewer rendering

- Commit: `002fdf3`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The viewer already supported explicit bonds plus selection-scoped
  ball-and-stick styling, but that abstraction did not extend cleanly to
  polyhedra-style surfaces because faces are explicit scene elements rather than a
  render style on top of atoms.
- Implementation: Added face domain types and per-frame storage in
  `crates/ak-vis/src/viewer/session.rs`, a dedicated face render path in
  `crates/ak-vis/src/render/render_faces.rs` plus
  `crates/ak-vis/src/visuals/face_visual.rs`, and Python-side normalization and
  bindings in `python/atomic_kernels/viewer/_render.py`,
  `python/atomic_kernels/viewer/_utils.py`, `python/atomic_kernels/viewer/_process.py`,
  and `crates/ak-py/src/pyfunctions/py_viewer.rs`. Example usage lives in
  `scripts/polyhedra_faces.py`.
- Difficulty: The main design choice was resisting the temptation to force faces into the
  existing `RenderStyleRule` machinery. That worked for ball-and-stick because bonds and
  atoms already existed as scene data, but it would have made polyhedra semantics
  selection-driven and ambiguous. The stable split was explicit per-frame face topology
  in Rust with permissive Python normalization and fan triangulation only at render
  time.
- Constraints: v1 faces are ordered polygons with 3 or more distinct atom indices and
  per-face RGBA colors. Rust validates and stores explicit faces but does not infer
  polygon order, convex hulls, or neighbor-derived polyhedra. Rendering assumes planar,
  convex-enough polygons for triangle-fan triangulation and draws translucent filled
  faces only, without outline edges.
- Follow-up: Add higher-level Python helpers for generating polyhedra faces from common
  chemistry inputs such as neighbor lists or coordination environments, and run a live
  viewer smoke check once a representative polyhedron script set exists beyond the
  synthetic tetrahedral example.

## 2026-03-10 - Windowless headless viewer rendering

- Commit: `6a9e216`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The viewer needed a fully windowless render path both for non-interactive
  scripting workflows and for CI coverage that exercises the real Bevy scene/render
  stack without relying on a display server or window screenshot hooks.
- Implementation: Split shared viewer bootstrap/state setup into
  `crates/ak-vis/src/viewer/runtime.rs`, kept window-specific behavior in
  `crates/ak-vis/src/viewer/app.rs`, and added the offscreen export pipeline in
  `crates/ak-vis/src/viewer/headless.rs` plus the Rust example in
  `crates/ak-vis/examples/headless_scene.rs`. Exposed the feature through PyO3 in
  `crates/ak-py/src/pyfunctions/py_viewer.rs` and the Python session facade in
  `python/atomic_kernels/viewer/_session.py` and
  `python/atomic_kernels/viewer/__init__.py`, with the scriptable demo in
  `scripts/headless_render.py` and real integration coverage in
  `tests/test_headless_render.py`.
- Difficulty: Several iterations were needed before camera scripting behaved like the
  interactive viewer. Headless sessions queue commands before `save()`, so camera state
  was initially being lost during app startup and `PanOrbitCamera` initialization. The
  final fix was to apply queued commands before inserting the Bevy resources, drive only
  the plugin `target_*` fields after initialization, and delay capture until after the
  camera/transform update path had produced a fresh rendered frame. That split also
  clarified an important future-web constraint: script commands must be expressible as
  durable viewer state before a concrete native window or render surface exists, because
  a browser/WebGPU or WebGL backend will likely need the same "prepare state first,
  attach surface later" behavior when canvas lifecycle and async device creation are not
  under direct Rust control.
- Constraints: The public Python workflow is now `headless_viewer_session(...).save()`;
  the one-shot `render_image()` helper was intentionally removed because it did not offer
  a better long-term path for scripted sequences. The CI job definition was added but
  explicitly disabled pending environment configuration, and local Rust tests still skip
  gracefully when no GPU/backend is available. The current offscreen implementation is
  still native-oriented: it depends on Bevy render-graph image readback, filesystem PNG
  output, and host-side GPU polling. Those choices are acceptable for CI and local batch
  rendering but should not be treated as the eventual abstraction boundary for a browser
  target. For a future WebGPU/WebGL backend, the reusable layer is the shared
  `ViewerState`/`CameraState` plus the scene-construction systems in
  `crates/ak-vis/src/viewer/runtime.rs` and `crates/ak-vis/src/viewer/systems.rs`; the
  replaceable layer is the runner/bootstrap code in `app.rs` and `headless.rs`, because
  browser canvas ownership, async adapter/device acquisition, and image export/download
  semantics differ materially from native winit/offscreen flows. In particular, avoid
  coupling higher-level Python or Rust scripting APIs to native-only concepts like
  `ScheduleRunnerPlugin`, filesystem output paths as the only sink, or "device exists at
  app construction time" assumptions.
- Follow-up: When sequence rendering becomes a priority, build it on top of a persistent
  headless session/app rather than reusing the current one-shot export path per frame.
  Re-enable the CI job once the software-rendering environment is settled, and consider
  adding an image-difference assertion on top of the existing camera regression test if
  byte inequality proves too weak. If a WebGPU/WebGL viewer backend is pursued, keep the
  current direction of travel: define backend-neutral viewer/session commands and shared
  scene systems first, then build separate native-window, native-headless, and browser
  runners around them. Do not reuse the current native headless image-readback path as a
  proxy for the browser design; instead, treat it as evidence that the state/systems
  split is useful and that future backend work should preserve that split while swapping
  out surface creation, frame scheduling, and image delivery.
