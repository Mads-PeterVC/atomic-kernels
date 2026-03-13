from __future__ import annotations

from typing import Iterable


def _structure_to_world(vector: Iterable[float]) -> tuple[float, float, float]:
    x, y, z = (float(component) for component in vector)
    return (x, z, -y)


class CameraController:
    """Control the viewer camera in structure-space coordinates.

    The viewer renders chemistry-style structure coordinates with ``z`` as the
    semantic up axis. Internally the Bevy camera still uses its own world
    convention, so focus points and pan vectors are rotated before being sent to
    the low-level backend.
    """

    def __init__(self, session: "ViewerSessionFacade") -> None:
        self._session = session

    def look_at(
        self,
        focus,
        radius: float | None = None,
        yaw: float | None = None,
        pitch: float | None = None,
    ) -> None:
        """Set the camera focus and optionally update view parameters.

        Parameters
        ----------
        focus
            Structure-space ``(x, y, z)`` focus point.
        radius : float or None, optional
            Camera distance from the focus point.
        yaw : float or None, optional
            Orbit angle around the structure ``z`` axis, in radians.
        pitch : float or None, optional
            Tilt away from the structure ``xy`` plane, in radians.
        """
        self._session._backend.set_camera_view(
            _structure_to_world(focus), radius, yaw, pitch
        )

    def set_focus(self, focus) -> None:
        """Move the camera focus to a new structure-space point.

        Parameters
        ----------
        focus
            Structure-space ``(x, y, z)`` focus point.
        """
        self._session._backend.set_camera_view(
            _structure_to_world(focus), None, None, None
        )

    def set_rotation(
        self, yaw: float | None = None, pitch: float | None = None
    ) -> None:
        """Set the camera yaw and pitch in structure-space terms.

        Parameters
        ----------
        yaw : float or None, optional
            Orbit angle around the structure ``z`` axis, in radians.
        pitch : float or None, optional
            Tilt away from the structure ``xy`` plane around structure ``x``, in radians.
        """
        self._session._backend.set_camera_view(None, None, yaw, pitch)

    def set_radius(self, radius: float) -> None:
        """Set the camera distance from the current focus point.

        Parameters
        ----------
        radius : float
            Camera distance from the current focus point.
        """
        self._session._backend.set_camera_view(None, radius, None, None)

    def pan(self, delta) -> None:
        """Translate the camera focus by a structure-space delta.

        Parameters
        ----------
        delta
            Structure-space ``(x, y, z)`` translation applied to the focus point.
        """
        self._session._backend.pan_camera(_structure_to_world(delta))

    def zoom(
        self, factor: float | None = None, delta: float | None = None
    ) -> None:
        """Zoom by a multiplicative factor or additive radius delta.

        Parameters
        ----------
        factor : float or None, optional
            Multiplicative zoom factor.
        delta : float or None, optional
            Additive change in camera radius.
        """
        if (factor is None) == (delta is None):
            raise ValueError("zoom requires exactly one of factor or delta")
        self._session._backend.zoom_camera(factor=factor, delta=delta)

    def orbit(self, yaw_delta: float = 0.0, pitch_delta: float = 0.0) -> None:
        """Apply an immediate orbit step.

        Parameters
        ----------
        yaw_delta : float, default=0.0
            Increment around the structure ``z`` axis, in radians.
        pitch_delta : float, default=0.0
            Increment away from the structure ``xy`` plane, in radians.
        """
        self._session._backend.orbit_camera(yaw_delta, pitch_delta)

    def frame_all(self) -> None:
        """Reset the camera to frame the whole current structure."""
        self._session._backend.frame_all()

    def start_orbit(self, yaw_rate: float = 0.5, pitch_rate: float = 0.0) -> None:
        """Start continuous orbit motion.

        Parameters
        ----------
        yaw_rate : float, default=0.5
            Continuous rotation rate around the structure ``z`` axis, in radians per second.
        pitch_rate : float, default=0.0
            Continuous tilt rate, in radians per second.
        """
        self._session._backend.start_orbit(yaw_rate, pitch_rate)

    def stop(self) -> None:
        """Stop any active camera motion started by ``start_orbit``."""
        self._session._backend.stop_camera_motion()
