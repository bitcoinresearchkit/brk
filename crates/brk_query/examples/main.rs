use std::{env, fs, path::Path};

use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_query::{Params, ParamsOpt, Query};
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_types::Index;
use vecdb::Exit;

pub fn main() -> Result<()> {
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

    let query = Query::build(&reader, &indexer, &computer, None);

    dbg!(query.search_and_format(Params {
        index: Index::Height,
        metrics: vec!["date"].into(),
        rest: ParamsOpt::default().set_from(-1),
    })?);
    dbg!(query.search_and_format(Params {
        index: Index::Height,
        metrics: vec!["date", "timestamp"].into(),
        rest: ParamsOpt::default().set_from(-10).set_count(5),
    })?);

    Ok(())
}
