use brk_core::Date;
use brk_fetcher::Fetcher;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(None);

    let mut fetcher = Fetcher::import(None)?;

    dbg!(fetcher.get_date(Date::new(2025, 1, 1))?);
    dbg!(fetcher.get_height(
        881_000_u32.into(),
        1740683986_u32.into(),
        Some(1740683000_u32.into())
    )?);

    Ok(())
}
