use std::sync::Mutex;

use brk_error::Error;
use tokio::sync::watch;

/// The exact resource that prevented this nonblocking read attempt.
#[derive(Default)]
pub(crate) struct ReadAttempt(Mutex<Option<watch::Receiver<()>>>);

impl ReadAttempt {
    pub fn waiting_on(&self, changes: watch::Receiver<()>) -> Error {
        *self.0.lock().unwrap() = Some(changes);
        Error::StateUpdating
    }

    pub fn take(&self) -> Option<watch::Receiver<()>> {
        self.0.lock().unwrap().take()
    }
}
