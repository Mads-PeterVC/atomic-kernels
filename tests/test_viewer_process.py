from __future__ import annotations

from atomic_kernels.viewer._process import ViewerSessionProxy


class ConnectionSpy:
    def __init__(self):
        self.closed = False
        self.messages = []

    def send(self, payload):
        self.messages.append(payload)

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
