use std::{fmt::Debug, thread::sleep, time::Duration};

use brk_error::Result;
use log::info;

pub fn default_retry<T>(function: impl Fn(usize) -> Result<T>) -> Result<T>
where
    T: Debug,
{
    retry(function, 5, 6)
}

fn retry<T>(function: impl Fn(usize) -> Result<T>, sleep_in_s: u64, retries: usize) -> Result<T>
where
    T: Debug,
{
    let mut i = 0;

    loop {
        let res = function(i);

        if i == retries || res.is_ok() {
            return res;
        } else {
            let _ = dbg!(res);
            info!("Failed, waiting {sleep_in_s} seconds...");
            sleep(Duration::from_secs(sleep_in_s));
        }

        i += 1;
    }
}
