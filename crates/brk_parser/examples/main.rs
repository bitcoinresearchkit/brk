use std::{collections::BTreeMap, path::Path};

use bitcoincore_rpc::{Auth, Client};
use brk_error::Result;
use brk_parser::Parser;

#[allow(clippy::needless_doctest_main)]
fn main() -> Result<()> {
    let i = std::time::Instant::now();

    let bitcoin_dir = Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin");

    let rpc = Box::leak(Box::new(Client::new(
        "http://localhost:8332",
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));

    let parser = Parser::new(bitcoin_dir.join("blocks"), rpc);

    let start = Some(915_138_u32.into());
    let end = None;
    let mut blk_index = 0;
    let mut diff = BTreeMap::new();
    parser.parse(start, end).iter().for_each(|block| {
        println!("{}: {}", block.height(), block.hash());
        let new_blk_index = block.position().blk_index();
        if new_blk_index < blk_index {
            diff.insert(blk_index - new_blk_index, block.height());
        }
        blk_index = new_blk_index;
    });

    // let v = diff.iter().rev().take(10).collect::<Vec<_>>();

    // let block_0 = parser.get(Height::new(0))?;
    // dbg!("{}", block_0.coinbase_tag());

    // let block_158251 = parser.get(Height::new(158251))?;
    // dbg!("{}", block_158251.coinbase_tag());

    // let block_173195 = parser.get(Height::new(173195))?;
    // dbg!("{}", block_173195.coinbase_tag());

    // let block_840_000 = parser.get(Height::new(840_004))?;
    // dbg!("{}", block_840_000.coinbase_tag());

    dbg!(i.elapsed());

    Ok(())
}
