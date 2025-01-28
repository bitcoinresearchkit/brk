use structs::Date;

mod structs;

pub fn main() -> color_eyre::Result<()> {
    let date1 = Date::from(jiff::civil::Date::constant(2009, 1, 9));
    let date2 = Date::from(jiff::civil::Date::constant(2009, 1, 31));
    let date3 = Date::from(jiff::civil::Date::constant(2019, 1, 9));
    dbg!(usize::try_from(date1))?;
    dbg!(usize::try_from(date2))?;
    dbg!(usize::try_from(date3))?;
    Ok(())
}
