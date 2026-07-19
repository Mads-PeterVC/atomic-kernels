use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ViewerLifecycleState {
    Pending,
    Ready,
    Closed,
}

#[derive(Debug)]
pub struct ViewerReadiness {
    state: Mutex<ViewerLifecycleState>,
    changed: Condvar,
}

impl ViewerReadiness {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(ViewerLifecycleState::Pending),
            changed: Condvar::new(),
        }
    }

    pub fn mark_ready(&self) {
        let mut state = self.state.lock().expect("viewer readiness mutex poisoned");
        if *state == ViewerLifecycleState::Pending {
            *state = ViewerLifecycleState::Ready;
            self.changed.notify_all();
        }
    }

    pub fn mark_closed(&self) {
        let mut state = self.state.lock().expect("viewer readiness mutex poisoned");
        if *state != ViewerLifecycleState::Closed {
            *state = ViewerLifecycleState::Closed;
            self.changed.notify_all();
        }
    }

    pub fn wait(&self, timeout: Option<Duration>) -> bool {
        let mut state = self.state.lock().expect("viewer readiness mutex poisoned");

        match timeout {
            Some(timeout) => {
                let deadline = Instant::now() + timeout;
                while *state == ViewerLifecycleState::Pending {
                    let now = Instant::now();
                    if now >= deadline {
                        return false;
                    }

                    let remaining = deadline.saturating_duration_since(now);
                    let (next_state, result) = self
                        .changed
                        .wait_timeout(state, remaining)
                        .expect("viewer readiness mutex poisoned");
                    state = next_state;
                    if result.timed_out() && *state == ViewerLifecycleState::Pending {
                        return false;
                    }
                }
            }
            None => {
                while *state == ViewerLifecycleState::Pending {
                    state = self
                        .changed
                        .wait(state)
                        .expect("viewer readiness mutex poisoned");
                }
            }
        }

        *state == ViewerLifecycleState::Ready
    }
}

impl Default for ViewerReadiness {
    fn default() -> Self {
        Self::new()
    }
}
