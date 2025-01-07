use std::{collections::BTreeMap, path::Path, str::FromStr};

use biter::{
    bitcoin::{hashes::Hash, Txid},
    bitcoincore_rpc::{Auth, Client},
};

mod structs;

use color_eyre::eyre::{eyre, ContextCompat};
use fjall::{PersistMode, Slice, TransactionalKeyspace, WriteTransaction};
use rayon::prelude::*;
use structs::{
    Addressbytes, Addressindex, Addresstxoutindex, Addresstype, Amount, Exit, Height, Partitions, Prefix,
    SliceExtended, Txindex, Txoutindex,
};

// https://github.com/fjall-rs/fjall/discussions/72
// https://github.com/romanz/electrs/blob/master/doc/schema.md

const DAILY_BLOCK_TARGET: usize = 144;
const MONTHLY_BLOCK_TARGET: usize = DAILY_BLOCK_TARGET * 30;
const U16MAX: usize = u16::MAX as usize;

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
        .get(parts.height_to_first_txindex.data(), Slice::from(height))?
        .map(Txindex::from)
        .unwrap_or(Txindex::default());

    let mut addressindex = wtx
        .get(parts.height_to_first_addressindex.data(), Slice::from(height))?
        .map(Addressindex::from)
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
            println!("Processing block {_height}...");

            height = Height::from(_height);

            let mut wtx = wtx_opt.take().context("option should have wtx")?;

            let saved_blockhash_slice_opt = wtx.get(parts.height_to_blockhash.data(), Slice::from(height))?;
            if let Some(saved_blockhash_slice) = saved_blockhash_slice_opt {
                if blockhash[..] != saved_blockhash_slice[..] {
                    parts.rollback_from(&mut wtx, height, &exit)?;
                } else {
                    wtx_opt.replace(wtx);
                    return Ok(());
                }
            }

            if parts.blockhash_prefix_to_height.needs(height) {
                if let Some(prev_height_slice) =
                    wtx.fetch_update(parts.blockhash_prefix_to_height.data(), blockhash.prefix(), |_| {
                        Some(Slice::from(height))
                    })?
                {
                    dbg!(blockhash, Height::from(prev_height_slice));
                    return Err(eyre!("Collision, expect prefix to need be set yet"));
                }
            }

            if parts.height_to_blockhash.needs(height) {
                wtx.insert(parts.height_to_blockhash.data(), Slice::from(height), blockhash);
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

            let mut txi_to_txid_and_prev_txindex_slice_opt = block
                .txdata
                .par_iter()
                .enumerate()
                .map(|(txi, tx)| -> color_eyre::Result<(usize, (Txid, Option<Slice>))> {
                    let txid = tx.compute_txid();

                    let prev_txindex_slice_opt = wtx.get(parts.txid_prefix_to_txindex.data(), txid.prefix())?;

                    Ok((txi, (txid, prev_txindex_slice_opt)))
                })
                .try_fold(
                    || -> BTreeMap<usize, (Txid, Option<Slice>)> { BTreeMap::default() },
                    |mut map, tuple| -> color_eyre::Result<BTreeMap<usize, (Txid, Option<Slice>)>> {
                        let (txi, tuple) = tuple?;
                        map.insert(txi, tuple);
                        Ok(map)
                    },
                )
                .try_reduce(BTreeMap::default, |mut map, mut map2| {
                    if map.len() > map2.len() {
                        map.append(&mut map2);
                        Ok(map)
                    } else {
                        map2.append(&mut map);
                        Ok(map2)
                    }
                })?;

            // let addresstxoutindexes_out = block
            //     .txdata
            //     .par_iter()
            //     .filter(|tx| !tx.is_coinbase())
            //     .flat_map(|tx| &tx.input)
            //     .try_fold(
            //         || -> Vec<Addresstxoutindex> { vec![] },
            //         |mut vec, txin| -> color_eyre::Result<Vec<Addresstxoutindex>> {
            //             let outpoint = txin.previous_output;
            //             let txid_prefix = outpoint.txid.prefix();
            //             let vout = outpoint.vout as u16;

            //             let txindex = Txindex::from(
            //                 wtx.get(parts.txid_prefix_to_txindex.data(), txid_prefix)?
            //                     .context("Expect txid to be saved")?,
            //             );

            //             let txoutindex = Txoutindex::from((txindex, vout));

            //             let addressindex = Addressindex::from(
            //                 wtx.get(parts.txoutindex_to_addressindex.data(), Slice::from(txoutindex))?
            //                     .context("Expect addressindex to not be none")
            //                     .inspect_err(|_| {
            //                         let height = Height::from(
            //                             wtx.get(parts.txindex_to_height.data(), Slice::from(txindex))
            //                                 .expect("txindex_to_height get not fail")
            //                                 .expect("Expect height for txindex"),
            //                         );
            //                         dbg!(outpoint.txid, txindex, vout, txoutindex, height);
            //                     })?,
            //             );

            //             if parts.addresstxoutindexes_out.needs(height) {
            //                 vec.push(Addresstxoutindex::from((addressindex, txoutindex)));
            //             }

            //             Ok(vec)
            //         },
            //     )
            //     .try_reduce(Vec::new, |mut v, mut v2| {
            //         if v.len() > v2.len() {
            //             v.append(&mut v2);
            //             Ok(v)
            //         } else {
            //             v2.append(&mut v);
            //             Ok(v2)
            //         }
            //     })?;

            // addresstxoutindexes_out.into_iter().for_each(|addresstxoutindex| {
            //     wtx.insert(
            //         parts.addresstxoutindexes_out.data(),
            //         Slice::from(addresstxoutindex),
            //         Slice::default(),
            //     );
            // });

            block
                .txdata
                .into_iter()
                .enumerate()
                .try_for_each(|(txi, tx)| -> color_eyre::Result<()> {
                    let is_coinbase = tx.is_coinbase();

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

                    let mut bad_tx = false;

                    let (txid, prev_txindex_slice_opt) = txi_to_txid_and_prev_txindex_slice_opt
                        .remove(&txi)
                        .context("Par compute of tx should have worked")
                        .inspect_err(|_| {
                            dbg!(&txi_to_txid_and_prev_txindex_slice_opt, &txi, tx.compute_txid());
                        })?;

                    if parts.txindex_to_txid.needs(height) {
                        wtx.insert(parts.txindex_to_txid.data(), Slice::from(txindex), txid);
                    }

                    match prev_txindex_slice_opt {
                        None => {
                            if parts.txid_prefix_to_txindex.needs(height) {
                                wtx.insert(parts.txid_prefix_to_txindex.data(), txid.prefix(), Slice::from(txindex));
                            }
                        }
                        Some(prev_txindex_slice) => {
                            let prev_txid = Txid::from_slice(
                                &wtx.get(parts.txindex_to_txid.data(), &prev_txindex_slice)?
                                    .expect("To have txid for txindex"),
                            )?;

                            let only_known_dup_txids = [
                                Txid::from_str("d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599")?,
                                Txid::from_str("e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468")?,
                            ];

                            let is_dup = only_known_dup_txids.contains(&prev_txid);
                            bad_tx = is_dup;

                            if !is_dup {
                                let prev_height = Height::from(
                                    wtx.get(parts.txindex_to_height.data(), &prev_txindex_slice)?
                                        .expect("To have height"),
                                );
                                let prev_txindex = Txindex::from(prev_txindex_slice);
                                dbg!(height, txid, txindex, prev_height, prev_txid, prev_txindex,);
                                return Err(eyre!("Expect none"));
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

                    tx.input
                        .par_iter()
                        // .into_par_iter()
                        .try_fold(
                            || -> Vec<Addresstxoutindex> { vec![] },
                            |mut vec, txin| -> color_eyre::Result<Vec<Addresstxoutindex>> {
                                if is_coinbase {
                                    return Ok(vec);
                                }

                                let outpoint = txin.previous_output;
                                let txid_prefix = outpoint.txid.prefix();
                                let vout = outpoint.vout as u16;

                                let txindex = Txindex::from(
                                    wtx.get(parts.txid_prefix_to_txindex.data(), txid_prefix)?
                                        .context("Expect txid to be saved")?,
                                );

                                let txoutindex = Txoutindex::from((txindex, vout));

                                let addressindex = Addressindex::from(
                                    wtx.get(parts.txoutindex_to_addressindex.data(), Slice::from(txoutindex))?
                                        .context("Expect addressindex to not be none")
                                        .inspect_err(|_| {
                                            let height = Height::from(
                                                wtx.get(parts.txindex_to_height.data(), Slice::from(txindex))
                                                    .expect("txindex_to_height get not fail")
                                                    .expect("Expect height for txindex"),
                                            );
                                            dbg!(txid, outpoint.txid, txindex, vout, txoutindex, height);
                                        })?,
                                );

                                if bad_tx {
                                    dbg!(tx.compute_txid(), outpoint);
                                    panic!("bad tx in input")
                                }
                                if !bad_tx && parts.addresstxoutindexes_out.needs(height) {
                                    vec.push(Addresstxoutindex::from((addressindex, txoutindex)));
                                }

                                Ok(vec)
                            },
                        )
                        .try_reduce(Vec::new, |mut v, mut v2| {
                            if v.len() > v2.len() {
                                v.append(&mut v2);
                                Ok(v)
                            } else {
                                v2.append(&mut v);
                                Ok(v2)
                            }
                        })?
                        .into_iter()
                        .for_each(|addresstxoutindex| {
                            wtx.insert(
                                parts.addresstxoutindexes_out.data(),
                                Slice::from(addresstxoutindex),
                                Slice::default(),
                            );
                        });

                    tx.output
                        .into_iter()
                        .enumerate()
                        .try_for_each(|(vout, txout)| -> color_eyre::Result<()> {
                            if vout > U16MAX {
                                return Err(eyre!("vout bigger than u16::MAX"));
                            }

                            let vout = vout as u16;
                            let txoutindex = Txoutindex::from((txindex, vout));
                            let amount = Amount::from(txout.value);

                            if amount.is_zero() {
                                if parts.zero_txoutindexes.needs(height) {
                                    wtx.insert(
                                        parts.zero_txoutindexes.data(),
                                        Slice::from(txoutindex),
                                        Slice::default(),
                                    );
                                }
                            } else if parts.txoutindex_to_amount.needs(height) {
                                wtx.insert(
                                    parts.txoutindex_to_amount.data(),
                                    Slice::from(txoutindex),
                                    Slice::from(amount),
                                );
                            }

                            let script = &txout.script_pubkey;

                            let mut addressindex_local = addressindex;

                            let addresstype = Addresstype::from(script);

                            let addressbytes = Addressbytes::try_from((script, addresstype)).inspect_err(|_| {
                                // dbg!(&txout, height, txi, &tx.compute_txid());
                            });

                            if let Some(addressindex_slice) = addressbytes.as_ref().ok().and_then(|addressbytes| {
                                wtx.get(
                                    parts.addressbytes_prefix_to_addressindex.data(),
                                    Slice::from(addressbytes),
                                )
                                .ok()
                                .and_then(|s| s)
                            }) {
                                addressindex_local = addressindex_slice.into()
                            } else {
                                if parts.addressindex_to_addresstype.needs(height) {
                                    wtx.insert(
                                        parts.addressindex_to_addresstype.data(),
                                        Slice::from(addressindex_local),
                                        Slice::from(addresstype),
                                    );
                                }

                                if let Ok(addressbytes) = addressbytes {
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

                            let addresstxoutindex = Addresstxoutindex::from((addressindex_local, txoutindex));

                            if !bad_tx && parts.addresstxoutindexes_in.needs(height) {
                                wtx.insert(
                                    parts.addresstxoutindexes_in.data(),
                                    Slice::from(addresstxoutindex),
                                    Slice::default(),
                                );
                            }

                            Ok(())
                        })?;

                    txindex.increment();

                    Ok(())
                })?;

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

    let wtx = wtx_opt.take().context("option should have WriteTransaction")?;
    export(&keyspace, wtx, &parts, height)?;

    dbg!(i.elapsed());

    Ok(())
}
