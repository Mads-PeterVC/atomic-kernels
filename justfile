default:
    @just --list

test:
    uv run --group test pytest tests -m "not viewer_integration"

build: 
    maturin develop --release

viewer-test:
    ATOMIC_KERNELS_USE_REAL_EXTENSION=1 ATOMIC_KERNELS_RUN_VIEWER_TESTS=1 uv run --group test pytest tests -m viewer_integration

headless-test:
    cargo test -p ak-vis viewer::headless::tests
    ATOMIC_KERNELS_USE_REAL_EXTENSION=1 ATOMIC_KERNELS_RUN_VIEWER_TESTS=1 uv run --group test pytest tests/test_headless_render.py -m viewer_integration
