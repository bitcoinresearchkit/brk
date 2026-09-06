use std::sync::Arc;

use brk_types::Lengths;
use parking_lot::RwLock;

#[derive(Clone)]
pub struct State(pub Arc<RwLock<Lengths>>);

impl State {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Lengths::default())))
    }

    pub fn lengths(&self) -> Lengths {
        // Helpers may load bounds while their caller pins the prefix. Do not
        // deadlock behind a writer waiting for that caller's read guard.
        *self.0.read_recursive()
    }

    pub fn finish_update(&self, next: Lengths) {
        let mut lengths = self.0.write();
        debug_assert!(
            {
                let mut clamped = next;
                clamped.clamp_to(&lengths);
                clamped == *lengths
            },
            "length regression"
        );
        *lengths = next;
    }

    pub fn lower_before(&self, starting: &Lengths) {
        self.0.write().clamp_to(starting);
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread, time::Duration};

    use brk_types::Height;

    use super::*;

    #[test]
    fn pinned_prefix_blocks_rollback_and_allows_nested_bound_reads() {
        let state = State::new();
        state.finish_update(Lengths {
            height: Height::new(2),
            ..Default::default()
        });
        let prefix = state.pin();
        let writer = state.clone();
        let (started, ready) = mpsc::channel();
        let (finished, done) = mpsc::channel();
        let task = thread::spawn(move || {
            started.send(()).unwrap();
            writer.lower_before(&Lengths::default());
            finished.send(()).unwrap();
        });
        ready.recv().unwrap();
        assert!(done.recv_timeout(Duration::from_millis(50)).is_err());
        assert!(
            state.try_pin().is_none(),
            "a queued writer has priority over new readers"
        );
        assert_eq!(prefix.lengths().height, Height::new(2));
        assert_eq!(state.lengths().height, Height::new(2));
        drop(prefix);
        done.recv_timeout(Duration::from_secs(2)).unwrap();
        task.join().unwrap();
        assert_eq!(state.lengths().height, Height::ZERO);
    }

    #[test]
    fn lifecycle_publishes_lengths() {
        let state = State::new();
        state.finish_update(Lengths::default());

        let lengths = Lengths {
            height: Height::new(1),
            ..Default::default()
        };
        state.finish_update(lengths);
        assert_eq!(state.lengths(), lengths);
    }

    #[test]
    fn lower_before_clamps_published_lengths() {
        let state = State::new();
        state.finish_update(Lengths {
            height: Height::new(1),
            ..Default::default()
        });
        state.lower_before(&Lengths::default());

        assert_eq!(state.lengths(), Lengths::default());
    }
}
