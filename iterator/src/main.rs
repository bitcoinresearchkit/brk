use std::path::Path;

use bitcoincore_rpc::{Auth, Client};

fn main() {
    let i = std::time::Instant::now();

    let data_dir = Path::new("../../bitcoin");
    let url = "http://localhost:8332";
    let cookie = Path::new(data_dir).join(".cookie");
    let auth = Auth::CookieFile(cookie);
    let rpc = Client::new(url, auth).unwrap();

    let start = None;
    let end = None;

    biter::new(data_dir, start, end, rpc)
        .iter()
        .for_each(|(height, _block, hash)| {
            println!("{height}: {hash}");
        });

    dbg!(i.elapsed());
}
