use std::time::Instant;

use log::info;

use crate::structs::DisplayInstant;

pub fn time<F, T>(text: &str, function: F) -> T
where
    F: FnOnce() -> T,
{
    let time = Instant::now();

    let returned = function();

    info!("{text} {}", time.display());

    returned
}
