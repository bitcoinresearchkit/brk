use log::info;
use logger::init_log;

fn main() {
    init_log(None);
    info!("test");
}
