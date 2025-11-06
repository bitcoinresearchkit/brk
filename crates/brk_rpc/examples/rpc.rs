use bitcoincore_rpc::RpcApi;
use brk_rpc::{Auth, Client};

fn main() {
    brk_logger::init(None).unwrap();

    let bitcoin_dir = Client::default_bitcoin_path();

    let auth = Auth::CookieFile(bitcoin_dir.join(".cookie"));

    let client = Client::new(Client::default_url(), auth).unwrap();

    loop {
        println!("{:?}", client.call(|c| c.get_block_count()).unwrap());
    }
}
