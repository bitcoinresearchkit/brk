use std::{path::Path, thread::sleep, time::Duration};

use bitcoincore_rpc::RpcApi;
use brk_computer::Computer;

use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_interface::Interface;
use brk_parser::Parser;
use brk_server::Server;
use brk_vecs::Exit;

pub fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")));

    let process = true;

    let bitcoin_dir = Path::new("");
    let brk_dir = Path::new("");

    let rpc = Box::leak(Box::new(bitcoincore_rpc::Client::new(
        "http://localhost:8332",
        bitcoincore_rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));
    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let parser = Parser::new(bitcoin_dir.join("blocks"), brk_dir.to_path_buf(), rpc);

    let outputs_dir = Path::new("../../_outputs");

    let mut indexer = Indexer::forced_import(outputs_dir)?;

    let fetcher = Some(Fetcher::import(None)?);

    let mut computer = Computer::forced_import(outputs_dir, &indexer, fetcher)?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let interface = Interface::build(&indexer, &computer);

            let server = Server::new(interface, None);

            let server = tokio::spawn(async move {
                server.serve(true).await.unwrap();
            });

            if process {
                loop {
                    let block_count = rpc.get_block_count()?;

                    let starting_indexes = indexer.index(&parser, rpc, &exit, true)?;

                    computer.compute(&indexer, starting_indexes, &exit)?;

                    while block_count == rpc.get_block_count()? {
                        sleep(Duration::from_secs(1))
                    }
                }
            }

            #[allow(unreachable_code)]
            server.await.unwrap();

            Ok(())
        })
}
