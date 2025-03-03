use std::{path::Path, thread::sleep, time::Duration};

use brk_computer::Computer;
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::{
    Parser,
    rpc::{self, RpcApi},
};
use log::info;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let bitcoin_dir = Path::new("../../../bitcoin");
    let rpc = Box::leak(Box::new(rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(Path::new(bitcoin_dir).join(".cookie")),
    )?));
    let exit = Exit::new();

    let parser = Parser::new(bitcoin_dir, rpc);

    let outputs_dir = Path::new("../../_outputs");

    let mut indexer = Indexer::new(&outputs_dir.join("indexed"))?;
    indexer.import_stores()?;
    indexer.import_vecs()?;

    let mut computer = Computer::new(&outputs_dir.join("computed"));
    computer.import_stores()?;
    computer.import_vecs()?;

    loop {
        let block_count = rpc.get_block_count()?;

        info!("{block_count} blocks found.");

        let starting_indexes = indexer.index(&parser, rpc, &exit)?;

        computer.compute(&mut indexer, starting_indexes, &exit)?;

        info!("Waiting for new blocks...");

        while block_count == rpc.get_block_count()? {
            sleep(Duration::from_secs(1))
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
