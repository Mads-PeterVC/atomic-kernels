# Agentic Development Notes

This page is the running log for agent-driven development updates. Follow the workflow
and format defined in
[`docs/src/agent-development.md`](/Users/au616397/Repositories/atomic-kernels/docs/src/agent-development.md).

## Existing notes

Entries are listed newest first.

## 2026-03-19 - Chemistry helper defaults, topology mutation commands, and viewer UI polish

- Commit: `b841170`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The viewer already supported explicit bonds, explicit faces, and
  selection-scoped ball-and-stick styling, but common chemistry workflows still
  required callers to derive all topology themselves. At the same time, the Python
  interactive viewer path still inherited the Rust-side `show_ui=false` default, and
  the new playback panel remained visible even for single-frame scenes where it did not
  add useful controls.
- Implementation: Added Python-side chemistry helpers in
  `python/atomic_kernels/viewer/_chemistry.py` and exposed them through
  `python/atomic_kernels/viewer/__init__.py` plus the render facade in
  `python/atomic_kernels/viewer/_render.py`. `render.set_bonds(mode="default")` now
  derives connectivity from ASE natural cutoffs, and
  `render.set_faces(mode="default", selection=...)` derives best-effort ligand-shell
  polyhedra from coordination environments while still reducing to explicit Rust-side
  bond and face lists. Added incremental Rust session commands for
  `add/remove/clear` on both bonds and faces in
  `crates/ak-vis/src/viewer/session.rs`, exposed them through
  `crates/ak-py/src/pyfunctions/py_viewer.rs`, and bridged them across the macOS
  subprocess path in `python/atomic_kernels/viewer/_process.py`. On the UX side,
  interactive Python viewer launches in `python/atomic_kernels/viewer/__init__.py` now
  synthesize a `show_ui=true` config when callers omit one, and
  `crates/ak-vis/src/ui/playback/systems.rs` now hides the playback panel root when the
  trajectory has only one frame. Examples and docs were updated in
  `scripts/polyhedra_faces.py`, `scripts/polyhedra_minimal.py`,
  `docs/src/viewer-examples.md`, and `docs/src/viewer-scripts.md`.
- Difficulty: The main friction was not the Rust mutation path but the Python-side
  ergonomics and visibility story. Auto-polyhedra generation itself worked, but a
  "minimal" example that only added translucent faces was visually misleading because
  the default space-filling atoms largely occluded the shell, so the minimal workflow
  still needed auto bonds plus whole-structure ball-and-stick styling to read as
  intended. The default bond path also exposed a subtle normalization edge case: a
  `2x2` array of explicit bond pairs was initially misread as an adjacency matrix until
  the explicit-pair interpretation was made to win in that ambiguous shape.
- Constraints: The chemistry helpers are still convenience APIs rather than chemically
  authoritative analysis tools. Auto-bonds use ASE natural cutoffs with the current
  default multiplier, auto-polyhedra use a local convex-hull style construction and
  skip degenerate/non-hullable environments, and the Rust viewer still stores only
  explicit per-frame bond and face topology. Hiding the playback panel for single-frame
  scenes only changes UI visibility; it does not remove playback state resources or
  special-case the rest of the viewer loop.
- Follow-up: Replace `TBD` with the actual implementation commit hash once the feature
  commit exists. If future interactive keybindings need selection-driven topology
  editing, build them on top of the new `add/remove/clear` session commands instead of
  adding a second mutation path.

## 2026-03-19 - Trajectory playback panel, Bevy slider adoption, and startup camera fix

- Commits: `8bf68f7`, `0cefc46`, `8e7c5d4`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The viewer had frame stepping and follow-tail session controls, but no
  dedicated trajectory playback surface. This work added a first-class playback panel
  with transport controls, a trajectory scrubber, speed presets, and clearer current
  frame status, then followed up with the module split and CI/headless fixes needed to
  make that implementation durable.
