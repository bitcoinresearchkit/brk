use std::{env, path::Path, thread};

use bitview::ImportContext;
use bitview_default::DefaultPlugins;
use bitview_query::AsyncQuery;
use bitview_server::{Server, ServerConfig, Website};
use brk_error::Result;
use brk_exit::Exit;
use brk_logger::init;
use brk_mempool::Mempool;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use tokio::{runtime::Builder, spawn};
use tracing::{error, info};
use vecdb::CacheBudget;

pub fn main() -> Result<()> {
    init(Some(Path::new(".log")))?;

    let bitcoin_dir = Client::default_bitcoin_path();
    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".bitview");

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);
    let context = ImportContext::new(&outputs_dir, &CACHE_BUDGET);
    let plugins = DefaultPlugins::import(context, &reader)?;

    let mut mempool = Mempool::new(&client);
    let read_only = mempool.read_only_clone();
    thread::spawn(move || {
        mempool.start();
    });

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let query = AsyncQuery::build(&plugins, Some(read_only));

    let runtime = Builder::new_multi_thread().enable_all().build()?;

    // Option 1: block_on to run and properly propagate errors
    runtime.block_on(async move {
        let server = Server::bind(
            &query,
            ServerConfig {
                data_path: outputs_dir,
                website: Website::Disabled,
                ..Default::default()
            },
        )
        .await?;

        let handle = spawn(server.serve());

        // Await the handle to catch both panics and errors
        match handle.await {
            Ok(Ok(())) => info!("Server shut down cleanly"),
            Ok(Err(e)) => error!("Server error: {e:?}"),
            Err(e) => {
                // JoinError - either panic or cancellation
                if e.is_panic() {
                    error!("Server panicked: {:?}", e.into_panic());
                } else {
                    error!("Server task cancelled");
                }
            }
        }

        Ok(()) as Result<()>
    })
}

static CACHE_BUDGET: CacheBudget = CacheBudget::new(2 * 1024 * 1024 * 1024);
