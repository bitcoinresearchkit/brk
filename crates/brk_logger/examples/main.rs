use log::{debug, error, info, trace};

fn main() {
    brk_logger::init(None);

    info!("info");
    debug!("debug");
    error!("error");
    trace!("trace");
}
