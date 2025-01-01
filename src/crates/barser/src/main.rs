use std::path::Path;

use biter::bitcoincore_rpc::{Auth, Client};

mod databases;
mod structs;

use databases::Databases;
use structs::{Exit, Height, Txindex, Txoutindex};

// https://github.com/fjall-rs/fjall/discussions/72
// https://github.com/romanz/electrs/blob/master/doc/schema.md

const DAILY_BLOCK_TARGET: usize = 144;
const MONTHLY_BLOCK_TARGET: usize = DAILY_BLOCK_TARGET * 30;

fn main() -> color_eyre::Result<()> {
    let i = std::time::Instant::now();

    let data_dir = Path::new("../bitcoin");
    let cookie = Path::new(data_dir).join(".cookie");
    let rpc = Client::new("http://localhost:8332", Auth::CookieFile(cookie)).unwrap();

    let exit = Exit::new();

    let mut dbs = Databases::import()?;

    let mut height = dbs.start_height(&rpc)?;

    let mut txindex = dbs
        .height_to_last_txindex
        .get(height)?
        .unwrap_or(Txindex::default());

    let export = |dbs: &mut Databases, height: Height| -> color_eyre::Result<()> {
        exit.block();
        println!("Exporting...");
        dbs.export(height)?;
        println!("Export done");
        exit.unblock();
        Ok(())
    };

    biter::new(data_dir, Some(height.into()), None, rpc)
        .iter()
        .try_for_each(|(_height, block, blockhash)| -> color_eyre::Result<()> {
            println!("Processing block {_height}...");

            height = Height::from(_height);

            if dbs.has_different_blockhash(height, &blockhash)? {
                dbs.erase_from(height)?;
            }

            dbs.blockhash_prefix_to_height.insert(&blockhash, height)?;
            dbs.height_to_blockhash.insert(height, &blockhash);

            let txlen = block.txdata.len();

            block.txdata.into_iter().enumerate().try_for_each(
                |(i, tx)| -> color_eyre::Result<()> {
                    if i == txlen - 1 {
                        dbs.height_to_last_txindex.insert(height, txindex);
                    }

                    if !dbs.txindex_to_txid.is_safe(height)
                        || !dbs.txid_prefix_to_txindex.is_safe(height)
                    {
                        let txid = tx.compute_txid();
                        dbs.txindex_to_txid.insert(txindex, &txid, height);
                        dbs.txid_prefix_to_txindex.insert(&txid, txindex, height)?;
                    }

                    txindex.increment();

                    tx.output.into_iter().enumerate().for_each(|(vout, txout)| {
                        let vout = vout as u16;
                        let txoutindex = Txoutindex::from((txindex, vout));
                        let amount = txout.value.into();
                        dbs.txoutindex_to_amount.insert(txoutindex, amount, height);
                    });

                    Ok(())
                },
            )?;

            let should_snapshot = _height % MONTHLY_BLOCK_TARGET == 0 && !exit.active();
            if should_snapshot {
                export(&mut dbs, height)?;
            }

            Ok(())
        })?;

    export(&mut dbs, height)?;

    dbg!(i.elapsed());

    Ok(())
}
