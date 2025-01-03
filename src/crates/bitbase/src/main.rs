use std::path::Path;

use biter::bitcoincore_rpc::{Auth, Client};

mod database;
mod structs;

use database::Database;
use fjall::Slice;
use structs::{Addressbytes, Addressindex, Addresstype, Exit, Height, Txindex, Txoutindex};

// https://github.com/fjall-rs/fjall/discussions/72
// https://github.com/romanz/electrs/blob/master/doc/schema.md

const DAILY_BLOCK_TARGET: usize = 144;
const MONTHLY_BLOCK_TARGET: usize = DAILY_BLOCK_TARGET * 30;

fn main() -> color_eyre::Result<()> {
    let i = std::time::Instant::now();

    let data_dir = Path::new("../../../../bitcoin");
    let cookie = Path::new(data_dir).join(".cookie");
    let rpc = Client::new("http://localhost:8332", Auth::CookieFile(cookie)).unwrap();

    let exit = Exit::new();

    let mut db = Database::import()?;

    let mut height = db.start_height();

    let mut txindex = db
        .height_to_last_txindex
        .get(height)?
        .map(Txindex::incremented)
        .unwrap_or(Txindex::default());

    let mut addressindex = db
        .txoutindex_to_addressindex
        .prefix(Slice::from(txindex))
        .last()
        .map(|res| -> color_eyre::Result<Addressindex> {
            Ok(Addressindex::from(res?.1).incremented())
        })
        .unwrap_or(Ok(Addressindex::default()))?;

    let export = |db: &mut Database, height: Height| -> color_eyre::Result<()> {
        exit.block();
        println!("Exporting...");
        db.export(height)?;
        println!("Export done");
        exit.unblock();
        Ok(())
    };

    biter::new(data_dir, Some(height.into()), None, rpc)
        .iter()
        .try_for_each(|(_height, block, blockhash)| -> color_eyre::Result<()> {
            println!("Processing block {_height}...");

            height = Height::from(_height);

            if db.has_different_blockhash(height, &blockhash)? {
                db.rollback_from(height, &exit)?;
            }

            db.blockhash_prefix_to_height.insert(&blockhash, height)?;
            db.height_to_blockhash.insert(height, &blockhash);

            let txlen = block.txdata.len();
            let last_txindex = txlen - 1;

            block.txdata.into_iter().enumerate().try_for_each(
                |(txi, tx)| -> color_eyre::Result<()> {
                    if txi == 0 {
                        db.height_to_first_txindex.insert(height, txindex);
                    }
                    if txi == last_txindex {
                        db.height_to_last_txindex.insert(height, txindex);
                    }

                    if !db.txindex_to_txid.is_safe(height)
                        || !db.txid_prefix_to_txindex.is_safe(height)
                    {
                        let txid = tx.compute_txid();
                        db.txindex_to_txid.insert(txindex, &txid, height);
                        db.txid_prefix_to_txindex.insert(&txid, txindex, height)?;
                    }

                    db.txindex_to_height.insert(txindex, height);

                    txindex.increment();

                    tx.output.into_iter().enumerate().try_for_each(
                        |(vout, txout)| -> color_eyre::Result<()> {
                            let vout = vout as u16;
                            let txoutindex = Txoutindex::from((txindex, vout));
                            let amount = txout.value.into();
                            db.txoutindex_to_amount.insert(txoutindex, amount, height);

                            let script = &txout.script_pubkey;

                            let addressbytes =
                                Addressbytes::try_from(script).inspect_err(|_| {
                                    dbg!(&txout, height, txi);
                                })?;
                            let addresstype = Addresstype::try_from(script)?;

                            let mut addressindex_local = addressindex;

                            if let Some(addressindex_slice) = db
                                .addressbytes_prefix_to_addressindex
                                .get((&addressbytes).into())?
                            {
                                addressindex_local = addressindex_slice.into()
                            } else {
                                db.addressbytes_prefix_to_addressindex
                                    .insert(&addressbytes, addressindex_local, height)
                                    .inspect_err(|_| {
                                        dbg!(addresstype);
                                    })?;
                                db.addressindex_to_addressbytes.insert(
                                    addressindex_local,
                                    &addressbytes,
                                    height,
                                );
                                db.addressindex_to_addresstype.insert(
                                    addressindex_local,
                                    addresstype,
                                    height,
                                );
                                addressindex.increment();
                            }

                            db.txoutindex_to_addressindex.insert(
                                txoutindex,
                                addressindex_local,
                                height,
                            );

                            db.addressindex_to_txoutindexes.insert(
                                addressindex_local,
                                txoutindex,
                                height,
                            );

                            Ok(())
                        },
                    )?;

                    Ok(())
                },
            )?;

            let should_snapshot = _height % MONTHLY_BLOCK_TARGET == 0 && !exit.active();
            if should_snapshot {
                export(&mut db, height)?;
            }

            Ok(())
        })?;

    export(&mut db, height)?;

    dbg!(i.elapsed());

    Ok(())
}
