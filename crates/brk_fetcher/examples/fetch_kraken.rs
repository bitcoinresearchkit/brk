use brk_error::Result;
use brk_fetcher::Kraken;

fn main() -> Result<()> {
    brk_logger::init(None)?;
    let _ = dbg!(Kraken::fetch_1d());
    let _ = dbg!(Kraken::fetch_1mn());
    Ok(())
}
