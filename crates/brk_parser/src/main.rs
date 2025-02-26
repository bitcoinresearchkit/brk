use std::path::Path;

use bitcoincore_rpc::{Auth, Client};
use brk_core::Height;
use brk_parser::Parser;

fn main() {
    let i = std::time::Instant::now();

    let data_dir = Path::new("../../../bitcoin");
    let rpc = Box::leak(Box::new(
        Client::new(
            "http://localhost:8332",
            Auth::CookieFile(Path::new(data_dir).join(".cookie")),
        )
        .unwrap(),
    ));

    let start = None;
    let end = None;

    let parser = Parser::new(data_dir, rpc);

    parser.parse(start, end).iter().for_each(|(height, _block, hash)| {
        println!("{height}: {hash}");
    });

    parser.get(Height::new(0));
    parser.get(Height::new(840_000));

    dbg!(i.elapsed());
}
