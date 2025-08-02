use brk_error::Result;
use brk_fetcher::{BRK, Binance, Fetcher, Kraken};
use brk_structs::{Date, Height};

fn main() -> Result<()> {
    brk_logger::init(None);

    let mut brk = BRK::default();
    dbg!(brk.get_from_height(Height::new(900_000))?);
    dbg!(brk.get_from_date(Date::new(2025, 6, 7))?);

    let mut fetcher = Fetcher::import(None)?;

    Binance::fetch_1d().map(|b| {
        dbg!(b.last_key_value());
    })?;
    Kraken::fetch_1d().map(|b| {
        dbg!(b.last_key_value());
    })?;
    Binance::fetch_1mn().map(|b| {
        dbg!(b.last_key_value());
    })?;
    Kraken::fetch_1mn().map(|b| {
        dbg!(b.last_key_value());
    })?;

    dbg!(fetcher.get_date(Date::new(2025, 6, 5))?);
    dbg!(fetcher.get_height(
        899911_u32.into(),
        1749133056_u32.into(),
        Some(1749132055_u32.into())
    )?);

    Ok(())
}
