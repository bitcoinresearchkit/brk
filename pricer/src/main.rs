use bricer::{Binance, Kibo, Kraken};
use indexer::Height;
use serde_json::Value;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    logger::init_log(None);

    dbg!(Binance::fetch_1d()?);
    // dbg!(Binance::fetch_1mn_prices());
    // dbg!(Kraken::fetch_1d()?);
    // dbg!(Kraken::fetch_1mn_prices()?);
    // dbg!(Kibo::fetch_date_prices(2025)?);
    // dbg!(Kibo::fetch_height_prices(Height::from(880_000_u32))?);

    Ok(())
}
