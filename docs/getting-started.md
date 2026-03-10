# Getting Started

## Prerequisites

- Rust toolchain
- Python 3.8+
- `uv`
- system dependencies required by the viewer stack

## Development environment

Install the Python package in development mode:

```bash
uv sync
uv run maturin develop
```

## Common workflows

Run a Python script:

```bash
uv run python scripts/py_slab_adsorbate_ball_and_stick.py
```

Run Rust tests:

```bash
cargo test
```

Serve the documentation site:

```bash
uv sync --group docs
uv run zensical serve
```

## Documentation rules

- Prefer Markdown documents under `docs/`
- Keep top-level user-facing context in the docs site, not in scattered notes
- Update the `Agent Development` page when agent-driven work changes architecture,
  workflows, conventions, or known limitations
