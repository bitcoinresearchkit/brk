use brk_error::Result;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_types::Height;

fn main() -> Result<()> {
    let bitcoin_dir = Client::default_bitcoin_path();

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

    let heights = [0, 100_000, 158_251, 173_195, 840_000];

    for &h in &heights {
        let height = Height::new(h);
        let i = std::time::Instant::now();

        if let Some(block) = reader.read(Some(height), Some(height)).iter().next() {
            println!(
                "height={} hash={} txs={} coinbase=\"{}\" ({:?})",
                block.height(),
                block.hash(),
                block.txdata.len(),
                block.coinbase_tag(),
                i.elapsed(),
            );
        }
    }

    Ok(())
}
