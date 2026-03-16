# Agentic Development Notes

This page is the running log for agent-driven development updates. Follow the workflow
and format defined in
[`docs/src/agent-development.md`](/Users/au616397/Repositories/atomic-kernels/docs/src/agent-development.md).

## Existing notes

Entries are listed newest first.

## 2026-03-16 - Apple Silicon wheel release workflow added

- Commit: `01b7d6f`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The Python package only had source-build instructions even though the repo
  already used `maturin` and had a clear need for distributable wheels. This change
  adds a first release-oriented CI path so users on Apple Silicon can install a
  prebuilt wheel from GitHub without waiting for PyPI publishing.
- Implementation: Added `.github/workflows/python-wheel.yml` as a dedicated
  `macos-14` wheel workflow that builds exactly one CPython 3.12 arm64 wheel with
  `maturin`, verifies the filename tags, smoke-installs the wheel into a clean venv,
  uploads it as a workflow artifact, and publishes it to a GitHub Release on tag
  pushes. Updated `README.md` and `docs/src/getting-started.md` to document the new
  GitHub Release install path and the intentionally narrow v1 platform support.
- Difficulty: The main friction was shaping the workflow around release artifacts
  rather than normal validation CI. The repo already had a Linux-centered test
  workflow, so the new job needed to stay separate, prove the wheel was actually
  installable before upload, and support both manual iteration and future tag-driven
  publishing without dragging wheel-release concerns into PR CI.
- Constraints: This first pass is intentionally limited to macOS Apple Silicon and
  CPython 3.12, with GitHub Releases as the only distribution channel. It does not add
  PyPI publishing, an sdist, Intel macOS, Linux, or Windows wheel coverage.
- Follow-up: If wheel distribution becomes part of normal releases, decide whether to
  broaden the Python/platform matrix, formalize the tag naming convention in docs, and
  add PyPI publishing as a separate release step instead of expanding this workflow ad
  hoc.

## 2026-03-12 - Python and Rust API documentation added to docs site

- Commit: `9a78507`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The docs site had an explicit feature placeholder for Rust and Python API
  reference pages, but users still had to infer public entry points from source files
  and local tooling. This change adds first-class API navigation to the docs so the
  Python facade and Rust crate surfaces are reachable from the published site.
- Implementation: Added new API pages under `docs/src/api/` and split the Python
  reference into focused pages for neighbor lists, viewer launch helpers, session
  facades, and controller classes. Enabled `mkdocstrings-python` in
  `docs/zensical.toml` and `pyproject.toml`, added docstrings in
  `python/atomic_kernels/neighbor_list.py` and `python/atomic_kernels/viewer/__init__.py`
  to improve generated output, and updated the docs nav to separate `Python API` and
  `Rust API` sections. For Rust, added `scripts/stage_rustdoc.py`, updated `justfile`,
  and extended `.github/workflows/docs.yml` so `cargo doc --no-deps -p ak-core -p
  ak-vis` is staged into the published docs site under `api/rustdoc/`.
- Difficulty: The awkward part was not generating the content but making it usable in
  both deployed and local-file browsing modes. Directory-style MkDocs URLs produced
  local index listings instead of pages, and the first Rust links were wrong because
  the final HTML layout under `use_directory_urls = false` changes how relative links
  resolve from `api/rust.html` to the staged `rustdoc` subtree.
- Constraints: The Python reference is intentionally curated rather than a full module
  dump; pages use selected `members:` blocks to keep the docs readable. Rust remains
  documented through native `rustdoc` instead of being re-rendered inside MkDocs, so
  the docs workflow now depends on preserving the rustdoc staging step in both local
  builds and the Pages workflow.
- Follow-up: If the Python API grows further, keep splitting reference pages by user
  task rather than expanding `api/python.html` into another monolithic generated page.
  If live docs preview becomes important, consider wrapping `zensical serve` with the
  rustdoc staging step so local iteration reflects Rust API changes automatically.

## 2026-03-12 - Core and viewer coverage expansion with structure-view fix

- Commit: `73deb24`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The `codex/quality-improvements` branch packages the code-only results from a
  broader quality pass. The highest-value functional fix in that batch was correcting
  `StructureView::is_empty()` in `crates/ak-core/src/geometry/structure_view.rs`, which
  had been returning `true` for non-empty views. The rest of the change raises
  confidence in the Rust crates by adding direct coverage around core geometry helpers,
  XYZ parsing, Bevy viewer controls, and render helpers.
- Implementation: Added `crates/ak-core/tests/core_module_coverage.rs` to exercise
  `ak-core` module surfaces including calculator behavior, geometry helpers, periodic
  neighbor lists, periodic-table lookup, and XYZ loading. Added
  `crates/ak-vis/tests/visual_module_coverage.rs` plus new unit tests in
  `crates/ak-vis/src/components.rs` and
  `crates/ak-vis/src/viewer/controls/{camera,navigation,screenshot,ui}.rs` to verify
  viewer control behavior, render helper spawning, and component wiring. Also fixed
  uppercase and word-form PBC parsing in `crates/ak-core/src/io/xyz.rs` and applied two
  small cleanup changes in `crates/ak-vis/src/viewer/headless.rs` and
  `crates/ak-vis/src/viewer/session.rs`.
- Difficulty: The awkward part was not the domain logic but Bevy test ergonomics.
  Systems using `Commands`, `Local<Timer>`, `Single<...>`, and generic `Time<T>` needed
  tests that matched Bevy 0.18's exact ECS APIs instead of the more obvious app-level
  setup. The work also confirmed that broad static "untested module" signals needed to
  be answered with real behavior coverage, not just crate-root imports.
