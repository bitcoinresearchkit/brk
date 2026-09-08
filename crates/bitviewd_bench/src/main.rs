use benchmark::Benchmark;
use bitview::{Config as RunnerConfig, ImportContext, UpdateContext, bootstrap};
use bitview_default::DefaultPlugins;
use bitviewd::Config;
use brk_error::Result;
use brk_exit::Exit;
use brk_reader::Reader;
use tracing::info;

mod benchmark;

fn main() -> Result<()> {
    let RunnerConfig {
        client,
        blocks_path,
        server,
    } = Config::import()?;
    brk_logger::init(Some(&server.logs_path()))?;
    let data_path = server.data_path;
    client.wait_for_synced_node()?;

    let chain_height = client.get_last_height()?;
    let benchmark = Benchmark::new(&data_path, &blocks_path, chain_height)?;
    let reader = Reader::new(blocks_path, &client);
    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let cleanup = benchmark.clone();
    exit.register_cleanup(move || {
        let _ = cleanup.abort();
    });

    benchmark.measure(|| {
        bootstrap(
            ImportContext::new(&data_path, &CACHE_BUDGET),
            |context| DefaultPlugins::import(context, &reader),
            UpdateContext::new(&exit),
        )
    })?;
    info!("Benchmark saved to {}", benchmark.path().display());
    Ok(())
}

static CACHE_BUDGET: vecdb::CacheBudget = vecdb::CacheBudget::new(2 * 1024 * 1024 * 1024);
