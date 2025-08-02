use std::path::Path;

use brk_error::Result;

fn main() -> Result<()> {
    let p = Path::new("./examples/_fjall");

    let _keyspace = brk_store::open_keyspace(p)?;

    // let mut store: Store<usize, usize> =
    //     brk_store::Store::import(&keyspace, p, "n", Version::ZERO, None)?;

    // store.insert_if_needed(Sats::new(10), Sats::FIFTY_BTC, Height::ZERO);

    // store.commit(Height::ZERO)?;

    // store.persist()?;

    Ok(())
}
