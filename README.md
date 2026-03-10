# atomic-kernels

`atomic-kernels` is a mixed Rust/Python project for atomistic structure tooling.
The current codebase combines:

- Rust crates for core geometry, visualization, and Python bindings
- A Python package built with `maturin`
- ASE-oriented scripting workflows for neighbor lists and interactive viewing

## Documentation

The project now uses a Markdown-first docs structure intended for `Zensical`.

- Source docs live in [`docs/`](/Users/au616397/Repositories/atomic-kernels/docs)
- Site configuration lives in [`zensical.toml`](/Users/au616397/Repositories/atomic-kernels/zensical.toml)
- Agent documentation conventions live in [`docs/agent-development.md`](/Users/au616397/Repositories/atomic-kernels/docs/agent-development.md)
- The running agent log lives in [`docs/agent-development-notes.md`](/Users/au616397/Repositories/atomic-kernels/docs/agent-development-notes.md)

## Running the docs locally

Install a docs toolchain and serve the site:

```bash
uv sync --group docs
uv run zensical serve
```

## Current docs scope

The initial docs focus on:

- project overview
- local development setup
- repository architecture
- agent-driven development conventions
