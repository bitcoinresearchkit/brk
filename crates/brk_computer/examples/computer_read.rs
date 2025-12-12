use std::{env, path::Path, thread};

use brk_computer::Computer;
use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_types::TxIndex;
use vecdb::{AnyStoredVec, Exit, GenericStoredVec};

pub fn main() -> Result<()> {
    // Can't increase main thread's stack size, thus we need to use another thread
    thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}

fn run() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");
    // let outputs_dir = Path::new("../../_outputs");

    let indexer = Indexer::forced_import(&outputs_dir)?;

    let fetcher = Fetcher::import(true, None)?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let computer = Computer::forced_import(&outputs_dir, &indexer, Some(fetcher))?;

    let txindex = TxIndex::new(134217893);

    dbg!(
        indexer
            .vecs
            .tx.txindex_to_txid
            .read_once(txindex)
            .unwrap()
            .to_string()
    );
    let first_txinindex = indexer.vecs.tx.txindex_to_first_txinindex.read_once(txindex)?;
    dbg!(first_txinindex);
    let first_txoutindex = indexer
        .vecs
        .tx.txindex_to_first_txoutindex
        .read_once(txindex)?;
    dbg!(first_txoutindex);
    let input_count = *computer.indexes.txindex_to_input_count.read_once(txindex)?;
    dbg!(input_count);
    let output_count = *computer
        .indexes
        .txindex_to_output_count
        .read_once(txindex)?;
    dbg!(output_count);
    dbg!(
        computer
            .indexes
            .txinindex_to_txoutindex
            .read_once(first_txinindex)
    );
    dbg!(
        computer
            .indexes
            .txinindex_to_txoutindex
            .read_once(first_txinindex + 1)
    );
    dbg!(computer.chain.txinindex_to_value.read_once(first_txinindex));
    dbg!(
        computer
            .chain
            .txinindex_to_value
            .read_once(first_txinindex + 1)
    );
    dbg!(indexer.vecs.txout.txoutindex_to_value.read_once(first_txoutindex));
    dbg!(
        indexer
            .vecs
            .txout.txoutindex_to_value
            .read_once(first_txoutindex + 1)
    );
    dbg!(computer.chain.txindex_to_input_value.read_once(txindex));
    dbg!(computer.chain.txindex_to_input_value.read_once(txindex));
    dbg!(computer.chain.txindex_to_output_value.read_once(txindex));
    // dbg!(computer.indexes.txindex_to_txindex.ge(txindex));
    dbg!(
        computer
            .indexes
            .txinindex_to_txoutindex
            .region()
            .meta()
            .len()
    );
    Ok(())
}
