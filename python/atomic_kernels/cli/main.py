"""Top-level Click entry point for atomic-kernels."""

from __future__ import annotations

import rich_click as click

from .view import view_command

click.rich_click.COMMAND_GROUPS = {
    "ak": [
        {
            "name": "Commands",
            "commands": ["view"],
        }
    ]
}


@click.group(name="ak")
def main() -> None:
    """Atomic Kernels command-line tools."""


main.add_command(view_command, name="view")


if __name__ == "__main__":
    main()
