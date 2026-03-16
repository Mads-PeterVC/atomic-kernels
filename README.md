# atomic-kernels

`atomic-kernels` (`ak`) is a mixed Rust/Python project for atomistic structure tooling.
The current codebase combines:

- Rust crates for core geometry, visualization, and Python bindings
- A Python package built with `maturin`
- ASE-oriented scripting workflows for neighbor lists and interactive viewing

## Documentation

The project now uses a Markdown-first docs structure intended for `Zensical`.

- Source docs live in [`docs/`](/Users/au616397/Repositories/atomic-kernels/docs)
- Site configuration lives in [`docs/zensical.toml`](/Users/au616397/Repositories/atomic-kernels/docs/zensical.toml)
- Source Markdown lives in [`docs/src/`](/Users/au616397/Repositories/atomic-kernels/docs/src)
- Built site output is written to [`docs/site/`](/Users/au616397/Repositories/atomic-kernels/docs/site)
- Agent documentation conventions live in [`docs/src/agent-development.md`](/Users/au616397/Repositories/atomic-kernels/docs/src/agent-development.md)
- The running agent log lives in [`docs/src/agent-development-notes.md`](/Users/au616397/Repositories/atomic-kernels/docs/src/agent-development-notes.md)

## Running the docs locally

Install a docs toolchain and serve the site:

```bash
uv sync --group docs
just docs-serve
```

## Running Python tests locally

If you have [`just`](https://github.com/casey/just) installed, use:

```bash
just test
just viewer-test
```

`just test` runs the default Python-side suite without GUI viewer integration tests.
`just viewer-test` runs the real viewer smoke test with the required environment flags.

## Installing prebuilt wheels

Prebuilt wheels are currently published only for macOS Apple Silicon (`arm64`) on
Python 3.12.

Install from a GitHub Release asset:

```bash
pip install "https://github.com/au616397/atomic-kernels/releases/download/vX.Y.Z/atomic_kernels-X.Y.Z-cp312-cp312-macosx_14_0_arm64.whl"
```

If your platform or Python version is not covered by that wheel, use the source-based
development setup with `maturin`.

## Current docs scope

The initial docs focus on:

- project overview
- local development setup
- repository architecture
- agent-driven development conventions

## License

This project is licensed under the Apache License 2.0. See
[`LICENSE`](/Users/au616397/Repositories/atomic-kernels/LICENSE).
