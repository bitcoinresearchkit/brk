use std::{thread::sleep, time::Duration};

use logger::info;

pub fn retry<T>(
    function: impl Fn(usize) -> color_eyre::Result<T>,
    sleep_in_s: u64,
    retries: usize,
) -> color_eyre::Result<T> {
    let mut i = 0;

    loop {
        let res = function(i);

        if i == retries || res.is_ok() {
            return res;
        } else {
            info!("Failed, waiting {sleep_in_s} seconds...");
            sleep(Duration::from_secs(sleep_in_s));
        }

        i += 1;
    }
}
