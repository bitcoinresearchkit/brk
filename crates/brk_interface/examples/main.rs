use std::{fs, path::Path};

use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_interface::{Interface, Params, ParamsOpt};
use brk_reader::Reader;
use brk_structs::Index;
use vecdb::Exit;

pub fn main() -> Result<()> {
    let bitcoin_dir = Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin");
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK1/bitcoin");

    let blocks_dir = bitcoin_dir.join("blocks");

    let outputs_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");
    fs::create_dir_all(&outputs_dir)?;
    // let outputs_dir = Path::new("/Volumes/WD_BLACK1/brk");

    let rpc = Box::leak(Box::new(bitcoincore_rpc::Client::new(
        "http://localhost:8332",
        bitcoincore_rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let parser = Reader::new(blocks_dir, rpc);

    let indexer = Indexer::forced_import(&outputs_dir)?;

    let computer = Computer::forced_import(&outputs_dir, &indexer, None)?;

    let interface = Interface::build(&parser, &indexer, &computer);

    dbg!(interface.search_and_format(Params {
        index: Index::Height,
        metrics: vec!["date"].into(),
        rest: ParamsOpt::default().set_from(-1),
    })?);
    dbg!(interface.search_and_format(Params {
        index: Index::Height,
        metrics: vec!["date", "timestamp"].into(),
        rest: ParamsOpt::default().set_from(-10).set_count(5),
    })?);

    Ok(())
}
