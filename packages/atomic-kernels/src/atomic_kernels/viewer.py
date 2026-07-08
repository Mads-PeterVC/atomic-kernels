# atomic_kernels/viewer.py or atomic_kernels/viewer/__init__.py
try:
    from ak_viewer import *
    from ak_viewer import ColorConfig, LightingConfig, ViewerConfig, RenderConfig
    from ak_viewer.viewer import QualityPreset


except ImportError as exc:
    raise ImportError(
        "Viewer support is not installed. Install with "
        "`pip install 'atomic-kernels[viewer]'`."
    ) from exc