// fn main() {}

use pricer::{Binance, Kraken};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    logger::init_log(None);

    // dbg!(Binance::fetch_daily_prices());
    // dbg!(Binance::fetch_1mn_prices());
    // dbg!(Kraken::fetch_daily_prices());
    dbg!(Kraken::fetch_1mn_prices());

    Ok(())
}
