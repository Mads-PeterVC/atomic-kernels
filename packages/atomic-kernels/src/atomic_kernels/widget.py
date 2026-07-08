try:
    from ak_widget.widget import ViewerWidget
    __all__ = ["ViewerWidget"]

except ImportError as exc:
    raise ImportError(
        "Widget support is not installed. Install with "
        "`pip install 'atomic-kernels[widget]'`."
    ) from exc