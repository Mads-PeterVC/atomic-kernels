from __future__ import annotations


class CameraController:
    """Control the viewer camera from Python."""

    def __init__(self, session: "ViewerSessionFacade") -> None:
        self._session = session

    def look_at(
        self,
        focus,
        radius: float | None = None,
        yaw: float | None = None,
        pitch: float | None = None,
    ) -> None:
        """Set the camera focus and optionally radius and orientation."""
        self._session._backend.set_camera_view(tuple(focus), radius, yaw, pitch)

    def set_focus(self, focus) -> None:
        """Move the camera focus to a new world-space point."""
        self._session._backend.set_camera_view(tuple(focus), None, None, None)

    def set_rotation(
        self, yaw: float | None = None, pitch: float | None = None
    ) -> None:
        """Set the camera yaw and/or pitch in radians."""
        self._session._backend.set_camera_view(None, None, yaw, pitch)

    def set_radius(self, radius: float) -> None:
        """Set the camera distance from the current focus point."""
        self._session._backend.set_camera_view(None, radius, None, None)

    def pan(self, delta) -> None:
        """Translate the camera focus by a world-space XYZ delta."""
        self._session._backend.pan_camera(tuple(delta))

    def zoom(
        self, factor: float | None = None, delta: float | None = None
    ) -> None:
        """Zoom by a multiplicative factor or additive radius delta."""
        if (factor is None) == (delta is None):
            raise ValueError("zoom requires exactly one of factor or delta")
        self._session._backend.zoom_camera(factor=factor, delta=delta)

    def orbit(self, yaw_delta: float = 0.0, pitch_delta: float = 0.0) -> None:
        """Apply an immediate orbit step in radians around the current focus."""
        self._session._backend.orbit_camera(yaw_delta, pitch_delta)

    def frame_all(self) -> None:
        """Reset the camera to frame the whole current structure."""
        self._session._backend.frame_all()

    def start_orbit(self, yaw_rate: float = 0.5, pitch_rate: float = 0.0) -> None:
        """Start continuous orbit motion handled by the viewer runtime."""
        self._session._backend.start_orbit(yaw_rate, pitch_rate)

    def stop(self) -> None:
        """Stop any active camera motion started by ``start_orbit``."""
        self._session._backend.stop_camera_motion()
