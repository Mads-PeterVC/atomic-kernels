from __future__ import annotations

import os

from ak_viewer.viewer._process import ViewerSessionProxy


class ConnectionSpy:
    def __init__(self):
        self.closed = False
        self.messages = []
        self._responses = []

    def send(self, payload):
        self.messages.append(payload)

    def poll(self, timeout=None):
        return bool(self._responses)

    def recv(self):
        return self._responses.pop(0)

    def close(self):
        self.closed = True


class ProcessSpy:
    def __init__(self, *, alive=True, join_outcomes=()):
        self.alive = alive
        self.join_outcomes = list(join_outcomes)
        self.join_calls = []
        self.terminate_calls = 0

    def is_alive(self):
        return self.alive

    def join(self, timeout):
        self.join_calls.append(timeout)
        if self.join_outcomes:
            self.alive = self.join_outcomes.pop(0)

    def terminate(self):
        self.terminate_calls += 1
        self.alive = False


def test_viewer_session_proxy_close_joins_process_after_sending_close():
    connection = ConnectionSpy()
    process = ProcessSpy(alive=True, join_outcomes=[False])
    proxy = ViewerSessionProxy(connection, process)

    proxy.close()

    assert connection.messages == [("close", None)]
    assert connection.closed is True
    assert process.join_calls == [proxy._close_timeout]
    assert process.terminate_calls == 0


def test_viewer_session_proxy_close_terminates_stuck_process():
    connection = ConnectionSpy()
    process = ProcessSpy(alive=True, join_outcomes=[True])
    proxy = ViewerSessionProxy(connection, process)

    proxy.close()

    assert connection.messages == [("close", None)]
    assert process.join_calls == [proxy._close_timeout, proxy._close_timeout]
    assert process.terminate_calls == 1


def test_viewer_session_proxy_wait_until_ready_uses_ci_process_liveness(monkeypatch):
    monkeypatch.setenv("CI", "true")
    connection = ConnectionSpy()
    process = ProcessSpy(alive=True)
    proxy = ViewerSessionProxy(connection, process)
    proxy._ci_startup_grace = 0.0

    assert proxy.wait_until_ready(timeout=1.0) is True
    assert connection.messages == []


def test_viewer_session_proxy_wait_until_ready_uses_explicit_response_outside_ci(
    monkeypatch,
):
    monkeypatch.delenv("CI", raising=False)
    connection = ConnectionSpy()
    connection._responses.append(("wait_until_ready", True))
    process = ProcessSpy(alive=True)
    proxy = ViewerSessionProxy(connection, process)

    assert proxy.wait_until_ready(timeout=1.0) is True
    assert connection.messages == [("wait_until_ready", 1.0)]


def test_viewer_session_proxy_selected_atoms_round_trips_response():
    connection = ConnectionSpy()
    connection._responses.append(("selected_atoms", [1, 3]))
    process = ProcessSpy(alive=True)
    proxy = ViewerSessionProxy(connection, process)

    assert proxy.selected_atoms(frame_index=2) == [1, 3]
    assert connection.messages == [("selected_atoms", 2)]


def test_viewer_session_proxy_selection_commands_forward_payloads():
    connection = ConnectionSpy()
    process = ProcessSpy(alive=True)
    proxy = ViewerSessionProxy(connection, process)

    proxy.set_selection([True, False], frame_index=0)
    proxy.add_selection([False, True], frame_index=1)
    proxy.remove_selection([True, False], frame_index=1)
    proxy.clear_selection(frame_index=2)

    assert connection.messages == [
        ("set_selection", ([True, False], 0)),
        ("add_selection", ([False, True], 1)),
        ("remove_selection", ([True, False], 1)),
        ("clear_selection", 2),
    ]


def test_viewer_session_proxy_topology_commands_forward_payloads():
    connection = ConnectionSpy()
    process = ProcessSpy(alive=True)
    proxy = ViewerSessionProxy(connection, process)

    proxy.add_bonds([(0, 1)], frame_index=0)
    proxy.remove_bonds([(0, 1)], frame_index=1)
    proxy.clear_bonds(frame_index=2)
    proxy.add_faces([(0, 1, 2)], frame_index=0)
    proxy.remove_faces([(0, 1, 2)], frame_index=1)
    proxy.clear_faces(frame_index=2)

    assert connection.messages == [
        ("add_bonds", ([(0, 1)], 0)),
        ("remove_bonds", ([(0, 1)], 1)),
        ("clear_bonds", 2),
        ("add_faces", ([(0, 1, 2)], (0.2, 0.6, 0.9, 0.35), None, 0)),
        ("remove_faces", ([(0, 1, 2)], (0.2, 0.6, 0.9, 0.35), None, 1)),
        ("clear_faces", 2),
    ]


def test_viewer_session_proxy_supercell_and_image_selection_commands_forward_payloads():
    connection = ConnectionSpy()
    connection._responses.extend(
        [
            ("selected_images", [(1, (1, 0, 0))]),
            ("supercell", ((2, 0, 0), False)),
        ]
    )
    process = ProcessSpy(alive=True)
    proxy = ViewerSessionProxy(connection, process)

    assert proxy.selected_images(frame_index=1) == [(1, (1, 0, 0))]
    proxy.set_image_selection([(0, (0, 0, 0))], frame_index=0)
    proxy.add_image_selection([(1, (1, 0, 0))], frame_index=0)
    proxy.remove_image_selection([(1, (1, 0, 0))], frame_index=0)
    proxy.clear_image_selection(frame_index=0)
    proxy.set_supercell((2, 0, 0))
    proxy.increment_supercell_axis(0)
    proxy.decrement_supercell_axis(1)
    proxy.reset_supercell()
    proxy.set_ghost_repeated_images(False)
    proxy.toggle_ghost_repeated_images()
    assert proxy.supercell() == ((2, 0, 0), False)

    assert connection.messages == [
        ("selected_images", 1),
        ("set_image_selection", ([(0, (0, 0, 0))], 0)),
        ("add_image_selection", ([(1, (1, 0, 0))], 0)),
        ("remove_image_selection", ([(1, (1, 0, 0))], 0)),
        ("clear_image_selection", 0),
        ("set_supercell", (2, 0, 0)),
        ("increment_supercell_axis", 0),
        ("decrement_supercell_axis", 1),
        ("reset_supercell", None),
        ("set_ghost_repeated_images", False),
        ("toggle_ghost_repeated_images", None),
        ("supercell", None),
    ]
