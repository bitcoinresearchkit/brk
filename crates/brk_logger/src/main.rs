use log::info;

fn main() {
    brk_logger::init(None);
    info!("test");
}
