use std::{path::Path, thread::sleep, time::Duration};

use brk_computer::Computer;

use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_iterator::Blocks;
use brk_query::Query;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_server::Server;
use vecdb::Exit;

pub fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let process = true;

    let bitcoin_dir = Path::new("");

    let client = Client::new(
        "http://localhost:8332",
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

    let blocks = Blocks::new(&client, &reader);

    let outputs_dir = Path::new("../../_outputs");

    let mut indexer = Indexer::forced_import(outputs_dir)?;

    let fetcher = Some(Fetcher::import(true, None)?);

    let mut computer = Computer::forced_import(outputs_dir, &indexer, fetcher)?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let query = Query::build(&reader, &indexer, &computer);

            let server = Server::new(&query, None);

            let server = tokio::spawn(async move {
                server.serve(true).await.unwrap();
            });

            if process {
                loop {
                    let last_height = client.get_last_height()?;

                    let starting_indexes = indexer.checked_index(&blocks, &client, &exit)?;

                    computer.compute(&indexer, starting_indexes, &reader, &exit)?;

                    while last_height == client.get_last_height()? {
                        sleep(Duration::from_secs(1))
                    }
                }
            }

            #[allow(unreachable_code)]
            server.await.unwrap();

            Ok(())
        })
}
