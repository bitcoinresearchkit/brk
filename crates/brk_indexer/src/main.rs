use std::{path::Path, thread::sleep, time::Duration};

use brk_exit::Exit;
use brk_indexer::{Indexer, rpc::RpcApi};
use brk_parser::{
    Parser,
    rpc::{self},
};
use brk_vec::CACHED_GETS;
use log::info;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let data_dir = Path::new("../../../bitcoin");
    let rpc = Box::leak(Box::new(rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(Path::new(data_dir).join(".cookie")),
    )?));
    let exit = Exit::new();

    let parser = Parser::new(data_dir, rpc);

    let mut indexer: Indexer<CACHED_GETS> = Indexer::import(Path::new("../../_outputs/indexes"))?;

    loop {
        let block_count = rpc.get_block_count()?;

        info!("{block_count} blocks found.");

        indexer.index(&parser, rpc, &exit)?;

        info!("Waiting for new blocks...");

        while block_count == rpc.get_block_count()? {
            sleep(Duration::from_secs(1))
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
