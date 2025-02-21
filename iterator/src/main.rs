use std::path::Path;

use bitcoincore_rpc::{Auth, Client};

fn main() {
    let i = std::time::Instant::now();

    let data_dir = Path::new("../../bitcoin");
    let rpc = Box::leak(Box::new(
        Client::new(
            "http://localhost:8332",
            Auth::CookieFile(Path::new(data_dir).join(".cookie")),
        )
        .unwrap(),
    ));

    let start = None;
    let end = None; //Some(200_000_u32.into());

    biterator::new(data_dir, start, end, rpc)
        .iter()
        .for_each(|(height, _block, hash)| {
            println!("{height}: {hash}");
        });

    dbg!(i.elapsed());
}
