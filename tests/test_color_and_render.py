from __future__ import annotations

import numpy as np
import pytest
from ase import Atoms

from atomic_kernels.viewer._color import ScalarRangeTracker
from atomic_kernels.viewer._session import ViewerSessionFacade


class BackendSpy:
    def __init__(self):
        self.calls = []

    def set_atom_scalars(self, name, values, frame_index=None):
        self.calls.append(("set_atom_scalars", name, values, frame_index))

    def color_by_scalar(
        self, name, palette="viridis", colors=None, min=None, max=None, append=False
    ):
        self.calls.append(
            ("color_by_scalar", name, palette, colors, min, max, append)
        )

    def reset_atom_colors(self):
        self.calls.append(("reset_atom_colors",))

    def set_faces(self, faces, color=(0.2, 0.6, 0.9, 0.35), face_colors=None, frame_index=None):
        self.calls.append(("set_faces", faces, color, face_colors, frame_index))

    def set_ball_and_stick_style(
        self,
        selection,
        atom_scale=0.45,
        bond_radius=0.08,
        bond_color=(0.7, 0.7, 0.7, 1.0),
        bond_scope="both_selected",
        frame_index=None,
        append=False,
    ):
        self.calls.append(
            (
                "set_ball_and_stick_style",
                selection,
                atom_scale,
                bond_radius,
                bond_color,
                bond_scope,
                frame_index,
                append,
            )
        )

    def reset_render_style(self):
        self.calls.append(("reset_render_style",))


def make_session():
    atoms = Atoms("H2O")
    return ViewerSessionFacade(BackendSpy(), [atoms])


def test_scalar_range_tracker_expands_over_multiple_updates():
    tracker = ScalarRangeTracker()

    assert tracker.update([1.0, np.nan, 3.0]) == (1.0, 3.0)
    assert tracker.update([-2.0, 5.0]) == (-2.0, 5.0)


def test_scalar_range_tracker_requires_finite_values():
    tracker = ScalarRangeTracker()

    with pytest.raises(ValueError, match="finite scalar"):
        tracker.update([np.nan, np.inf])


def test_selection_set_atom_scalars_masks_unselected_atoms():
    session = make_session()

    result = session.select([0, 2]).set_atom_scalars("charge", [1.0, -1.0])

    assert result is session
    call = session._backend.calls[0]
    assert call[:2] == ("set_atom_scalars", "charge")
    assert call[3] == 0
    assert call[2][0] == pytest.approx(1.0)
    assert np.isnan(call[2][1])
    assert call[2][2] == pytest.approx(-1.0)


def test_selection_color_by_scalar_appends_overlay():
    session = make_session()

    session.select([0, 2]).color_by_scalar("charge", [1.0, -1.0], palette="plasma")

    assert session._backend.calls[1] == (
        "color_by_scalar",
        "charge",
        "plasma",
        None,
        None,
        None,
        True,
    )


def test_ball_and_stick_selection_resolves_mask_and_normalizes_color():
    session = make_session()

    session.render().ball_and_stick(selection=[0, 2], bond_color=(0.1, 0.2, 0.3))

    assert session._backend.calls == [
        (
            "set_ball_and_stick_style",
            [True, False, True],
            0.45,
            0.08,
            pytest.approx((0.1, 0.2, 0.3, 1.0)),
            "both_selected",
            0,
            False,
        )
    ]


def test_set_faces_normalizes_polygons_and_broadcasts_color():
    session = make_session()

    session.render().set_faces([(0, 1, 2), (1, 2, 0)], color=(0.3, 0.4, 0.5))

    assert session._backend.calls == [
        (
            "set_faces",
            [[0, 1, 2]],
            pytest.approx((0.3, 0.4, 0.5, 1.0)),
            [pytest.approx((0.3, 0.4, 0.5, 1.0))],
            0,
        )
    ]