- Implementation: Added playback-specific UI state, systems, widgets, and tests under
  `crates/ak-vis/src/ui/playback/{state,systems,widgets,tests}.rs`, plus the module
  entrypoint `crates/ak-vis/src/ui/playback.rs`. Extended
  `crates/ak-vis/src/viewer/app.rs`,
  `crates/ak-vis/src/viewer/controls/navigation.rs`,
  `crates/ak-vis/src/viewer/runtime.rs`, and
  `crates/ak-vis/src/viewer/session.rs` so timed playback, delayed key-repeat frame
  stepping, and session-driven frame/camera behavior share the same viewer state path.
  Reworked the panel build path into
  `crates/ak-vis/src/ui/{build.rs,build_inspector.rs,build_playback_panel.rs}` and
  replaced the custom scrubber interaction with Bevy `ui_widgets` slider support via
  `crates/ak-vis/Cargo.toml`. The final follow-up in
  `crates/ak-vis/src/viewer/runtime.rs` preserves startup-scripted camera commands for
  prepared headless renders by preventing the default initial camera reset from
  clobbering them.
- Difficulty: The hard part was not the timer-based playback logic itself but the UI
  interaction path. The first custom scrubber implementation repeatedly mis-mapped the
  cursor, leaked drag gestures into camera orbiting, and was hard to grab at frame
  zero, so the final solution abandoned that path and switched to Bevy's experimental
  slider widget. The Rust 2018 module split also hit an easy-to-miss repository
  constraint: putting submodules under `crates/ak-vis/src/ui/build/` looked natural
  locally, but the top-level `.gitignore` ignores `build/`, so CI checked out a tree
  missing those files until the split was moved to non-ignored paths. The last bug was
  even subtler: startup-queued camera commands for prepared headless renders were
  applied correctly and then silently overwritten by the normal first-frame camera
  reset, which made scripted and default renders identical in CI.
- Constraints: The playback panel is currently viewer-only and intentionally narrow. It
  exposes play/pause, prev/next, a scrubber, speed presets, and a current-frame
  readout, but it does not yet add loop modes, timeline annotations, or Python-facing
  play/pause APIs. The scrubber now depends on Bevy's experimental `ui_widgets` slider
  support, and the playback panel remains a separate HUD surface from the inspector
  rather than a generalized UI framework.
- Follow-up: If playback grows further, keep using the separated
  `ui/playback/{state,systems,widgets}` structure instead of folding more behavior back
  into `ui.rs` or `ui/build.rs`. If headless scripting adds more startup-time scene
  configuration, preserve the current rule that queued camera commands must survive app
  initialization rather than being treated as disposable pre-start state.

## 2026-03-18 - Supercell viewer state, image-aware selection, and repeat hotkeys

- Commit: `c24f53c`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The viewer already had live selection and Python session control, but it
  still treated the displayed structure as exactly one canonical cell. This work added
  repeated-image display as first-class viewer state, preserved a distinct notion of
  main-cell versus repeated atoms, and exposed that model through both the Rust session
  layer and the Python viewer facade.
- Implementation: Extended `crates/ak-vis/src/viewer/session.rs` with supercell
  settings, image-aware selected-atom identity, snapshot/session commands for repeat
  control, and a display-expanded atom list derived from the canonical trajectory
  rather than replacing it. Wired repeated-image rendering and picking through
  `crates/ak-vis/src/viewer/systems.rs`,
  `crates/ak-vis/src/render/render_atoms.rs`,
  `crates/ak-vis/src/components.rs`, and the visuals modules so repeated images carry
  `(atom_index, image_offset)` identity and ghosted styling. Added `1`/`2`/`3`,
  `Shift+1`/`Shift+2`/`Shift+3`, and `0` viewer shortcuts in
  `crates/ak-vis/src/viewer/controls/navigation.rs` plus matching keybinding text in
  `crates/ak-vis/src/ui/shortcuts.rs`. Exposed launch-time and live supercell controls
  through the PyO3 bridge in `crates/ak-py/src/{viewer_config.rs,pyfunctions/py_viewer.rs}`
  and the Python facade/proxy/stub layers in
  `python/atomic_kernels/viewer/{_config,_process,_session}.py` and
  `python/atomic_kernels/atomic_kernels.pyi`, with coverage added in
  `tests/test_camera_and_session.py`, `tests/test_viewer_process.py`, and
  `tests/test_viewer_integration.py`.
