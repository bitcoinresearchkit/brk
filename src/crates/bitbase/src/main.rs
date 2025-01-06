use std::{path::Path, str::FromStr};

use biter::{
    bitcoin::{hashes::Hash, Txid},
    bitcoincore_rpc::{Auth, Client},
};

mod structs;

use color_eyre::eyre::{eyre, ContextCompat};
use fjall::{PersistMode, Slice, TransactionalKeyspace, WriteTransaction};
use structs::{
    Addressbytes, Addressindex, Addresstxoutindex, Addresstype, Amount, Exit, Height, Partitions,
    Prefix, SliceExtended, Txindex, Txoutindex,
};

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

    let keyspace = fjall::Config::new("./database").open_transactional()?;

    let mut parts = Partitions::import(&keyspace, &exit)?;

    let wtx = keyspace.write_tx();

    let mut height = parts.start_height();

    let mut txindex = wtx
        .get(parts.height_to_last_txindex.data(), Slice::from(height))?
        .map(Txindex::from)
        .map(Txindex::incremented)
        .unwrap_or(Txindex::default());

    let mut addressindex = wtx
        .get(
            parts.height_to_last_addressindex.data(),
            Slice::from(height),
        )?
        .map(Addressindex::from)
        .map(Addressindex::incremented)
        .unwrap_or(Addressindex::default());

    let export = |keyspace: &TransactionalKeyspace,
                  mut wtx: WriteTransaction,
                  parts: &Partitions,
                  height: Height|
     -> color_eyre::Result<()> {
        parts.udpate_meta(&mut wtx, height);
        exit.block();
        println!("Exporting...");
        wtx.commit()?;
        keyspace.persist(PersistMode::SyncAll)?;
        println!("Export done");
        exit.unblock();
        Ok(())
    };

    let mut wtx_opt = Some(wtx);

    biter::new(data_dir, Some(height.into()), None, rpc)
        .iter()
        .try_for_each(|(_height, block, blockhash)| -> color_eyre::Result<()> {
            let mut wtx = wtx_opt.take().context("option should've wtx")?;

            println!("Processing block {_height}...");

            height = Height::from(_height);

            let has_different_blockhash = wtx
                .get(parts.height_to_blockhash.data(), Slice::from(height))?
                .is_some_and(|saved_blockhash_slice| blockhash[..] != saved_blockhash_slice[..]);

            if has_different_blockhash {
                parts.rollback_from(&mut wtx, height, &exit)?;
            }

            if parts.blockhash_prefix_to_height.needs(height) {
                if let Some(prev) = wtx.fetch_update(
                    parts.blockhash_prefix_to_height.data(),
                    blockhash.prefix(),
                    |_| Some(Slice::from(height)),
                )? {
                    dbg!(prev);
                    return Err(eyre!("Expect none"));
                }
            }

            if parts.height_to_blockhash.needs(height) {
                wtx.insert(
                    parts.height_to_blockhash.data(),
                    Slice::from(height),
                    blockhash,
                );
            }

            if parts.height_to_first_addressindex.needs(height) {
                wtx.insert(
                    parts.height_to_first_addressindex.data(),
                    Slice::from(addressindex),
                    blockhash,
                );
            }

            let txlen = block.txdata.len();
            let last_txi = txlen - 1;

            block.txdata.into_iter().enumerate().try_for_each(
                |(txi, tx)| -> color_eyre::Result<()> {
                    if txi == 0 && parts.height_to_first_txindex.needs(height) {
                        wtx.insert(
                            parts.height_to_first_txindex.data(),
                            Slice::from(height),
                            Slice::from(txindex),
                        );
                    }
                    if txi == last_txi && parts.height_to_last_txindex.needs(height) {
                        wtx.insert(
                            parts.height_to_last_txindex.data(),
                            Slice::from(height),
                            Slice::from(txindex),
                        );
                    }

                    if parts.txindex_to_txid.needs(height)
                        || parts.txid_prefix_to_txindex.needs(height)
                    {
                        let txid = tx.compute_txid();

                        if parts.txindex_to_txid.needs(height) {
                            wtx.insert(parts.txindex_to_txid.data(), Slice::from(txindex), txid);
                        }

                        if parts.txid_prefix_to_txindex.needs(height) {
                            if let Some(prev) = wtx.fetch_update(
                                parts.txid_prefix_to_txindex.data(),
                                txid.prefix(),
                                |_| Some(Slice::from(txindex)),
                            )? {
                                let prev_txid = Txid::from_slice(&wtx
                                    .get(parts.txindex_to_txid.data(), &prev)?.expect("To have txid for txindex"))?;

                                let only_known_dup_txids = [Txid::from_str("d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599")?, Txid::from_str("e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468")?];

                                // TODO
                                // link txindex to txid
                                // txid_prefix should point to the first vout so no override
                                // do not add vout as they're invalid

                                if !only_known_dup_txids.contains(&prev_txid) {
                                    let prev_height = Height::from(wtx.get(parts.txindex_to_height.data(), &prev)?.expect("To have height"));
                                    let prev_txindex = Txindex::from(prev);
                                    dbg!(
                                        height,
                                        txid,
                                        txindex,
                                        prev_height,
                                        prev_txid,
                                        prev_txindex,
                                    );
                                    return Err(eyre!("Expect none"));
                                }
                            }
                        }
                    }

                    if parts.txindex_to_height.needs(height) {
                        wtx.insert(
                            parts.txindex_to_height.data(),
                            Slice::from(txindex),
                            Slice::from(height),
                        );
                    }

                    txindex.increment();

                    tx.output.iter().enumerate().try_for_each(
                        |(vout, txout)| -> color_eyre::Result<()> {
                            let vout = vout as u16;
                            let txoutindex = Txoutindex::from((txindex, vout));
                            let amount = Amount::from(txout.value);

                            if parts.txoutindex_to_amount.needs(height) {
                                wtx.insert(
                                    parts.txoutindex_to_amount.data(),
                                    Slice::from(txoutindex),
                                    Slice::from(amount),
                                );
                            }

                            let script = &txout.script_pubkey;

                            let addresstype = Addresstype::from(script);
                            let addressbytes =
                                Addressbytes::try_from((script, addresstype, addressindex))
                                    .inspect_err(|_| {
                                        dbg!(&txout, height, txi, &tx.compute_txid());
                                    })?;

                            let mut addressindex_local = addressindex;

                            if let Some(addressindex_slice) = wtx.get(
                                parts.addressbytes_prefix_to_addressindex.data(),
                                Slice::from(&addressbytes),
                            )? {
                                addressindex_local = addressindex_slice.into()
                            } else {
                                if parts.addressbytes_prefix_to_addressindex.needs(height) {
                                    if let Some(prev) = wtx.fetch_update(
                                        parts.addressbytes_prefix_to_addressindex.data(),
                                        Slice::from(&addressbytes),
                                        |_| Some(Slice::from(addressindex_local)),
                                    )? {
                                        dbg!(prev);
                                        return Err(eyre!("Expect none"));
                                    }
                                }

                                if parts.addressindex_to_addressbytes.needs(height) {
                                    wtx.insert(
                                        parts.addressindex_to_addressbytes.data(),
                                        Slice::from(addressindex_local),
                                        Slice::from(&addressbytes),
                                    );
                                }

                                if parts.addressindex_to_addresstype.needs(height) {
                                    wtx.insert(
                                        parts.addressindex_to_addresstype.data(),
                                        Slice::from(addressindex_local),
                                        Slice::from(addresstype),
                                    );
                                }

                                addressindex.increment();
                            }

                            if parts.txoutindex_to_addressindex.needs(height) {
                                wtx.insert(
                                    parts.txoutindex_to_addressindex.data(),
                                    Slice::from(txoutindex),
                                    Slice::from(addressindex_local),
                                );
                            }

                            if parts.addresstxoutindexes.needs(height) {
                                wtx.insert(
                                    parts.addresstxoutindexes.data(),
                                    Slice::from(Addresstxoutindex::from((
                                        addressindex_local,
                                        txoutindex,
                                    ))),
                                    Slice::default(),
                                );
                            }

                            Ok(())
                        },
                    )?;

                    Ok(())
                },
            )?;

            if parts.height_to_last_addressindex.needs(height) {
                wtx.insert(
                    parts.height_to_last_addressindex.data(),
                    Slice::from(addressindex.decremented()),
                    blockhash,
                );
            }

            let should_snapshot = _height % MONTHLY_BLOCK_TARGET == 0 && !exit.active();
            if should_snapshot {
                export(&keyspace, wtx, &parts, height)?;
                wtx_opt.replace(keyspace.write_tx());
            } else {
                wtx_opt.replace(wtx);
            }

            Ok(())
        })?;

    let wtx = wtx_opt
        .take()
        .context("option should have WriteTransaction")?;
    export(&keyspace, wtx, &parts, height)?;

    dbg!(i.elapsed());

    Ok(())
}
