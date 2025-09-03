use std::path::Path;

use bitcoincore_rpc::{Auth, Client, Result};
use brk_parser::Parser;
use brk_structs::Height;

#[allow(clippy::needless_doctest_main)]
fn main() -> Result<()> {
    let i = std::time::Instant::now();

    let bitcoin_dir = Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin");
    let brk_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");

    let rpc = Box::leak(Box::new(Client::new(
        "http://localhost:8332",
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));

    let parser = Parser::new(bitcoin_dir.join("blocks"), Some(brk_dir), rpc);

    let start = None;
    let end = None;
    parser
        .parse(start, end)
        .iter()
        .for_each(|(height, _block, hash)| {
            println!("{height}: {}", hash);
        });

    let block_0 = parser.get(Height::new(0));

    println!(
        "{}",
        block_0
            .txdata
            .first()
            .unwrap()
            .output
            .first()
            .unwrap()
            .script_pubkey
    );

    let block_158251 = parser.get(Height::new(158251));
    println!(
        "{}",
        block_158251
            .txdata
            .first()
            .unwrap()
            .output
            .first()
            .unwrap()
            .script_pubkey
    );

    let block_840_000 = parser.get(Height::new(840_004));

    println!(
        "{}",
        block_840_000
            .txdata
            .first()
            .unwrap()
            .output
            .first()
            .unwrap()
            .value
    );

    dbg!(i.elapsed());

    Ok(())
}
