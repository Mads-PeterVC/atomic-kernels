# Getting Started

## Prerequisites

- Rust toolchain
- Python 3.8+
- `uv`
- system dependencies required by the viewer stack

## Installing a prebuilt wheel

Prebuilt wheels are currently available only for macOS Apple Silicon (`arm64`) on
Python 3.12, via GitHub Releases.

```bash
pip install "https://github.com/au616397/atomic-kernels/releases/download/vX.Y.Z/atomic_kernels-X.Y.Z-cp312-cp312-macosx_14_0_arm64.whl"
```

If you are on a different platform or Python version, build from source instead.

## Development environment

Install the Python package in development mode:

```bash
uv sync
uv run maturin develop
```

## Common workflows

Run a Python script:

```bash
uv run python scripts/slab_adsorbate_ball_and_stick.py
```

Browse the reusable viewer examples:

- [Viewer Examples](/Users/au616397/Repositories/atomic-kernels/docs/src/viewer-examples.md)
- [Viewer Scripts](/Users/au616397/Repositories/atomic-kernels/docs/src/viewer-scripts.md)

Run Rust tests:

```bash
cargo test
```

Serve the documentation site:

```bash
uv sync --group docs
just docs-serve
```

## Documentation rules

- Prefer Markdown documents under `docs/`
- Keep top-level user-facing context in the docs site, not in scattered notes
- Update the `Agent Development` page when agent-driven work changes architecture,
  workflows, conventions, or known limitations
