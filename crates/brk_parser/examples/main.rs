use std::path::Path;

use bitcoincore_rpc::{Auth, Client, Result};
use brk_parser::Parser;
use brk_structs::Height;

#[allow(clippy::needless_doctest_main)]
fn main() -> Result<()> {
    let i = std::time::Instant::now();

    let bitcoin_dir = Path::new("").join("");
    let brk_dir = Path::new("").join("");

    let rpc = Box::leak(Box::new(Client::new(
        "http://localhost:8332",
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));

    let start = None;
    let end = None;

    let parser = Parser::new(bitcoin_dir.join("blocks"), brk_dir, rpc);

    parser
        .parse(start, end)
        .iter()
        .for_each(|(height, _block, hash)| {
            println!("{height}: {hash}");
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

    let block_840_000 = parser.get(Height::new(840_000));

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
