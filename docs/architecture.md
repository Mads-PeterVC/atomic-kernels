# Architecture

## Workspace layout

The repository is organized as a Cargo workspace with Python bindings layered on top.

## Rust crates

### `ak-core`

Core structure and geometry functionality:

- structure and structure views
- cell and periodic boundary handling
- neighbor list logic
- calculator utilities
- XYZ I/O support

### `ak-vis`

Visualization and interactive viewing:

- atom, bond, cell, and axis visuals
- rendering systems
- viewer app, controls, and sessions
- example scenes and structure assets

### `ak-py`

Python integration boundary:

- conversion from ASE-like inputs into Rust-side structures
- bound Python functions for neighbor lists, viewers, and trajectories
- Python-visible viewer configuration types

### `ak-cli`

CLI surface area for standalone workflows.

## Python package

`python/atomic_kernels` provides the user-facing Python API. At the moment it mainly
does two things:

- re-export Rust-backed kernels
- add higher-level viewer helpers and session facades around the low-level bindings

## Scripts

The `scripts/` directory currently acts as executable documentation. Many new features
appear there first as focused workflows before they are promoted into more stable API
surfaces or formal docs pages.
