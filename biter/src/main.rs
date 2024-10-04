use bitcoincore_rpc::{Auth, Client};

fn main() {
    let i = std::time::Instant::now();

    let url = "http://localhost:8332";
    let auth = Auth::UserPass("satoshi".to_string(), "nakamoto".to_string());
    let rpc = Client::new(url, auth).unwrap();

    let data_dir = "../bitcoin";
    let export_dir = "./target";
    let start = None;
    let end = None;

    biter::new(data_dir, export_dir, start, end, rpc)
        .iter()
        .for_each(|(height, _block, hash)| {
            println!("{height}: {hash}");
        });

    dbg!(i.elapsed());
}
