# Getting Started

## Prerequisites

- Rust toolchain
- Python 3.12+
- `uv`
- system dependencies required by the viewer stack

## Installing a prebuilt wheel

Prebuilt wheels are currently available via GitHub Releases for:

- macOS Apple Silicon (`arm64`)
- macOS Intel (`x86_64`)
- Linux `x86_64`
- Python 3.12 and 3.13

=== "macOS Apple Silicon"

    ```bash
    pip install "https://github.com/Mads-PeterVC/atomic-kernels/releases/download/vX.Y.Z/atomic_kernels-X.Y.Z-cp312-cp312-macosx_11_0_arm64.whl"
    ```

=== "macOS Intel"

    ```bash
    pip install "https://github.com/Mads-PeterVC/atomic-kernels/releases/download/vX.Y.Z/atomic_kernels-X.Y.Z-cp312-cp312-macosx_11_0_x86_64.whl"
    ```

=== "Linux x86_64"

    ```bash
    pip install "https://github.com/Mads-PeterVC/atomic-kernels/releases/download/vX.Y.Z/atomic_kernels-X.Y.Z-cp312-cp312-manylinux_2_17_x86_64.manylinux2014_x86_64.whl"
    ```

See [GitHub latest release page](https://github.com/Mads-PeterVC/atomic-kernels/releases/latest) for the newest versions (replacing `X.Y.Z` in the above).

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
