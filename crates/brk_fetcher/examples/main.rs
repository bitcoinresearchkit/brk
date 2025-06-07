use brk_core::{Date, Height};
use brk_fetcher::{BRK, Fetcher};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(None);

    let mut brk = BRK::default();
    dbg!(brk.get_from_height(Height::new(900_000))?);
    dbg!(brk.get_from_date(Date::new(2025, 6, 7))?);

    let mut fetcher = Fetcher::import(None)?;

    dbg!(fetcher.get_date(Date::new(2025, 6, 5))?);
    dbg!(fetcher.get_height(
        899911_u32.into(),
        1749133056_u32.into(),
        Some(1749132055_u32.into())
    )?);

    Ok(())
}
