use std::{collections::BTreeMap, path::Path, str::FromStr, thread};

use biter::{
    bitcoin::{hashes::Hash, TxIn, TxOut, Txid},
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

enum TxInOrAddresstxoutindex<'a> {
    TxIn(&'a TxIn),
    Addresstxoutindex(Addresstxoutindex),
}

fn main() -> color_eyre::Result<()> {
    let i = std::time::Instant::now();

    let check_collisions = true;

    let data_dir = Path::new("../../../../bitcoin");
    let cookie = Path::new(data_dir).join(".cookie");
    let rpc = Client::new("http://localhost:8332", Auth::CookieFile(cookie))?;

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
                if check_collisions {
                    if let Some(prev_height_slice) =
                        wtx.get(parts.blockhash_prefix_to_height.data(), blockhash.prefix())?
                    {
                        dbg!(blockhash, Height::from(prev_height_slice));
                        return Err(eyre!("Collision, expect prefix to need be set yet"));
                    }
                }

                wtx.insert(parts.blockhash_prefix_to_height.data(), blockhash.prefix(),Slice::from(height));
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

            let (txid_prefix_slice_to_txid_and_index_join_handle, txin_or_addresstxoutindex_vec_handle, txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle) = thread::scope(|scope| {
                let txid_prefix_slice_to_txid_and_index_handle = scope.spawn(|| -> color_eyre::Result<_> {
                    block
                    .txdata
                    .par_iter()
                    .enumerate()
                    .map(|(index, tx)| -> color_eyre::Result<_> {
                        let txid = tx.compute_txid();

                        // Could be removed as should only trigger for two txid (duplicates)
                        let prev_txindex_slice_opt = wtx.get(parts.txid_prefix_to_txindex.data(), txid.prefix())?;

                        Ok((Slice::from(txid.prefix()), (txid, index, prev_txindex_slice_opt)))
                    })
                    .try_fold(
                        BTreeMap::new,
                        |mut map, tuple| {
                            let (key, value) = tuple?;
                            map.insert(key, value);
                            Ok(map)
                        },
                    )
                    .try_reduce(BTreeMap::new, |mut map, mut map2| {
                        if map.len() > map2.len() {
                            map.append(&mut map2);
                            Ok(map)
                        } else {
                            map2.append(&mut map);
                            Ok(map2)
                        }
                    })});

                let txin_or_addresstxoutindex_vec_handle = scope.spawn(|| -> color_eyre::Result<Vec<TxInOrAddresstxoutindex>> {
                    block
                        .txdata
                        .par_iter()
                        .filter(|tx| !tx.is_coinbase())
                        .flat_map(|tx| &tx.input)
                        .map(|txin| -> color_eyre::Result<_> {
                            let outpoint = txin.previous_output;
                            let txid_prefix = outpoint.txid.prefix();
                            let vout = outpoint.vout as u16;

                            let txindex = if let Some(txindex) = wtx
                                .get(parts.txid_prefix_to_txindex.data(), txid_prefix)?
                                .map(Txindex::from)
                            {
                                txindex
                            } else {
                                return Ok(TxInOrAddresstxoutindex::TxIn(txin));
                            };

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
                                        dbg!(outpoint.txid, txindex, vout, txoutindex, height);
                                    })?,
                            );

                            Ok(TxInOrAddresstxoutindex::Addresstxoutindex(Addresstxoutindex::from((
                                addressindex,
                                txoutindex,
                            ))))
                        })
                        .try_fold(
                            Vec::new,
                            |mut vec, addresstxoutindex| {
                                // There is no need to check for bad_tx as there are only 2 instances known
                                // Which you can find below and which are coinbase tx and thus which are already filtered
                                if parts.addresstxoutindexes_out.needs(height) {
                                    vec.push(addresstxoutindex?);
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
                        })
                });

                let txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle = scope.spawn(|| -> color_eyre::Result<BTreeMap<Txoutindex,
                (&TxOut, Addresstype, color_eyre::Result<Addressbytes>, Option<Slice>)>> {
                    block
                        .txdata
                        .par_iter()
                        .enumerate()
                        .flat_map(|(index, tx)| {
                            tx.output
                                .par_iter()
                                .enumerate()
                                .map(move |(vout, txout)| (index, vout, txout))
                        })
                        .map(
                            |(index, vout, txout)| {
                                if vout > U16MAX {
                                    return Err(eyre!("vout bigger than u16::MAX"));
                                }

                                let vout = vout as u16;
                                let txindex = txindex + index;
                                let txoutindex = Txoutindex::from((txindex, vout));

                                let script = &txout.script_pubkey;

                                let addresstype = Addresstype::from(script);

                                let addressbytes_res = Addressbytes::try_from((script, addresstype)).inspect_err(|_| {
                                    // dbg!(&txout, height, txi, &tx.compute_txid());
                                });

                                let addressindex_slice_opt = addressbytes_res.as_ref().ok().and_then(|addressbytes| {
                                    wtx.get(
                                        parts.addressbytes_prefix_to_addressindex.data(),
                                        Slice::from(addressbytes),
                                    )
                                    .ok()
                                    .and_then(|s| s)
                                });

                                let is_new_address = addressindex_slice_opt.is_none();

                                if check_collisions && is_new_address {
                                    if let Ok(addressbytes) = &addressbytes_res {
                                        if let Some(prev) = wtx.get(
                                            parts.addressbytes_prefix_to_addressindex.data(),
                                            Slice::from(addressbytes),
                                        )? {
                                            dbg!(prev);
                                            return Err(eyre!("addressbytes_prefix_to_addressindex collision, expect none"));
                                        }
                                    }
                                }

                                Ok((
                                    txoutindex,
                                    (txout, addresstype, addressbytes_res, addressindex_slice_opt),
                                ))
                            },
                        )
                        .try_fold(
                            BTreeMap::new,
                            |mut map, tuple| -> color_eyre::Result<_> {
                                let (key, value) = tuple?;
                                map.insert(key, value);
                                Ok(map)
                            },
                        )
                        .try_reduce(BTreeMap::new, |mut map, mut map2| {
                            if map.len() > map2.len() {
                                map.append(&mut map2);
                                Ok(map)
                            } else {
                                map2.append(&mut map);
                                Ok(map2)
                            }
                        })
                });

                (txid_prefix_slice_to_txid_and_index_handle.join(), txin_or_addresstxoutindex_vec_handle.join(), txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle.join())
            });

            let txid_prefix_slice_to_txid_and_index = txid_prefix_slice_to_txid_and_index_join_handle.ok().context("Expect txid_prefix_slice_to_txid_and_index_join_handle to join")??;

            let txin_or_addresstxoutindex_vec = txin_or_addresstxoutindex_vec_handle.ok().context("Export txin_or_addresstxoutindex_vec_handle to join")??;

            let txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt = txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle.ok().context("Expect txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle to join")??;

            let mut new_txoutindex_to_addressindex: BTreeMap<Txoutindex, Addressindex> = BTreeMap::new();

            txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt
                .into_iter()
                .try_for_each(|(txoutindex, (txout, addresstype, addressbytes_res, addressindex_slice_opt))| -> color_eyre::Result<()> {
                    let amount = Amount::from(txout.value);

                    let mut addressindex_local = addressindex;

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

                    if let Some(addressindex_slice) = addressindex_slice_opt {
                        addressindex_local = addressindex_slice.into()
                    } else {
                        new_txoutindex_to_addressindex.insert(txoutindex, addressindex_local);

                        if parts.addressindex_to_addresstype.needs(height) {
                            wtx.insert(
                                parts.addressindex_to_addresstype.data(),
                                Slice::from(addressindex_local),
                                Slice::from(addresstype),
                            );
                        }

                        if let Ok(addressbytes) = addressbytes_res {
                            if parts.addressbytes_prefix_to_addressindex.needs(height) {
                                wtx.insert(
                                    parts.addressbytes_prefix_to_addressindex.data(),
                                    Slice::from(addressbytes.prefix()),
                                    Slice::from(addressindex_local),
                                );
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


                    if parts.addresstxoutindexes_in.needs(height) {
                        let addresstxoutindex = Addresstxoutindex::from((addressindex_local, txoutindex));
                        wtx.insert(
                            parts.addresstxoutindexes_in.data(),
                            Slice::from(addresstxoutindex),
                            Slice::default(),
                        );
                    }

                    Ok(())
                })?;

            if parts.addresstxoutindexes_out.needs(height) {
                txin_or_addresstxoutindex_vec
                .into_iter()
                .map(|txin_or_addresstxoutindex| -> color_eyre::Result<Addresstxoutindex> {
                    match txin_or_addresstxoutindex {
                        TxInOrAddresstxoutindex::Addresstxoutindex(addresstxoutindex) => Ok(addresstxoutindex),
                        TxInOrAddresstxoutindex::TxIn(txin) => {
                            let outpoint = txin.previous_output;
                            let txid = outpoint.txid;
                            let vout = outpoint.vout as u16;
                            let index = txid_prefix_slice_to_txid_and_index
                                .get(txid.prefix())
                                .context("txid should be in same block")?.1;
                            let txindex = txindex + index;
                            let txoutindex = Txoutindex::from((txindex, vout));
                            let addressindex = new_txoutindex_to_addressindex
                                .remove(&txoutindex)
                                .context("should have found addressindex from same block").inspect_err(|_| {
                                    dbg!(txin, txoutindex);
                                })?;

                            Ok(Addresstxoutindex::from((addressindex, txoutindex)))
                        }
                    }
                })
                .try_for_each(|addresstxoutindex| -> color_eyre::Result<()> {
                    wtx.insert(
                        parts.addresstxoutindexes_out.data(),
                        Slice::from(addresstxoutindex?),
                        Slice::default(),
                    );
                    Ok(())
                })?;
            }

            txid_prefix_slice_to_txid_and_index.into_iter().try_for_each(
                |(txid_prefix, (txid, index, prev_txindex_slice_opt))| -> color_eyre::Result<()> {
                    let txindex = txindex + index;

                    if index == 0 && parts.height_to_first_txindex.needs(height) {
                        wtx.insert(
                            parts.height_to_first_txindex.data(),
                            Slice::from(height),
                            Slice::from(txindex),
                        );
                    }
                    if index == last_txi && parts.height_to_last_txindex.needs(height) {
                        wtx.insert(
                            parts.height_to_last_txindex.data(),
                            Slice::from(height),
                            Slice::from(txindex),
                        );
                    }

                    if parts.txindex_to_txid.needs(height) {
                        wtx.insert(parts.txindex_to_txid.data(), Slice::from(txindex), txid);
                    }

                    match prev_txindex_slice_opt {
                        None => {
                            if parts.txid_prefix_to_txindex.needs(height) {
                                wtx.insert(parts.txid_prefix_to_txindex.data(), txid_prefix, Slice::from(txindex));
                            }
                        }
                        Some(prev_txindex_slice) => {
                            // Ok if `get` is not par as should happen only twice
                            let prev_txid = Txid::from_slice(
                                &wtx.get(parts.txindex_to_txid.data(), &prev_txindex_slice)?
                                    .expect("To have txid for txindex"),
                            )?;

                            // If another Txid needs to be added to the list
                            // We need to check that it's also a coinbase tx otherwise par_iter inputs needs to be updated
                            let only_known_dup_txids = [
                                Txid::from_str("d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599")?,
                                Txid::from_str("e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468")?,
                            ];

                            let is_dup = only_known_dup_txids.contains(&prev_txid);

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

            txindex += txlen;

            Ok(())
        })?;

    // dbg!(i.elapsed());

    // loop {}

    let wtx = wtx_opt.take().context("option should have wtx")?;
    export(&keyspace, wtx, &parts, height)?;

    dbg!(i.elapsed());

    Ok(())
}
