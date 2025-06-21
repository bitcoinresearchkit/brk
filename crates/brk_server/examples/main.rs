use std::{path::Path, thread::sleep, time::Duration};

use bitcoincore_rpc::RpcApi;
use brk_computer::Computer;
use brk_core::default_bitcoin_path;
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_parser::Parser;
use brk_server::{Server, Website};
use brk_vec::{Computation, Format};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let process = true;

    let bitcoin_dir = default_bitcoin_path();

    let rpc = Box::leak(Box::new(bitcoincore_rpc::Client::new(
        "http://localhost:8332",
        bitcoincore_rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));
    let exit = Exit::new();

    let parser = Parser::new(bitcoin_dir.join("blocks"), rpc);

    let outputs_dir = Path::new("../../_outputs");

    let format = Format::Compressed;

    let mut indexer = Indexer::forced_import(outputs_dir)?;

    let fetcher = Some(Fetcher::import(None)?);

    let mut computer =
        Computer::forced_import(outputs_dir, &indexer, Computation::Lazy, fetcher, format)?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let served_indexer = indexer.clone();
            let served_computer = computer.clone();

            let server = Server::new(served_indexer, served_computer, Website::Default)?;

            let server = tokio::spawn(async move {
                server.serve(true, true).await.unwrap();
            });

            if process {
                loop {
                    let block_count = rpc.get_block_count()?;

                    let starting_indexes = indexer.index(&parser, rpc, &exit, true)?;

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
