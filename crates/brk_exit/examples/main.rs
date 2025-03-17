use std::{path::Path, thread::sleep, time::Duration};

use brk_exit::Exit;
use log::info;

fn main() {
    let exit = Exit::new();

    brk_logger::init(Some(Path::new(".log")));

    exit.block();

    let mut i = 0;
    while i < 21 {
        info!("i = {i}");
        sleep(Duration::from_secs(1));
        i += 1;
    }

    exit.release();

    let mut j = 0;
    while j < 10 {
        info!("j = {j}");
        sleep(Duration::from_secs(1));
        j += 1;
    }
}
