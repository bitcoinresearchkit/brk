use std::time::Instant;

use crate::utils::log;

pub fn time<F, T>(name: &str, function: F) -> T
where
    F: FnOnce() -> T,
{
    let time = Instant::now();

    let returned = function();

    log(&format!("{name}: {} seconds", time.elapsed().as_secs_f32()));

    returned
}
