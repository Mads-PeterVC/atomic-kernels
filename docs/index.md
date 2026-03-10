# atomic-kernels

`atomic-kernels` is a Rust-first atomistic tooling project with Python bindings and
viewer workflows built around ASE structures.

## What is here

- `crates/ak-core`: shared geometry and numerical primitives
- `crates/ak-vis`: rendering and viewer functionality
- `crates/ak-py`: Python bindings exposed through `maturin`
- `crates/ak-cli`: command-line entry points
- `python/atomic_kernels`: Python-side API and viewer session helpers
- `scripts/`: usage-oriented examples and smoke tests

## Why Zensical here

This repository benefits from Markdown-native docs more than a heavier API-doc-first
system. `Zensical` keeps the authoring model simple and is a good fit for architecture
notes, workflow guides, and the incremental `Agent Development` log.

For this project, that is a better default than:

- `Sphinx`, which is stronger when generated API reference is the center of the docs
  system
- `Quarto`, which is stronger when the docs are closer to notebooks, reports, or
  publishing workflows