- Difficulty: The hard part was that supercells were not just a render toggle. The
  original selection model was only a per-frame boolean mask over canonical atoms, so
  repeated images initially had nowhere stable to live in picking, snapshots, or
  Python readback. The camera path also needed a correction during rollout: once the
  first supercell implementation used displayed-atom bounds for `camera_view_for_frame`,
  the initial framing and `X`/`Y`/`Z` snap views unexpectedly zoomed tighter than the
  longstanding cell-based behavior. The final shape deliberately separates those
  concerns by keeping display repetition dynamic while preserving the old default/snap
  framing contract.
- Constraints: Repetition is currently symmetric around the main cell and controlled by
  per-axis repeat extents, not explicit negative/positive bounds. The viewer ghosts
  repeated-only images as its only distinction mode, with `0` toggling that styling
  off. Main-cell selection helpers remain part of the public API for convenience, while
  repeated-image selection uses explicit `(atom_index, image_offset)` records instead
  of flattening all displayed atoms into one global index space.
- Follow-up: If future work wants `FrameAll` or other explicit camera commands to fit
  the displayed supercell instead of the canonical cell, do that as a deliberate camera
  policy change rather than by reusing the initial/snap framing path. If repeated-image
  styling later needs image-aware bonds, faces, or subset render rules, build on the
  new display-identity model instead of falling back to ambiguous flattened indices.

## 2026-03-18 - Python viewer CLI and follow-up viewer fixes

- Commits: `55444a6`, `3550aa1`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: This work introduced the first installed Python CLI entry point for the
  viewer so ASE-readable files can be opened directly from the shell, while also
  tightening several viewer-facing details discovered during rollout: live window size
  defaults, inspector keybinding label wording, and the axis-snap shortcut mapping for
  `X`/`Y`/`Z`.
- Implementation: Added the new `ak` console script in `pyproject.toml` and the
  Python package modules `python/atomic_kernels/cli/{__init__,main,view}.py`, using
  `click` plus `rich-click` option groups to expose `ak view FILE` with theme, window
  sizing, and toggle flags. Extended the Python/Rust viewer config bridge in
  `python/atomic_kernels/viewer/_config.py`,
  `python/atomic_kernels/atomic_kernels.pyi`,
  `crates/ak-py/src/viewer_config.rs`, and
  `crates/ak-vis/src/viewer/config.rs` so live viewers can carry explicit window
  dimensions, then threaded those dimensions into Bevy window creation in
  `crates/ak-vis/src/viewer/app.rs`. Follow-up fixes updated
  `crates/ak-vis/src/viewer/controls/camera.rs` so `X` snaps to `+X`, `Y` to `+Y`,
  and `Z` to `-Z`, and adjusted the inspector keybinding header in
  `crates/ak-vis/src/ui/{shortcuts,state,tests}.rs` to keep the keycap while showing
  the shorter `Keybindings (H)` label.
- Difficulty: The awkward parts were mostly in the seams between systems. The first
  Bevy window-size implementation accidentally replaced the default primary window with
  `None` whenever the CLI did not provide explicit dimensions, which caused normal
  viewer scripts to exit immediately with "No windows are open". The inspector label
  cleanup also needed a second pass because shortening the text too aggressively broke
  the dedicated toggle-text node and dropped the keycap rendering that the UI tests
  and shortcut affordance relied on. Even the test suite needed iteration: rich-click
  help formatting is environment-sensitive enough that strict string assertions were
  brittle, and Rust-backed color tuples had to be compared approximately rather than
  by exact decimal literals.
