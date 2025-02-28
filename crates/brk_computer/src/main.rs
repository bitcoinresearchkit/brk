use std::{path::Path, thread::sleep, time::Duration};

use brk_computer::Computer;
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::{
    Parser,
    rpc::{self, RpcApi},
};
use brk_vec::CACHED_GETS;
use log::info;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let data_dir = Path::new("../../../bitcoin");
    let rpc = Box::leak(Box::new(rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(Path::new(data_dir).join(".cookie")),
    )?));
    let exit = Exit::new();

    let parser = Parser::new(data_dir, rpc);

    let outputs_dir = Path::new("../../_outputs");

    let indexer: Indexer<CACHED_GETS> = Indexer::import(&outputs_dir.join("indexes"))?;

    // let mut computer = Computer::import(&outputs_dir.join("computed"))?;

    // loop {
    //     let block_count = rpc.get_block_count()?;

    //     info!("{block_count} blocks found.");

    //     indexer.index(&parser, rpc, &exit)?;

    //     computer.compute(indexer, &exit)?;

    //     info!("Waiting for new blocks...");

    //     while block_count == rpc.get_block_count()? {
    //         sleep(Duration::from_secs(1))
    //     }
    // }

    Ok(())
}
