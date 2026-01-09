use std::io;

use tracing::{debug, error, info, trace};

fn main() -> io::Result<()> {
    brk_logger::init(None)?;

    info!("info");
    debug!("debug");
    error!("error");
    trace!("trace");

    Ok(())
}
