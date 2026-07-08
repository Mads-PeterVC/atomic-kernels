from __future__ import annotations

import numpy as np
import pytest
from ase import Atoms

from ak_viewer.viewer import bonds_from_ase, faces_from_coordination
from ak_viewer.viewer._color import ScalarRangeTracker
from ak_viewer.viewer._session import ViewerSessionFacade


class BackendSpy:
    def __init__(self):
        self.calls = []

    def set_atom_scalars(self, name, values, frame_index=None):
        self.calls.append(("set_atom_scalars", name, values, frame_index))

    def material_by_scalar(
        self,
        name,
        channel,
        palette="viridis",
        colors=None,
        min=None,
        max=None,
        append=False,
    ):
        self.calls.append(
            ("material_by_scalar", name, channel, palette, colors, min, max, append)
        )

    def reset_atom_materials(self, channel=None):
        self.calls.append(("reset_atom_materials", channel))

    def set_faces(self, faces, color=(0.2, 0.6, 0.9, 0.35), face_colors=None, frame_index=None):
        self.calls.append(("set_faces", faces, color, face_colors, frame_index))

    def add_faces(self, faces, color=(0.2, 0.6, 0.9, 0.35), face_colors=None, frame_index=None):
        self.calls.append(("add_faces", faces, color, face_colors, frame_index))

    def remove_faces(
        self, faces, color=(0.2, 0.6, 0.9, 0.35), face_colors=None, frame_index=None
    ):
        self.calls.append(("remove_faces", faces, color, face_colors, frame_index))

    def clear_faces(self, frame_index=None):
        self.calls.append(("clear_faces", frame_index))

    def set_bonds(self, bonds, frame_index=None):
        self.calls.append(("set_bonds", bonds, frame_index))

    def add_bonds(self, bonds, frame_index=None):
        self.calls.append(("add_bonds", bonds, frame_index))

    def remove_bonds(self, bonds, frame_index=None):
        self.calls.append(("remove_bonds", bonds, frame_index))

    def clear_bonds(self, frame_index=None):
        self.calls.append(("clear_bonds", frame_index))

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

    def selected_atoms(self, frame_index=None):
        self.calls.append(("selected_atoms", frame_index))
        return []

    def set_selection(self, selection, frame_index=None):
        self.calls.append(("set_selection", selection, frame_index))

    def add_selection(self, selection, frame_index=None):
        self.calls.append(("add_selection", selection, frame_index))

    def remove_selection(self, selection, frame_index=None):
        self.calls.append(("remove_selection", selection, frame_index))

    def clear_selection(self, frame_index=None):
        self.calls.append(("clear_selection", frame_index))


def make_session():
    atoms = Atoms(
        "H2O",
        positions=[
            [0.0, 0.76, 0.0],
            [0.0, -0.76, 0.0],
            [0.0, 0.0, 0.0],
        ],
    )
    return ViewerSessionFacade(BackendSpy(), [atoms])


def make_tetrahedral_session():
    atoms = Atoms(
        symbols=["Ti", "O", "O", "O", "O"],
        positions=np.array(
            [
                [0.0, 0.0, 0.0],
                [1.0, 1.0, 1.0],
                [1.0, -1.0, -1.0],
                [-1.0, 1.0, -1.0],
                [-1.0, -1.0, 1.0],
            ]
        ),
        cell=[12.0, 12.0, 12.0],
        pbc=False,
    )
    atoms.center()
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


def test_selection_material_by_scalar_appends_overlay():
    session = make_session()

    session.select([0, 2]).material_by_scalar(
        "charge", [1.0, -1.0], channel="color", palette="plasma"
    )

    assert session._backend.calls[1] == (
        "material_by_scalar",
        "charge",
        "color",
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


def test_set_bonds_default_generates_expected_water_connectivity():
    session = make_session()

    session.render().set_bonds(mode="default")

    assert session._backend.calls == [("set_bonds", [(0, 2), (1, 2)], 0)]


def test_set_bonds_default_respects_selection_subgraph():
    session = make_session()

    session.render().set_bonds(mode="default", selection=[0, 2])

    assert session._backend.calls == [("set_bonds", [(0, 2)], 0)]


def test_set_bonds_rejects_mixed_explicit_and_default_inputs():
    session = make_session()

    with pytest.raises(ValueError, match="explicit bonds cannot be combined"):
        session.render().set_bonds([(0, 2)], mode="default")


def test_add_remove_and_clear_bonds_forward_normalized_payloads():
    session = make_session()

    session.render().add_bonds([(2, 0), (0, 2)])
    session.render().remove_bonds([(0, 2)])
    session.render().clear_bonds()

    assert session._backend.calls == [
        ("add_bonds", [(0, 2)], None),
        ("remove_bonds", [(0, 2)], None),
        ("clear_bonds", None),
    ]


def test_bonds_from_ase_matches_water_connectivity():
    atoms = make_session()._frame()

    assert bonds_from_ase(atoms) == [(0, 2), (1, 2)]


def test_set_faces_default_generates_tetrahedral_shell_faces():
    session = make_tetrahedral_session()

    session.render().set_faces(mode="default", selection=[0])

    call = session._backend.calls[0]
    assert call[0] == "set_faces"
    assert sorted(call[1]) == [[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]]
    assert call[4] == 0


def test_set_faces_default_skips_centers_without_polyhedron():
    session = make_tetrahedral_session()

    session.render().set_faces(mode="default", selection=[1])

    assert session._backend.calls == [
        ("set_faces", [], pytest.approx((0.2, 0.6, 0.9, 0.35)), [], 0)
    ]


def test_set_faces_rejects_mixed_explicit_and_default_inputs():
    session = make_tetrahedral_session()

    with pytest.raises(ValueError, match="explicit faces cannot be combined"):
        session.render().set_faces([[1, 2, 3]], mode="default")


def test_add_remove_and_clear_faces_forward_normalized_payloads():
    session = make_tetrahedral_session()

    session.render().add_faces([(1, 2, 3), (2, 3, 1)], color=(0.3, 0.4, 0.5))
    session.render().remove_faces([(1, 2, 3)], color=(0.1, 0.2, 0.3))
    session.render().clear_faces()

    assert session._backend.calls == [
        (
            "add_faces",
            [[1, 2, 3]],
            pytest.approx((0.3, 0.4, 0.5, 1.0)),
            [pytest.approx((0.3, 0.4, 0.5, 1.0))],
            None,
        ),
        (
            "remove_faces",
            [[1, 2, 3]],
            pytest.approx((0.1, 0.2, 0.3, 1.0)),
            [pytest.approx((0.1, 0.2, 0.3, 1.0))],
            None,
        ),
        ("clear_faces", None),
    ]


def test_faces_from_coordination_builds_tetrahedral_faces():
    session = make_tetrahedral_session()

    faces = faces_from_coordination(session._frame(), selection=[0])

    assert sorted(faces) == [[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]]
