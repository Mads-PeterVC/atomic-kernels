# Python Controllers

Viewer interaction is organized around focused controller objects returned from
the session facade.

## Camera

::: atomic_kernels.viewer._camera
    options:
      members:
        - CameraController

## Colors and Selection

::: atomic_kernels.viewer._color
    options:
      members:
        - ScalarRangeTracker
        - ColorController
        - ViewerSelection

## Rendering

::: atomic_kernels.viewer._render
    options:
      members:
        - RenderController
