use bitcoincore_rpc::{Auth, Client};
use brk_core::{Height, OutputType, default_bitcoin_path};
use brk_parser::Parser;

fn main() {
    let i = std::time::Instant::now();

    let bitcoin_dir = default_bitcoin_path();

    let rpc = Box::leak(Box::new(
        Client::new(
            "http://localhost:8332",
            Auth::CookieFile(bitcoin_dir.join(".cookie")),
        )
        .unwrap(),
    ));

    // let start = None;
    // let end = None;

    let parser = Parser::new(bitcoin_dir.join("blocks"), rpc);

    // parser
    //     .parse(start, end)
    //     .iter()
    //     .for_each(|(height, _block, hash)| {
    //         println!("{height}: {hash}");
    //     });

    // println!(
    //     "{}",
    //     parser
    //         .get(Height::new(0))
    //         .txdata
    //         .first()
    //         .unwrap()
    //         .output
    //         .first()
    //         .unwrap()
    //         .script_pubkey
    // );

    let block_850_000 = parser.get(Height::new(850_000));

    let tx = block_850_000.txdata.iter().find(|tx| {
        tx.compute_txid().to_string()
            == "b10c0000004da5a9d1d9b4ae32e09f0b3e62d21a5cce5428d4ad714fb444eb5d"
    });

    let output = tx.unwrap().tx_out(7).unwrap();

    dbg!(OutputType::from(&output.script_pubkey));

    dbg!(output);

    // println!(
    //     "{}",

    //         .txdata
    //         .first()
    //         .unwrap()
    //         .output
    //         .first()
    //         .unwrap()
    //         .value
    // );

    dbg!(i.elapsed());
}