- Constraints: The CLI currently remains viewer-only and intentionally narrow. It
  shells directly into `bevy_viewer(...)` with ASE-based loading, a light/dark theme
  preset, explicit live window sizing, and a small set of render toggles; it does not
  yet expose broader viewer/session control, playback, or headless render commands.
  The snap-view shortcut mapping is now opinionated for slab work by treating `Z` as a
  top-down `-Z` view rather than a bottom-up `+Z` one.
- Follow-up: If the CLI grows beyond `view`, keep the command tree under
  `python/atomic_kernels/cli/` instead of mixing it into package roots, and prefer
  extending the existing `ViewerConfig` seam rather than introducing a parallel
  CLI-only viewer launch path. If more keyboard-driven camera views are added, cover
  them in `crates/ak-vis/src/viewer/controls/camera.rs` tests so axis semantics do not
  drift again.

## 2026-03-18 - Inspector UI, angle cue, and UI module split

- Commits: `fb94de0`, `8e2d2f7`, `fd041e5`, `24496e7`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: Once live selection existed in the viewer, the next high-value gap was a
  usable inspection surface. This work replaced the placeholder overlay with a real
  inspector panel, added selection-driven distance and angle readouts, and then pushed
  the angle case far enough to be legible in-scene instead of only as text. The same
  branch also cleaned up the now-large UI module into a directory-backed Rust 2018
  module layout so future UI work does not pile more behavior into one file.
- Implementation: Added a structured inspector in `crates/ak-vis/src/ui.rs` and later
  split it into `crates/ak-vis/src/ui/{build,shortcuts,state,tests}.rs`, with
  `crates/ak-vis/src/ui.rs` kept as the module entrypoint. The panel now owns
  selection and measurement state, a collapsible keybindings section, symbol-font
  keycaps, and the `U`/`H` toggle behavior. Selection-driven angle rendering was added
  in `crates/ak-vis/src/viewer/systems.rs` as derived scene geometry (two rays plus an
  arc), with supporting frame markers and teardown updates in
  `crates/ak-vis/src/components.rs` and
  `crates/ak-vis/src/viewer/controls/navigation.rs`. Preserving click order for the
  angle vertex required extending `crates/ak-vis/src/viewer/session.rs` so
  `SelectionFrames` stores per-frame selection order alongside the boolean mask.
- Difficulty: The awkward parts came in layers. The first panel version initially
  bound itself to the orientation-widget camera instead of the main scene camera, then
  briefly broke click-picking because the UI overlay was still pickable. The angle cue
  then rendered correctly in data terms but was hidden inside the enlarged selection
  shell until its radius was made relative to the actual vertex atom highlight. The
  most important semantic bug was that the old selection model only stored a boolean
  mask, so `selected_atoms()` reconstructed indices in ascending order and silently
  changed `A-B-C` measurements into `min-mid-max`; fixing that required treating
  selection order as real viewer state rather than an incidental UI detail. Even the
  keybinding chips took several iterations because the chosen font and codepoint family
  had to be made consistent before the arrows stopped looking mismatched.
- Constraints: Measurement remains viewer-only and selection-driven. Distances are
  still text-only for 2 selected atoms, while the scene cue exists only for 3-atom
  angle measurements and uses the second selected atom as the vertex. The keybinding
  arrows currently depend on the bundled `NotoSansSymbols2-Regular.ttf` asset and use
  per-glyph placement tweaks rather than a general icon system. The inspector does not
  yet include trajectory status or playback controls.
- Follow-up: The next clean UI slice is a separate trajectory/status control surface,
  since the current inspector intentionally dropped trajectory information to stay
  focused on selection and measurement. If distance measurements also need an in-scene
  cue, build it on top of the same derived measurement path rather than introducing a
  second ad hoc overlay model.

