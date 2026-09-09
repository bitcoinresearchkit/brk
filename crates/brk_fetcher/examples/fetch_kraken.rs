use brk_error::Result;
use brk_fetcher::Kraken;
use brk_logger::init;

fn main() -> Result<()> {
    init(None)?;
    let kraken = Kraken::new();
    let _ = dbg!(kraken.fetch_1d());
    let _ = dbg!(kraken.fetch_1mn());
    Ok(())
}