- Constraints: This branch intentionally excludes the `desloppify` workspace artifacts
  and dependency changes so it can be reviewed as a normal code PR into `development`.
  The Python bridge coverage work was left out of this branch because the first clean
  extraction target was the Rust-only improvement set.
- Follow-up: If the Python crate needs the same treatment, mirror this branch's
  strategy by adding Rust-side unit tests around `ak-py` conversion helpers and binding
  surfaces, then document that separately once it lands as its own implementation
  commit.

## 2026-03-11 - Live viewer camera smoothing restored to PanOrbit defaults

- Commit: `f65ff89`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: A recent viewer change made the interactive camera feel less sensitive even
  though the keyboard orbit and zoom increments had not changed. The regression came
  from stronger live-mode smoothing in the Bevy `PanOrbitCamera` setup rather than from
  the explicit camera control constants.
- Implementation: Updated `crates/ak-vis/src/viewer/systems.rs` so the live viewer now
  sets `orbit_smoothness`, `pan_smoothness`, and `zoom_smoothness` to the
  `bevy_panorbit_camera` defaults (`0.1`, `0.02`, `0.1`) explicitly, while keeping the
  headless-render path at `0.0` smoothing for deterministic camera state application.
- Difficulty: The misleading part of this regression was that the obvious camera input
  code in `crates/ak-vis/src/viewer/controls/camera.rs` still used the same per-frame
  orbit and zoom deltas as before. The behavior change came from interpolation settings
  added later in camera setup, so the investigation had to compare the input layer,
  session-state refactor, and the upstream `PanOrbitCamera` defaults before touching
  anything.
- Constraints: This restores the previous feel for live interactive viewing only. It
  does not change the headless camera path, which still disables smoothing on purpose,
  and it does not yet expose camera smoothing as a viewer config option or Python API.
- Follow-up: If camera feel becomes something users need to tune intentionally, add a
  viewer config surface for smoothing rather than relying on hard-coded crate defaults
  buried in the Bevy camera spawn path.

## 2026-03-11 - Rust headless render tests disabled in default CI

- Commit: `e599547`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: After promoting the cached validation workflow to the primary CI entrypoint,
  the remaining unstable signal was the Rust-side headless render tests in
  `crates/ak-vis/src/viewer/headless.rs`, which still failed semi-randomly on GitHub's
  Ubuntu runners even after the Python headless integration path had been stabilized
  enough to pass.
- Implementation: Gated the Rust headless render tests behind the
  `ATOMIC_KERNELS_RUN_RUST_HEADLESS_TESTS` environment variable in
  `crates/ak-vis/src/viewer/headless.rs`, so they now skip automatically in CI while
  still running by default outside CI. Updated `justfile` so `just headless-test`
  explicitly sets that environment variable before invoking
  `cargo test -p ak-vis viewer::headless::tests`, preserving the manual/local workflow
  for end-to-end headless coverage.
- Difficulty: The important decision here was scope rather than mechanics. Repeated
  attempts to make the Rust headless tests deterministic on CI still left a flaky path,
  while the Python job already exercised the real headless renderer more reliably. At
  that point the better engineering choice was to narrow default CI to the stable
  signal instead of continuing to treat a semi-random test as required validation.
- Constraints: This does not mean the Rust headless tests are fixed. It means the
  default CI suite should not be interpreted as full coverage of the Rust-native
  offscreen render path. Those tests now require explicit opt-in in CI, and future
  regressions in that path will not be caught unless a dedicated headless-render job is
  added back with a more deterministic environment.
- Follow-up: If the Rust-native headless renderer becomes a required release gate, put
  it in its own explicitly named CI job with dedicated environment assumptions rather
  than folding it back into the default `cargo test` path. When that happens, add a
  follow-up note describing the environment and why it is stable enough to trust.

## 2026-03-11 - Cached validation workflow promoted to primary CI

- Commit: `fc7d913`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The branch-only cache experiment had reached the point where its three-job
  shape was more trustworthy than the repository’s historical `maturin`-generated
  `CI.yml`, and future CI work needed a real canonical workflow rather than a sidecar
  proof of concept.
- Implementation: Replaced `.github/workflows/CI.yml` with the validated Ubuntu-based
  workflow that runs separate cached Rust build, Rust test, and Python test jobs, and
  removed `.github/workflows/rust-build-poc.yml`. The promoted workflow keeps the
  shared `Swatinem/rust-cache@v2` setup, reuses the
  `.github/actions/install-linux-build-deps/action.yml` composite action, and preserves
  the Python job pattern of `uv sync --no-install-project`, explicit `maturin`
  installation, `maturin develop`, and pytest execution.
- Difficulty: The main care point was preserving the working behavior while changing
  the repository’s source of truth. The old file still contained a large amount of
  dead release-matrix configuration, so the promotion step needed to be a clean
  replacement rather than an incremental edit that left two competing CI entrypoints in
  the tree.
- Constraints: This promoted CI is intentionally limited to the Linux validation jobs
  that were actually exercised on this branch. It should now be treated as the default
  template for future CI edits, but it still inherits the known headless-render/runtime
  caveats discovered during the investigation and does not yet restore the historical
  wheel-building and release automation.
- Follow-up: If publishing artifacts is still required, rebuild that flow deliberately
  on top of this CI rather than reviving the old autogenerated matrix wholesale.
  Capture the headless CI findings in a separate note once that runtime path is either
  stabilized or explicitly scoped out of default validation.

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