## 2026-03-17 - Real viewer click smoke test added under Xvfb

- Commit: `294787b`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The new inspector UI briefly regressed live click picking, which exposed a
  gap in the existing integration coverage: the project had real viewer CI, but no
  end-to-end test that proved UI-enabled pointer input still selected atoms. This
  change added one narrow smoke test for that contract and a matching manual scene so
  the setup can be inspected visually before trusting CI.
- Implementation: Added `tests/viewer_integration_helpers.py` with a small reusable
  harness for deterministic viewer-input scenes, UI-enabled session launch, polling,
  and real X11 click injection through `xdotool`. Added the manual validation script
  `scripts/viewer_click_selection_scene.py`, extended
  `tests/test_viewer_integration.py` with a single UI-enabled atom-click smoke test,
  and updated `.github/actions/install-linux-build-deps/action.yml` so the viewer
  integration lane installs `xdotool`.
- Difficulty: The hard part was that several plausible click strategies worked
  differently under `xvfb-run` than in a normal desktop session. The initial
  screen-center click missed the window, window activation failed because there is no
  real window manager in the CI environment, and window-targeted `xdotool click
  --window ...` still did not trigger Bevy selection reliably. The stable solution was
  to resolve the actual viewer window geometry, move the real X pointer to absolute
  coordinates, and emit explicit `mousedown`/`mouseup` events. The test harness also
  needed richer debug output while converging on that path.
- Constraints: The smoke test is intentionally narrow. It only asserts that one visible
  atom can be selected with `show_ui=True` under the Linux/Xvfb CI path, and it skips
  cleanly unless `ATOMIC_KERNELS_RUN_VIEWER_TESTS=1`, a windowing session, and
  `xdotool` are all present. It is a regression guard for UI-enabled click picking,
  not a broad GUI automation framework.
- Follow-up: If more live viewer input coverage is added, extend the same helper
  module with additional deterministic scenes rather than duplicating subprocess and
  `xdotool` setup inline. The next likely additions would be Shift-click selection
  order and marquee-selection coexistence with the inspector enabled.

## 2026-03-17 - Interactive viewer selection and marquee picking

- Commit: `bd816a4`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The viewer already had Python-side selection-aware rendering and coloring,
  but live interaction still treated selection as a script-led concept instead of real
  viewer state. This work made atom selection durable in the running viewer, exposed it
  through the Python session API, and added Shift-drag marquee selection so common slab
  and layer workflows do not depend on clicking atoms one by one.
- Implementation: Extended `crates/ak-vis/src/viewer/session.rs` with frame-scoped
  shared selection state, selection commands, and a snapshot path for live readback.
  Wired live atom picking and selection highlighting through
  `crates/ak-vis/src/viewer/systems.rs`, `crates/ak-vis/src/viewer/app.rs`, and the
  atom/entity tagging changes in `crates/ak-vis/src/render/render_atoms.rs`,
  `crates/ak-vis/src/components.rs`, and `crates/ak-vis/src/visuals/convert.rs`.
  Exposed selection query and mutation methods through the PyO3 bindings in
  `crates/ak-py/src/pyfunctions/py_viewer.rs` and the Python facade/proxy layers in
  `python/atomic_kernels/viewer/_session.py`,
  `python/atomic_kernels/viewer/_process.py`, and
  `python/atomic_kernels/atomic_kernels.pyi`. Follow-up work on the same commit added a
  screen-space Shift-drag marquee rectangle that targets the main scene camera and
  replaces the current frame selection using projected atom centers without occlusion
  filtering.
