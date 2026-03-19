from __future__ import annotations

import pytest
from ase import Atoms

from atomic_kernels.neighbor_list import NeighborList
from atomic_kernels.viewer._camera import CameraController, _structure_to_world
from atomic_kernels.viewer._session import ViewerSessionFacade


class BackendSpy:
    def __init__(self):
        self.calls = []
        self.selection = []
        self.image_selection = []
        self.supercell_state = ((0, 0, 0), True)

    def append_frame(self, frame):
        self.calls.append(("append_frame", frame))

    def set_frame(self, index):
        self.calls.append(("set_frame", index))

    def follow_tail(self, enabled):
        self.calls.append(("follow_tail", enabled))

    def close(self):
        self.calls.append(("close",))

    def set_bonds(self, bonds, frame_index=None):
        self.calls.append(("set_bonds", bonds, frame_index))

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

    def selected_atoms(self, frame_index=None):
        self.calls.append(("selected_atoms", frame_index))
        return list(self.selection)

    def set_selection(self, selection, frame_index=None):
        self.calls.append(("set_selection", selection, frame_index))

    def add_selection(self, selection, frame_index=None):
        self.calls.append(("add_selection", selection, frame_index))

    def remove_selection(self, selection, frame_index=None):
        self.calls.append(("remove_selection", selection, frame_index))

    def clear_selection(self, frame_index=None):
        self.calls.append(("clear_selection", frame_index))

    def selected_images(self, frame_index=None):
        self.calls.append(("selected_images", frame_index))
        return list(self.image_selection)

    def set_image_selection(self, selection, frame_index=None):
        self.calls.append(("set_image_selection", selection, frame_index))

    def add_image_selection(self, selection, frame_index=None):
        self.calls.append(("add_image_selection", selection, frame_index))

    def remove_image_selection(self, selection, frame_index=None):
        self.calls.append(("remove_image_selection", selection, frame_index))

    def clear_image_selection(self, frame_index=None):
        self.calls.append(("clear_image_selection", frame_index))

    def set_supercell(self, repeats):
        self.calls.append(("set_supercell", repeats))

    def increment_supercell_axis(self, axis):
        self.calls.append(("increment_supercell_axis", axis))

    def decrement_supercell_axis(self, axis):
        self.calls.append(("decrement_supercell_axis", axis))

    def reset_supercell(self):
        self.calls.append(("reset_supercell",))

    def set_ghost_repeated_images(self, enabled=True):
        self.calls.append(("set_ghost_repeated_images", enabled))

    def toggle_ghost_repeated_images(self):
        self.calls.append(("toggle_ghost_repeated_images",))

    def supercell(self):
        self.calls.append(("supercell",))
        return self.supercell_state

    def set_camera_view(self, focus, radius, yaw, pitch):
        self.calls.append(("set_camera_view", focus, radius, yaw, pitch))

    def pan_camera(self, delta):
        self.calls.append(("pan_camera", delta))

    def zoom_camera(self, factor=None, delta=None):
        self.calls.append(("zoom_camera", factor, delta))

    def orbit_camera(self, yaw_delta, pitch_delta):
        self.calls.append(("orbit_camera", yaw_delta, pitch_delta))

    def frame_all(self):
        self.calls.append(("frame_all",))

    def start_orbit(self, yaw_rate, pitch_rate):
        self.calls.append(("start_orbit", yaw_rate, pitch_rate))

    def stop_camera_motion(self):
        self.calls.append(("stop_camera_motion",))


def test_structure_to_world_rotates_axes_for_viewer_backend():
    assert _structure_to_world((1, 2, 3)) == (1.0, 3.0, -2.0)


def test_camera_controller_translates_structure_space_calls():
    session = type("Session", (), {"_backend": BackendSpy()})()
    camera = CameraController(session)

    camera.look_at((1, 2, 3), radius=4.0, yaw=0.5, pitch=0.25)
    camera.pan((3, 2, 1))
    camera.set_focus((0, 1, 2))

    assert session._backend.calls == [
        ("set_camera_view", (1.0, 3.0, -2.0), 4.0, 0.5, 0.25),
        ("pan_camera", (3.0, 1.0, -2.0)),
        ("set_camera_view", (0.0, 2.0, -1.0), None, None, None),
    ]


