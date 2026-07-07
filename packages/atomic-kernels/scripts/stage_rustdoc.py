#!/usr/bin/env python3
"""Stage generated rustdoc output into the MkDocs site directory."""

from __future__ import annotations

import shutil
from pathlib import Path


def find_repo_root(start: Path) -> Path:
    for candidate in (start, *start.parents):
        if (candidate / "Cargo.toml").exists() and (candidate / "docs").is_dir():
            return candidate
    raise SystemExit("repository root not found")


ROOT = find_repo_root(Path(__file__).resolve())
RUSTDOC_SOURCE = ROOT / "target" / "doc"
RUSTDOC_DESTINATION = ROOT / "docs" / "site" / "api" / "rustdoc"


def main() -> None:
    if not RUSTDOC_SOURCE.exists():
        raise SystemExit(
            "rustdoc output not found; run `cargo doc --no-deps -p ak-core -p ak-vis` first"
        )

    if RUSTDOC_DESTINATION.exists():
        shutil.rmtree(RUSTDOC_DESTINATION)

    RUSTDOC_DESTINATION.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(RUSTDOC_SOURCE, RUSTDOC_DESTINATION)


if __name__ == "__main__":
    main()
