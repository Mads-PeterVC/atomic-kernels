# Agent Development Notes

This page is the running log for agent-driven development updates. Follow the workflow
and format defined in
[`docs/agent-development.md`](/Users/au616397/Repositories/atomic-kernels/docs/agent-development.md).

## Existing notes

## 2026-03-10 - Documentation build workflow in GitHub Actions

- Commits: `e3ad597`, `adf8e27`
- Context: The documentation setup needed CI coverage so docs configuration and content
  changes are validated automatically instead of only when someone runs the site
  locally.
- Implementation: Added `.github/workflows/docs.yml` to install the docs-only Python
  dependency group with `uv`, run `zensical build`, upload a GitHub Pages artifact, and
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
- Context: The repository had reached a size where architecture and workflow knowledge
  was no longer recoverable from source layout alone, and the top-level README did not
  provide a usable entry point.
- Implementation: Added a Markdown-first docs site via `Zensical`, introduced
  `docs/index.md`, `docs/getting-started.md`, `docs/architecture.md`, and the agent
  development pages, and linked the docs entry points from the repository README.
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
- Context: The Python viewer API needed a second rendering mode beyond space-filling so
  scripts could highlight subsets such as adsorbates, alloy species, or coordination
  environments without replacing the full-scene representation.
- Implementation: Added bond and render-style domain types in
  `crates/ak-vis/src/viewer/session.rs`, bond rendering in
  `crates/ak-vis/src/render/render_bonds.rs` and
  `crates/ak-vis/src/visuals/bond_visual.rs`, and a Python render controller in
  `python/atomic_kernels/viewer/_render.py` wired through the PyO3 bindings in
  `crates/ak-py/src/pyfunctions/py_viewer.rs`. Example usage lives in
  `scripts/py_ball_and_stick_selection.py` and
  `scripts/py_slab_adsorbate_ball_and_stick.py`.
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
