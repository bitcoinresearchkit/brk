use std::path::Path;

use bomputer::Computer;
use exit::Exit;
use iterator::rpc;
use storable_vec::SINGLE_THREAD;

mod structs;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let data_dir = Path::new("../../bitcoin");
    let rpc = rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(Path::new(data_dir).join(".cookie")),
    )?;
    let exit = Exit::new();

    let i = std::time::Instant::now();

    let mut computer: Computer<SINGLE_THREAD> = Computer::import(Path::new("../_outputs"))?;

    computer.compute(data_dir, rpc, &exit)?;

    dbg!(i.elapsed());

    Ok(())
}