- Difficulty: The awkward part was that selection touched several different abstractions
  at once. The viewer command channel was one-way, so Python readback needed a shared
  snapshot instead of another ad hoc request/response path. The first visible selection
  highlight also broke Shift-click deselection because the translucent shell itself was
  pickable and intercepted clicks until it was explicitly marked `Pickable::IGNORE`.
  Marquee selection then added another layer of input coordination: the drag path needed
  to suppress PanOrbit's normal left-button orbiting only for the active gesture, avoid
  turning a tiny Shift-click jitter into a rectangle selection, and target the correct
  UI camera so the marquee box actually rendered above the 3D scene.
- Constraints: Shared selection is frame-scoped and atom-only. It supports live picking,
  Shift-click toggling, Python query/replace/add/remove/clear operations, and
  Shift-drag rectangle replacement on the current frame. The marquee path is
  intentionally depth-agnostic and uses projected atom centers rather than sphere
  overlap, which is what makes side-view slab selection practical but also means it is
  not a visibility-filtered lasso tool. The overlay box is a live-viewer affordance and
  does not introduce a new Python UI API.
- Follow-up: The next clean feature on top of this is measurement and inspection UI:
  distances for 2 selected atoms, angles for 3 selected atoms, and a deliberate choice
  about whether those appear as a HUD panel, inline labels, or a more explicit viewer
  tool mode. If supercell display lands later, selection identity will need another pass
  so repeated images can participate without collapsing back into the original-cell atom
  set.

## 2026-03-17 - Dedicated viewer integration CI stabilized on Linux

- Commits: `e043738`, `56b64dd`, `5c07633`, `bf15327`, `9a9a986`, `2221f78`,
  `9d6b5da`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The repository already had real viewer and headless-render tests, but they
  were either skipped in normal CI or mixed into the default Python path without a
  trustworthy runtime contract. This work carved those checks into a dedicated CI lane
  and then iterated on the Linux/Xvfb path until the live viewer session and Rust
  headless tests could pass reliably enough to use as a real integration signal.
- Implementation: Added a `Viewer Integration` job in `.github/workflows/CI.yml`,
  removed `ATOMIC_KERNELS_RUN_VIEWER_TESTS=1` from the default Python test job, and
  extended `.github/actions/install-linux-build-deps/action.yml` with the Xvfb and X11
  keyboard runtime packages the live viewer actually needs on GitHub-hosted Ubuntu.
  Tightened `crates/ak-vis/src/viewer/headless.rs` for slower CI rendering, adjusted
  the live-viewer readiness path in `crates/ak-vis/src/viewer/app.rs`, hardened the
  subprocess-backed Python viewer lifecycle in
  `python/atomic_kernels/viewer/_process.py`, and added focused regression coverage in
  `tests/test_viewer_process.py` plus a second real viewer trajectory command smoke
  test in `tests/test_viewer_integration.py`.
- Difficulty: The hard part was that the first failures were misleading. The viewer
  test initially looked like a generic readiness timeout, but the actual problems came
  in sequence: missing `libxkbcommon-x11` on the runner, unreaped subprocess teardown
  that left `xvfb-run` hanging, and finally a readiness handshake that stayed false in
  the Linux subprocess path even after Bevy had clearly created a real window. Getting
  to a green lane required separating genuine runtime dependencies from process-lifetime
  bugs and from the handshake path itself instead of treating everything as the same
  timeout symptom.
- Constraints: The dedicated lane is intentionally Linux-only for now and runs against
  `xvfb` plus Mesa's software-rendered Vulkan (`llvmpipe`), so it should be understood
  as an explicit CI environment rather than a promise about all GUI backends. The
  Python subprocess viewer path now uses a CI-specific startup-liveness fallback for
  readiness because that proved more reliable than the existing explicit ready signal
  under Xvfb, while the non-CI path still uses the original request/response handshake.
  The docs workflow was also narrowed during this branch so `docs.yml` is no longer
  triggered on pull requests and remains manual/push-driven instead.
