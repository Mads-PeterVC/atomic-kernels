import rich_click as click
from rich.console import Console
from rich.table import Table


_FEATURE_PACKAGES = {'viewer': 'ak_viewer', 'widget': 'ak_widget'}

@click.command(name="features")
def list_features_command():
    """List available features and their corresponding packages."""
    console = Console()
    table = Table(title="Available Features")
    table.add_column("Feature", style="cyan", no_wrap=True)
    table.add_column("Status", style="green")
    table.add_column("Package", style="magenta")

    for feature, package in _FEATURE_PACKAGES.items():

        # Check if package is available:
        try:
            __import__(package)
            status = "available"
        except ImportError:
            status = "not available"

        table.add_row(feature, status, package)

    console.print(table)


        
