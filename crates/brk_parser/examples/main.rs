use bitcoincore_rpc::{Auth, Client};
use brk_core::{Height, default_bitcoin_path, default_brk_path};
use brk_parser::Parser;

fn main() {
    let i = std::time::Instant::now();

    let bitcoin_dir = default_bitcoin_path();
    let brk_dir = default_brk_path();

    let rpc = Box::leak(Box::new(
        Client::new(
            "http://localhost:8332",
            Auth::CookieFile(bitcoin_dir.join(".cookie")),
        )
        .unwrap(),
    ));

    let start = None;
    let end = None;

    let parser = Parser::new(bitcoin_dir.join("blocks"), brk_dir, rpc);

    parser
        .parse(start, end)
        .iter()
        .for_each(|(height, _block, hash)| {
            println!("{height}: {hash}");
        });

    println!(
        "{}",
        parser
            .get(Height::new(0))
            .txdata
            .first()
            .unwrap()
            .output
            .first()
            .unwrap()
            .script_pubkey
    );

    println!(
        "{}",
        parser
            .get(Height::new(840_000))
            .txdata
            .first()
            .unwrap()
            .output
            .first()
            .unwrap()
            .value
    );

    dbg!(i.elapsed());
}
