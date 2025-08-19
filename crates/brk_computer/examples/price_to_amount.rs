use std::path::Path;

use brk_computer::PriceToAmount;
use brk_error::Result;
use brk_structs::Height;

pub fn main() -> Result<()> {
    let path = Path::new(&std::env::var("HOME").unwrap())
        .join(".brk")
        .join("computed/stateful/states");
    let mut price_to_amount = PriceToAmount::create(&path, "addrs_above_1btc_under_10btc");
    dbg!(price_to_amount.import_at_or_before(Height::new(890000))?);
    dbg!(price_to_amount);
    Ok(())
}
