use std::{collections::BTreeMap, path::Path, thread};

use brk_computer::{Computer, pools};
use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use vecdb::Exit;

fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || -> Result<()> {
            let outputs_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");

            let indexer = Indexer::forced_import(&outputs_dir)?;

            let fetcher = Fetcher::import(true, None)?;

            let computer = Computer::forced_import(&outputs_dir, &indexer, Some(fetcher))?;

            let pools = pools();

            let mut res: BTreeMap<&'static str, usize> = BTreeMap::default();

            let mut height_to_first_txindex_iter = indexer.vecs.height_to_first_txindex.iter();
            // let mut i = indexer.vecs.txz

            indexer
                .stores
                .height_to_coinbase_tag
                .iter()
                .for_each(|(_, coinbase_tag)| {
                    let pool = pools.find_from_coinbase_tag(&coinbase_tag);
                    if let Some(pool) = pool {
                        *res.entry(pool.name).or_default() += 1;
                    } else {
                        *res.entry(pools.get_unknown().name).or_default() += 1;
                    }
                });

            let mut v = res.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
            v.sort_unstable();
            println!("{:#?}", v);
            println!("{:#?}", v.len());

            Ok(())
        })?
        .join()
        .unwrap()
}