- Follow-up: If the live viewer integration lane stays stable, consider whether the
  CI-specific readiness fallback can be replaced with a more principled backend signal
  from the Rust viewer app. If more live viewer coverage is added, prefer a small
  number of behavior-distinct smoke tests over many overlapping windowed tests so the
  lane stays fast and diagnosable.

## 2026-03-16 - Wheel build and release workflow consolidated

- Commits: `01b7d6f`, `b84b0a4`, `3ffc69b`, `87c72c8`, `e164e0f`, `5c55fa2`,
  `9b5929a`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The Python package started with source-build instructions only. This work
  introduced a GitHub Release-based wheel distribution path, then expanded it into a
  practical binary install workflow with broader platform coverage and improved docs.
- Implementation: Added `.github/workflows/python-wheel.yml` for release-oriented
  wheel builds, then expanded it from a single Apple Silicon CPython 3.12 wheel to a
  matrix covering macOS arm64, macOS x86_64, and Linux x86_64 for CPython 3.12 and
  3.13. The workflow now builds with `maturin`, smoke-installs each wheel, uploads
  artifacts per matrix job, and publishes all wheel assets together on tag pushes.
  Follow-up changes moved the workflow's Python environment setup to `uv`, updated the
  Intel macOS runner label to a supported GitHub-hosted runner, and refreshed
  `README.md`, `docs/src/getting-started.md`, and `docs/zensical.toml` so the install
  docs reflect the supported platforms, use platform tabs, and expose copy buttons.
- Difficulty: The hard part was keeping the release flow coherent while broadening
  support. Once the workflow moved beyond one wheel, it needed a separate publish
  phase to avoid matrix jobs racing each other, target-specific filename verification,
  and runner labels that matched GitHub's supported Intel macOS offerings.
- Constraints: The current wheel matrix is intentionally limited to macOS arm64,
  macOS x86_64, and Linux x86_64 for CPython 3.12 and 3.13. Linux still builds on the
  native Ubuntu runner rather than manylinux, and the project still does not publish
  an sdist or PyPI release. The docs can link to the latest release page, but they
  cannot provide a truly version-agnostic direct wheel URL because wheel filenames
  embed the package version.
- Follow-up: If wheel distribution becomes a primary install path, decide whether to
  add Windows and Linux aarch64 builds, revisit Linux portability versus manylinux,
  and consider whether the docs should move from explicit asset examples toward a
  helper flow that resolves the latest matching wheel automatically.

## 2026-03-16 - Workspace version became the single release source of truth

- Commit: `90afb85`
- Agent: `Codex (GPT-5, OpenAI)`
- Context: The mixed Rust/Python project needed one shared version number across the
  Rust crates and the Python package, but the repository had been repeating `0.1.0` in
  each crate manifest while the Python package version was derived indirectly through
  `maturin`. This change makes version control explicit and centralized so future
  release bumps are less error-prone.
- Implementation: Added `[workspace.package] version = "0.1.0"` to `Cargo.toml`,
  switched all workspace crates to `version.workspace = true`, and added `just`
  recipes in `justfile` for `bump`, `bump-patch`, `bump-minor`, and `bump-major`, all
  delegating to `cargo set-version --workspace ...`. The existing dynamic Python
  version flow in `pyproject.toml` remains intact, so `maturin` continues exposing the
  workspace-controlled Cargo version to the Python package.
- Difficulty: The main design choice was picking the correct source of truth rather
  than the mechanics of editing manifests. Because Python already derives its version
  from the Rust packaging path, using a Python-first bump tool such as `uv version`
  would have introduced competing authority instead of simplifying the release flow.
- Constraints: The new `just bump*` commands assume `cargo set-version` is available in
  the developer environment, typically via `cargo-edit`. This change does not add tag
  creation or release publishing automation; it only makes the shared version bump
  itself consistent.
- Follow-up: If release hygiene matters further, add a higher-level `just release`
  helper that validates the worktree, runs the relevant checks, and creates the `vX.Y.Z`
  tag after a successful version bump.

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
