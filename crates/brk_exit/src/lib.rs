#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::{process::exit, sync::Arc};

use log::info;
use parking_lot::{RwLock, RwLockReadGuard};

#[derive(Default, Clone)]
pub struct Exit(Arc<RwLock<()>>);

impl Exit {
    pub fn new() -> Self {
        let arc = Arc::new(RwLock::new(()));

        let copy = arc.clone();

        ctrlc::set_handler(move || {
            if copy.is_locked() {
                info!("Waiting to exit safely...");
            }
            let _lock = copy.write();
            info!("Exiting...");
            exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        Self(arc)
    }

    pub fn lock(&self) -> RwLockReadGuard<'_, ()> {
        self.0.read()
    }
}
