use brk_error::Result;
use brk_fetcher::Kraken;

fn main() -> Result<()> {
    brk_logger::init(None)?;
    let kraken = Kraken::new();
    let _ = dbg!(kraken.fetch_1d());
    let _ = dbg!(kraken.fetch_1mn());
    Ok(())
}
