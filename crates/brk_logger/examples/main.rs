use std::io;

use log::{debug, error, info, trace};

fn main() -> io::Result<()> {
    brk_logger::init(None)?;

    info!("info");
    debug!("debug");
    error!("error");
    trace!("trace");

    Ok(())
}
