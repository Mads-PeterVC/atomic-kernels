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