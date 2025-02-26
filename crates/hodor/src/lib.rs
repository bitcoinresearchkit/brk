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
pub struct Hodor {
    holding: Arc<AtomicBool>,
    triggered: Arc<AtomicBool>,
}

impl Hodor {
    pub fn new() -> Self {
        let s = Self {
            triggered: Arc::new(AtomicBool::new(false)),
            holding: Arc::new(AtomicBool::new(false)),
        };

        let triggered = s.triggered.clone();

        let holding = s.holding.clone();
        let is_holding = move || holding.load(Ordering::SeqCst);

        ctrlc::set_handler(move || {
            info!("Exitting...");

            triggered.store(true, Ordering::SeqCst);

            if is_holding() {
                info!("Waiting to exit safely...");

                while is_holding() {
                    sleep(Duration::from_millis(50));
                }
            }

            exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        s
    }

    pub fn hold(&self) {
        self.holding.store(true, Ordering::SeqCst);
    }

    pub fn release(&self) {
        self.holding.store(false, Ordering::SeqCst);
    }

    pub fn triggered(&self) -> bool {
        self.triggered.load(Ordering::SeqCst)
    }
}
