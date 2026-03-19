default:
    @just --list

bump version:
    cargo set-version --workspace {{version}}

bump-patch:
    cargo set-version --workspace --bump patch

bump-minor:
    cargo set-version --workspace --bump minor

bump-major:
    cargo set-version --workspace --bump major

test:
    uv run --group test pytest tests -m "not viewer_integration"

build: 
    maturin develop --release

viewer-test:
    ATOMIC_KERNELS_USE_REAL_EXTENSION=1 ATOMIC_KERNELS_RUN_VIEWER_TESTS=1 uv run --group test pytest tests -m viewer_integration

headless-test:
    ATOMIC_KERNELS_RUN_RUST_HEADLESS_TESTS=1 cargo test -p ak-vis viewer::headless::tests
    ATOMIC_KERNELS_USE_REAL_EXTENSION=1 ATOMIC_KERNELS_RUN_VIEWER_TESTS=1 uv run --group test pytest tests/test_headless_render.py -m viewer_integration

docs-serve:
    cargo doc --no-deps -p ak-core -p ak-vis
    uv run zensical serve --config-file docs/zensical.toml -o 

docs-build:
    cargo doc --no-deps -p ak-core -p ak-vis
    uv run zensical build --config-file docs/zensical.toml
    python3 scripts/stage_rustdoc.py
