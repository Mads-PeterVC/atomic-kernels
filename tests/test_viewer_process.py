from __future__ import annotations

import os

from atomic_kernels.viewer._process import ViewerSessionProxy


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