def test_camera_zoom_requires_exactly_one_mode():
    session = type("Session", (), {"_backend": BackendSpy()})()
    camera = CameraController(session)

    with pytest.raises(ValueError, match="exactly one"):
        camera.zoom()

    with pytest.raises(ValueError, match="exactly one"):
        camera.zoom(factor=1.1, delta=0.2)


def test_viewer_session_facade_copies_frames_and_tracks_current_index():
    first = Atoms("H2", positions=[(0, 0, 0), (0, 0, 1)])
    second = Atoms("He", positions=[(1, 0, 0)])
    backend = BackendSpy()
    facade = ViewerSessionFacade(backend, [first])

    facade.append_frame(second)
    second.positions[0, 0] = 99.0
    facade.set_frame(1)

    assert backend.calls[:2] == [
        ("append_frame", second),
        ("set_frame", 1),
    ]
    assert facade._frame(1).positions[0, 0] == pytest.approx(1.0)


def test_viewer_session_facade_wait_until_ready_delegates_to_backend():
    class ReadyBackend(BackendSpy):
        def wait_until_ready(self, timeout):
            self.calls.append(("wait_until_ready", timeout))
            return True

    backend = ReadyBackend()
    facade = ViewerSessionFacade(backend, [Atoms("H")])

    assert facade.wait_until_ready(timeout=1.5) is True
    assert backend.calls == [("wait_until_ready", 1.5)]


def test_viewer_session_facade_selection_methods_normalize_masks():
    atoms = Atoms("H2O")
    backend = BackendSpy()
    backend.selection = [2]
    facade = ViewerSessionFacade(backend, [atoms])

    assert facade.selected_atoms() == [2]

    facade.set_selection([0, 2])
    facade.add_selection(lambda frame: [atom.symbol == "O" for atom in frame])
    facade.remove_selection([2])
    facade.clear_selection()

    assert backend.calls == [
        ("selected_atoms", 0),
        ("set_selection", [True, False, True], 0),
        ("add_selection", [False, False, True], 0),
        ("remove_selection", [False, False, True], 0),
        ("clear_selection", 0),
    ]


def test_viewer_session_facade_exposes_image_selection_and_supercell_controls():
    atoms = Atoms("H2")
    backend = BackendSpy()
    backend.image_selection = [(1, (1, 0, 0))]
    backend.supercell_state = ((2, 1, 0), False)
    facade = ViewerSessionFacade(backend, [atoms])

    assert facade.selected_images() == [{"atom_index": 1, "image_offset": (1, 0, 0)}]

    facade.set_image_selection([(0, (0, 0, 0))])
    facade.add_image_selection([(1, (1, 0, 0))])
    facade.remove_image_selection([(1, (1, 0, 0))])
    facade.clear_image_selection()
    facade.set_supercell((2, 1, 0))
    facade.increment_supercell_axis(0)
    facade.decrement_supercell_axis(1)
    facade.reset_supercell()
    facade.set_ghost_repeated_images(False)
    facade.toggle_supercell_distinction()
    assert facade.supercell() == {
        "repeats": (2, 1, 0),
        "ghost_repeated_images": False,
    }

    assert backend.calls == [
        ("selected_images", 0),
        ("set_image_selection", [(0, (0, 0, 0))], 0),
        ("add_image_selection", [(1, (1, 0, 0))], 0),
        ("remove_image_selection", [(1, (1, 0, 0))], 0),
        ("clear_image_selection", 0),
        ("set_supercell", (2, 1, 0)),
        ("increment_supercell_axis", 0),
        ("decrement_supercell_axis", 1),
        ("reset_supercell",),
        ("set_ghost_repeated_images", False),
        ("toggle_ghost_repeated_images",),
        ("supercell",),
    ]


def test_render_controller_accepts_neighbor_list_shortcut_for_bonds():
    atoms = Atoms("H2O")
    backend = BackendSpy()
    facade = ViewerSessionFacade(backend, [atoms])

    facade.render().set_bonds(NeighborList(i=[2, 0, 1], j=[0, 2, 1], S=[]))

    assert backend.calls == [("set_bonds", [(0, 2)], None)]
