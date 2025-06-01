use std::path::Path;

use brk_core::{Dollars, Height, Result, Sats, Version};
use brk_store::Store;

fn main() -> Result<()> {
    let p = Path::new("./examples/_fjall");

    let keyspace = brk_store::open_keyspace(p)?;

    let mut store: Store<Dollars, Sats> =
        brk_store::Store::import(&keyspace, p, "n", Version::ZERO, None)?;

    store.copy_db_to_puts();

    *store.puts_entry_or_default(&Dollars::from(10.0)) += Sats::ONE_BTC;
    *store.puts_entry_or_default(&Dollars::from(1.0)) += Sats::ONE_BTC;
    *store.puts_entry_or_default(&Dollars::ZERO) += Sats::ONE_BTC;
    *store.puts_entry_or_default(&Dollars::ZERO) += Sats::ONE_BTC;

    dbg!(store.tx_iter().collect::<Vec<_>>());

    store.commit(Height::ZERO)?;

    store.copy_db_to_puts();

    dbg!(store.tx_iter().collect::<Vec<_>>());

    *store.puts_entry_or_default(&Dollars::from(10.0)) += Sats::ONE_BTC;
    *store.puts_entry_or_default(&Dollars::from(1.0)) += Sats::ONE_BTC;
    *store.puts_entry_or_default(&Dollars::ZERO) += Sats::ONE_BTC;
    *store.puts_entry_or_default(&Dollars::ZERO) += Sats::ONE_BTC;

    dbg!(store.tx_iter().collect::<Vec<_>>());

    store.commit(Height::from(1_u32))?;

    store.copy_db_to_puts();

    dbg!(store.tx_iter().collect::<Vec<_>>());

    Ok(())
}
