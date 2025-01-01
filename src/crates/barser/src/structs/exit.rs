use std::{
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

#[derive(Default, Clone)]
pub struct Exit {
    blocked: Arc<AtomicBool>,
    active: Arc<AtomicBool>,
}

impl Exit {
    pub fn new() -> Self {
        let s = Self {
            active: Arc::new(AtomicBool::new(false)),
            blocked: Arc::new(AtomicBool::new(false)),
        };

        let active = s.active.clone();

        let _blocked = s.blocked.clone();
        let blocked = move || _blocked.load(Ordering::SeqCst);

        ctrlc::set_handler(move || {
            println!("Exitting...");

            active.store(true, Ordering::SeqCst);

            if blocked() {
                println!("Waiting to exit safely");

                while blocked() {
                    sleep(Duration::from_millis(50));
                }
            }

            exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        s
    }

    pub fn block(&self) {
        self.blocked.store(true, Ordering::SeqCst);
    }

    pub fn unblock(&self) {
        self.blocked.store(false, Ordering::SeqCst);
    }

    pub fn active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }
}
