# Rust API

The public Rust API is split across two crates:

- `ak-core` for structures, trajectories, geometry, neighbor lists, and
  calculator primitives.
- `ak-vis` for visuals, rendering helpers, viewer sessions, and headless image
  export.

## Reference

- [ak-core rustdoc](rustdoc/ak_core/index.html)
- [ak-vis rustdoc](rustdoc/ak_vis/index.html)

## Public entry points

The crate roots are the best place to start when navigating the Rust API:

- [`crates/ak-core/src/lib.rs`](/Users/au616397/Repositories/atomic-kernels/crates/ak-core/src/lib.rs)
- [`crates/ak-vis/src/lib.rs`](/Users/au616397/Repositories/atomic-kernels/crates/ak-vis/src/lib.rs)

From there, the main public modules are:

- `ak_core::geometry` for `Structure`, `Trajectory`, `Cell`, `Pbc`,
  `NeighborList`, and related helpers.
- `ak_core::calculator` for calculator traits and the Lennard-Jones example
  implementation.
- `ak_vis::viewer` for viewer configuration, live sessions, and headless export.
- `ak_vis::visuals` and `ak_vis::render` for explicit scene primitives and
  rendering utilities.
 
