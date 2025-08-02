use std::{process::exit, sync::Arc};

use log::info;
use parking_lot::{RwLock, RwLockReadGuard};

#[derive(Default, Clone)]
pub struct Exit(Arc<RwLock<()>>);

impl Exit {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(())))
    }

    pub fn set_ctrlc_handler(&self) {
        let copy = self.0.clone();

        ctrlc::set_handler(move || {
            if copy.is_locked() {
                info!("Waiting to exit safely...");
            }
            let _lock = copy.write();
            info!("Exiting...");
            exit(0);
        })
        .expect("Error setting Ctrl-C handler");
    }

    pub fn lock(&self) -> RwLockReadGuard<'_, ()> {
        self.0.read()
    }
}
