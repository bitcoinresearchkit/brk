use std::path::Path;

use brk_core::{Dollars, Height, Result, Sats, Version};
use brk_store::Store;

fn main() -> Result<()> {
    let p = Path::new("./examples/_fjall");

    let keyspace = brk_store::open_keyspace(p)?;

    let mut store: Store<Dollars, Sats> =
        brk_store::Store::import(&keyspace, p, "n", Version::ZERO, None)?;

    store.insert_if_needed(Dollars::from(10.0), Sats::FIFTY_BTC, Height::ZERO);

    store.commit(Height::ZERO)?;

    Ok(())
}
