use std::{path::Path, time::Instant};

use brk_error::Result;
use brk_iterator::Blocks;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_structs::Height;

fn main() -> Result<()> {
    let bitcoin_dir = Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin");

    let client = Client::new(
        "http://localhost:8332",
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), client.clone());

    let blocks = Blocks::new(client, reader);

    let i = Instant::now();
    blocks
        .range(Height::new(920040), Height::new(920041))?
        // .start(Height::new(920040))?
        // .end(Height::new(10))?
        // .after(brk_structs::BlockHash::try_from(
        //     "00000000000000000000840d205cac2728740e0e7c5dc92a04c52503017c6241",
        // )?)?
        .for_each(|b| {
            dbg!(b.height());
        });
    dbg!(i.elapsed());

    Ok(())
}
