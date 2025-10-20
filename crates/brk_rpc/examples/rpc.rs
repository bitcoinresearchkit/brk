use std::path::Path;

use bitcoincore_rpc::RpcApi;
use brk_rpc::{Auth, Client};

fn main() {
    brk_logger::init(None).unwrap();

    let bitcoin_dir = Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin");

    let auth = Auth::CookieFile(bitcoin_dir.join(".cookie"));

    let client = Client::new("http://localhost:8332", auth).unwrap();

    loop {
        println!("{:?}", client.call(|c| c.get_block_count()).unwrap());
    }
}
