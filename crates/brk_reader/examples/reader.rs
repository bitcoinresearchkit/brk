use brk_error::Result;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};

#[allow(clippy::needless_doctest_main)]
fn main() -> Result<()> {
    let i = std::time::Instant::now();

    let bitcoin_dir = Client::default_bitcoin_path();

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let blocks_dir = bitcoin_dir.join("blocks");

    let reader = Reader::new(blocks_dir, &client);

    let start = None;
    // let start = Some(916037_u32.into());
    let end = None;
    reader.read(start, end).iter().for_each(|block| {
        println!("{}: {}", block.height(), block.hash());
    });

    // let v = diff.iter().rev().take(10).collect::<Vec<_>>();

    // let block_0 = parser.get(Height::new(0))?;
    // dbg!("{}", block_0.coinbase_tag());

    // let block_158251 = parser.get(Height::new(158251))?;
    // dbg!("{}", block_158251.coinbase_tag());

    // let block_173195 = parser.get(Height::new(173195))?;
    // dbg!("{}", block_173195.coinbase_tag());

    // let block_840_000 = parser.get(Height::new(840_004))?;
    // dbg!("{}", block_840_000.coinbase_tag());

    dbg!(i.elapsed());

    Ok(())
}
