use std::time::Instant;

use brk_error::Result;
use brk_iterator::Blocks;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_types::Height;

fn main() -> Result<()> {
    let bitcoin_dir = Client::default_bitcoin_path();

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

    let blocks = Blocks::new(&client, &reader);

    let i = Instant::now();
    blocks
        .range(Height::new(920040), Height::new(920041))?
        // .start(Height::new(920040))?
        // .end(Height::new(10))?
        // .after(brk_types::BlockHash::try_from(
        //     "00000000000000000000840d205cac2728740e0e7c5dc92a04c52503017c6241",
        // )?)?
        .for_each(|b| {
            dbg!(b.height());
        });
    dbg!(i.elapsed());

    Ok(())
}
