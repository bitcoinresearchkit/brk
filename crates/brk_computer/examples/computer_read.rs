use std::{env, path::Path, thread};

use brk_computer::Computer;
use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use mimalloc::MiMalloc;
use vecdb::{AnyStoredVec, Exit};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub fn main() -> Result<()> {
    // Can't increase main thread's stack size, thus we need to use another thread
    thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}

fn run() -> Result<()> {
    brk_logger::init(None)?;

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");
    // let outputs_dir = Path::new("../../_outputs");

    let indexer = Indexer::forced_import(&outputs_dir)?;

    let fetcher = Fetcher::import(true, None)?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let computer = Computer::forced_import(&outputs_dir, &indexer, Some(fetcher))?;

    let _a = dbg!(computer.chain.txinindex_to_value.region().meta());
    let _b = dbg!(indexer.vecs.txout.txoutindex_to_value.region().meta());

    Ok(())
}
