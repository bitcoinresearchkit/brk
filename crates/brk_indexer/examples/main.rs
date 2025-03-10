use std::{path::Path, thread::sleep, time::Duration};

use brk_core::default_bitcoin_path;
use brk_exit::Exit;
use brk_indexer::{Indexer, rpc::RpcApi};
use brk_parser::{
    Parser,
    rpc::{self},
};
use log::info;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let bitcoin_dir = default_bitcoin_path();

    let rpc = Box::leak(Box::new(rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));
    let exit = Exit::new();

    let parser = Parser::new(bitcoin_dir.join("blocks"), rpc);

    let mut indexer = Indexer::new(Path::new("../../_outputs/indexed").to_owned())?;
    indexer.import_stores()?;
    indexer.import_vecs()?;

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
