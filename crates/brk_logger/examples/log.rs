use std::io;

use brk_logger::init;
use tracing::{debug, error, info, trace};

fn main() -> io::Result<()> {
    init(None)?;

    info!("info");
    debug!("debug");
    error!("error");
    trace!("trace");

    Ok(())
}
