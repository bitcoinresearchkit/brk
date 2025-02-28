#![doc = include_str!("../README.md")]

use std::{
    process::exit,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::Duration,
};

use log::info;

#[derive(Default, Clone)]
pub struct Exit {
    blocking: Arc<AtomicBool>,
    triggered: Arc<AtomicBool>,
}

impl Exit {
    pub fn new() -> Self {
        let s = Self {
            triggered: Arc::new(AtomicBool::new(false)),
            blocking: Arc::new(AtomicBool::new(false)),
        };

        let triggered = s.triggered.clone();

        let blocking = s.blocking.clone();
        let is_blocking = move || blocking.load(Ordering::SeqCst);

        ctrlc::set_handler(move || {
            info!("Exitting...");

            triggered.store(true, Ordering::SeqCst);

            if is_blocking() {
                info!("Waiting to exit safely...");

                while is_blocking() {
                    sleep(Duration::from_millis(50));
                }
            }

            exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        s
    }

    pub fn block(&self) {
        self.blocking.store(true, Ordering::SeqCst);
    }

    pub fn release(&self) {
        self.blocking.store(false, Ordering::SeqCst);
    }

    pub fn triggered(&self) -> bool {
        self.triggered.load(Ordering::SeqCst)
    }
}
