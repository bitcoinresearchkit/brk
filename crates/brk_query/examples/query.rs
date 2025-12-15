use std::{env, fs, path::Path, thread};

use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_mempool::Mempool;
use brk_query::Query;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_types::{Address, OutputType};
use vecdb::Exit;

pub fn main() -> Result<()> {
    // Can't increase main thread's stack size, thus we need to use another thread
    thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}

fn run() -> Result<()> {
    let bitcoin_dir = Client::default_bitcoin_path();
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK1/bitcoin");

    let blocks_dir = bitcoin_dir.join("blocks");

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");
    fs::create_dir_all(&outputs_dir)?;
    // let outputs_dir = Path::new("/Volumes/WD_BLACK1/brk");

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");
    // let outputs_dir = Path::new("../../_outputs");

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let reader = Reader::new(blocks_dir, &client);

    let indexer = Indexer::forced_import(&outputs_dir)?;

    let computer = Computer::forced_import(&outputs_dir, &indexer, None)?;

    let mempool = Mempool::new(&client);
    let mempool_clone = mempool.clone();
    thread::spawn(move || {
        mempool_clone.start();
    });

    let query = Query::build(&reader, &indexer, &computer, Some(mempool));

    dbg!(
        indexer
            .stores
            .addresstype_to_addresshash_to_addressindex
            .get_unwrap(OutputType::P2WSH)
            .approximate_len()
    );

    let _ = dbg!(query.address(Address {
        address: "bc1qwzrryqr3ja8w7hnja2spmkgfdcgvqwp5swz4af4ngsjecfz0w0pqud7k38".to_string(),
    }));

    let _ = dbg!(query.address_txids(
        Address {
            address: "bc1qwzrryqr3ja8w7hnja2spmkgfdcgvqwp5swz4af4ngsjecfz0w0pqud7k38".to_string(),
        },
        None,
        25
    ));

    let _ = dbg!(query.address_utxos(Address {
        address: "bc1qwzrryqr3ja8w7hnja2spmkgfdcgvqwp5swz4af4ngsjecfz0w0pqud7k38".to_string(),
    }));

    // dbg!(query.search_and_format(MetricSelection {
    //     index: Index::Height,
    //     metrics: vec!["date"].into(),
    //     range: DataRangeFormat::default().set_from(-1),
    // })?);
    // dbg!(query.search_and_format(MetricSelection {
    //     index: Index::Height,
    //     metrics: vec!["date", "timestamp"].into(),
    //     range: DataRangeFormat::default().set_from(-10).set_count(5),
    // })?);

    Ok(())
}
