use std::{path::Path, thread::sleep, time::Duration};

use brk_computer::Computer;
use brk_core::default_bitcoin_path;
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_parser::{
    Parser,
    rpc::{self, RpcApi},
};
use brk_server::{Server, Website};
use brk_vec::Computation;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let process = true;

    let bitcoin_dir = default_bitcoin_path();

    let rpc = Box::leak(Box::new(rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));
    let exit = Exit::new();

    let parser = Parser::new(bitcoin_dir.join("blocks"), rpc);

    let outputs_dir = Path::new("../../_outputs");

    let compressed = true;

    let mut indexer = Indexer::new(outputs_dir, compressed, true)?;
    indexer.import_stores()?;
    indexer.import_vecs()?;

    let fetcher = Some(Fetcher::import(None)?);

    let mut computer = Computer::new(outputs_dir, fetcher, compressed);
    computer.import_stores(&indexer)?;
    computer.import_vecs(&indexer, Computation::Lazy)?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let served_indexer = indexer.clone();
            let served_computer = computer.clone();

            let server = Server::new(served_indexer, served_computer, Website::KiboMoney)?;

            let server = tokio::spawn(async move {
                server.serve().await.unwrap();
            });

            if process {
                loop {
                    let block_count = rpc.get_block_count()?;

                    let starting_indexes = indexer.index(&parser, rpc, &exit)?;

                    computer.compute(&mut indexer, starting_indexes, &exit)?;

                    while block_count == rpc.get_block_count()? {
                        sleep(Duration::from_secs(1))
                    }
                }
            }

            #[allow(unreachable_code)]
            server.await.unwrap();

            Ok(())
        }) as color_eyre::Result<()>
}
