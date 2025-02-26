use std::{path::Path, thread::sleep, time::Duration};

use brk_indexer::{Indexer, rpc::RpcApi};
use brk_parser::{
    Parser,
    rpc::{self},
};
use hodor::Hodor;
use log::info;
use storable_vec::CACHED_GETS;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let data_dir = Path::new("../../../bitcoin");
    let rpc = Box::leak(Box::new(rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(Path::new(data_dir).join(".cookie")),
    )?));
    let hodor = Hodor::new();

    let parser = Parser::new(data_dir, rpc);

    loop {
        let block_count = rpc.get_blockchain_info().unwrap().blocks as usize;

        info!("{block_count} blocks found.");

        let i = std::time::Instant::now();

        let mut indexer: Indexer<CACHED_GETS> = Indexer::import(Path::new("../../_outputs/indexes"))?;

        indexer.index(&parser, rpc, &hodor)?;

        info!("Took: {:?}", i.elapsed());

        info!("Waiting for a new block...");

        while block_count == rpc.get_blockchain_info().unwrap().blocks as usize {
            sleep(Duration::from_secs(1))
        }
    }
}
