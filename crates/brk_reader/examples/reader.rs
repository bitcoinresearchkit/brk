use brk_error::Result;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};

fn main() -> Result<()> {
    let bitcoin_dir = Client::default_bitcoin_path();

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

    // Stream all blocks
    let i = std::time::Instant::now();
    for block in reader.read(None, None) {
        println!("{}: {}", block.height(), block.hash());
    }
    println!("Full read: {:?}", i.elapsed());

    Ok(())
}
