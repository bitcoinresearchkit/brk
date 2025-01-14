use std::{
    collections::BTreeMap,
    io::{Read, Write},
    path::Path,
    str::FromStr,
    thread::{self},
};

use biter::{
    bitcoin::{TxIn, TxOut, Txid},
    bitcoincore_rpc::{Auth, Client},
};

mod structs;

use color_eyre::eyre::{eyre, ContextCompat};
use fjall::{PersistMode, Slice, TransactionalKeyspace, WriteTransaction};
use rayon::prelude::*;
use structs::{
    Addressbytes, AddressbytesPrefix, Addressindex, Addressindextxoutindex, Addresstype, Addresstypeindex, Amount,
    AnyVecdisk, BlockHashPrefix, Exit, Height, Partitions, TxidPrefix, Txindex, Txindexvout, Txoutindex, Vecdisks,
};

// https://github.com/fjall-rs/fjall/discussions/72
// https://github.com/romanz/electrs/blob/master/doc/schema.md

#[derive(Debug)]
enum TxInOrAddressindextoutindex<'a> {
    TxIn(&'a TxIn),
    Addressindextoutindex(Addressindextxoutindex),
}

fn main() -> color_eyre::Result<()> {
    let i = std::time::Instant::now();

    let check_collisions = true;

    let data_dir = Path::new("../../../../bitcoin");
    let cookie = Path::new(data_dir).join(".cookie");
    let rpc = Client::new("http://localhost:8332", Auth::CookieFile(cookie))?;

    let exit = Exit::new();

    let path_database = Path::new("./database");

    let mut vecdisks = Vecdisks::import(path_database)?;

    let keyspace = fjall::Config::new(path_database).open_transactional()?;

    let parts = Partitions::import(&keyspace, &exit)?;

    let wtx = keyspace.write_tx();

    let mut height = parts.start_height();

    let mut txindex = vecdisks
        .height_to_first_txindex
        .get(height)?
        .cloned()
        .unwrap_or(Txindex::default());

    let mut txoutindex = vecdisks
        .height_to_first_txoutindex
        .get(height)?
        .cloned()
        .unwrap_or(Txoutindex::default());

    let mut addressindex = vecdisks
        .height_to_first_addressindex
        .get(height)?
        .cloned()
        .unwrap_or(Addressindex::default());

    let export = |keyspace: &TransactionalKeyspace,
                  mut wtx: WriteTransaction,
                  parts: &Partitions,
                  vecdisks: &mut Vecdisks,
                  height: Height|
     -> color_eyre::Result<()> {
        parts.udpate_meta(&mut wtx, height);
        exit.block();
        println!("Exporting...");
        wtx.commit()?;
        keyspace.persist(PersistMode::SyncAll)?;
        vecdisks.flush()?;
        println!("Export done");
        exit.unblock();
        Ok(())
    };

    let mut wtx_opt = Some(wtx);

    biter::new(data_dir, Some(height.into()), Some(400_000), rpc)
        .iter()
        .try_for_each(|(_height, block, blockhash)| -> color_eyre::Result<()> {
            println!("Processing block {_height}...");

            height = Height::from(_height);

            let mut wtx = wtx_opt.take().context("option should have wtx")?;

            // if let Some(saved_blockhash) = vecdisks.height_to_blockhash.get(height)? {
            //     if &blockhash != saved_blockhash {
            //         parts.rollback_from(&mut wtx, height, &exit)?;
            //     } else {
            //         wtx_opt.replace(wtx);
            //         return Ok(());
            //     }
            // }

            if parts.blockhash_prefix_to_height.needs(height) {
                let blockhash_prefix = BlockHashPrefix::from(&blockhash);

                if check_collisions {

                    if let Some(prev_height_slice) =
                        wtx.get(parts.blockhash_prefix_to_height.data(), blockhash_prefix.clone())?
                    {
                        dbg!(blockhash, Height::try_from(prev_height_slice)?);
                        return Err(eyre!("Collision, expect prefix to need be set yet"));
                    }
                }

                wtx.insert(parts.blockhash_prefix_to_height.data(), blockhash_prefix.clone(),Slice::from(height));
            }

            vecdisks.height_to_blockhash.push_if_needed(height, blockhash)?;
            vecdisks.height_to_first_txindex.push_if_needed(height, txindex)?;
            vecdisks.height_to_first_txoutindex.push_if_needed(height, txoutindex)?;
            vecdisks.height_to_first_addressindex.push_if_needed(height, addressindex)?;

            let outputs = block
                .txdata
                .iter()
                .enumerate()
                .flat_map(|(index, tx)| {
                    tx.output
                        .iter()
                        .enumerate()
                        .map(move |(vout, txout)| (Txindex::from(index), vout as u32, txout))
                }).collect::<Vec<_>>();

            let tx_len = block.txdata.len();
            let outputs_len = outputs.len();

            let (txid_prefix_to_txid_and_block_txindex_and_prev_txindex_join_handle, txin_or_addressindextxoutindex_vec_handle, txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle) = thread::scope(|scope| {
                let txid_prefix_to_txid_and_block_txindex_and_prev_txindex_handle = scope.spawn(|| -> color_eyre::Result<_> {
                    block
                    .txdata
                    .par_iter()
                    .enumerate()
                    .map(|(index, tx)| -> color_eyre::Result<_> {
                        let txid = tx.compute_txid();

                        let prev_txindex_slice_opt = if check_collisions {
                            // Should only find collisions for two txids (duplicates), see below
                            wtx.get(parts.txid_prefix_to_txindex.data(), TxidPrefix::from(&txid).clone())?.map(Txindex::try_from)
                        } else {
                            None
                        };

                        Ok((TxidPrefix::from(&txid), (txid, Txindex::from(index), prev_txindex_slice_opt)))
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

                let txin_or_addressindextxoutindex_vec_handle = scope.spawn(|| -> color_eyre::Result<Vec<TxInOrAddressindextoutindex>> {
                    block
                        .txdata
                        .par_iter()
                        .filter(|tx| !tx.is_coinbase())
                        .flat_map(|tx| &tx.input)
                        .map(|txin| -> color_eyre::Result<_> {
                            let outpoint = txin.previous_output;
                            let txid = outpoint.txid;
                            let vout = outpoint.vout;

                            let txindex_local = if let Some(txindex_local) = wtx
                                .get(parts.txid_prefix_to_txindex.data(), TxidPrefix::from(&txid).clone())?
                                .map(Txindex::try_from)
                            {
                                txindex_local
                            } else {
                                return Ok(TxInOrAddressindextoutindex::TxIn(txin));
                            }?;

                            let txindexvout = Txindexvout::from((txindex_local, vout));

                            let txoutindex = Txoutindex::try_from(
                                wtx.get(parts.txindexvout_to_txoutindex.data(), Slice::from(txindexvout))?
                                    .context("Expect txoutindex to not be none")
                                    .inspect_err(|_| {
                                        // let height = vecdisks.txindex_to_height.get(txindex.into()).expect("txindex_to_height get not fail")
                                        // .expect("Expect height for txindex");
                                        dbg!(outpoint.txid, txindex_local, vout, txindexvout);
                                    })?,
                            )?;

                            let addressindex = *vecdisks.txoutindex_to_addressindex.get(txoutindex)?
                                .context("Expect addressindex to not be none")
                                .inspect_err(|_| {
                                    // let height = vecdisks.txindex_to_height.get(txindex.into()).expect("txindex_to_height get not fail")
                                    // .expect("Expect height for txindex");
                                    dbg!(outpoint.txid, txindex_local, vout, txindexvout);
                                })?;

                            Ok(TxInOrAddressindextoutindex::Addressindextoutindex(Addressindextxoutindex::from((
                                addressindex,
                                txoutindex,
                            ))))
                        })
                        .try_fold(
                            Vec::new,
                            |mut vec, addressindextxoutindex| {
                                // There is no need to check for bad_tx as there are only 2 instances known
                                // Which you can find below and which are coinbase tx and thus which are already filtered
                                if parts.addressindextxoutindex_out.needs(height) {
                                    vec.push(addressindextxoutindex?);
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
                (&TxOut, Txindexvout, Addresstype, color_eyre::Result<Addressbytes>, Option<Slice>)>> {
                    outputs.into_par_iter().enumerate()
                        .map(
                            |(block_txoutindex, (block_txindex, vout, txout))| {
                                let txindex_local = txindex + block_txindex;
                                let txindexvout = Txindexvout::from((txindex_local, vout));
                                let txoutindex_local = txoutindex + Txoutindex::from(block_txoutindex);

                                let script = &txout.script_pubkey;

                                let addresstype = Addresstype::from(script);

                                let addressbytes_res = Addressbytes::try_from((script, addresstype)).inspect_err(|_| {
                                    // dbg!(&txout, height, txi, &tx.compute_txid());
                                });

                                let addressindex_slice_opt = addressbytes_res.as_ref().ok().and_then(|addressbytes| {
                                    let prefix = AddressbytesPrefix::from(addressbytes);
                                    wtx.get(
                                        parts.addressbytes_prefix_to_addressindex.data(),
                                        prefix.clone(),
                                    )
                                    .ok()
                                    .and_then(|s| s)
                                });

                                let is_new_address = addressindex_slice_opt.is_none();

                                if check_collisions && is_new_address {
                                    if let Ok(addressbytes) = &addressbytes_res {
                                        let prefix = AddressbytesPrefix::from(addressbytes);
                                        if let Some(prev) = wtx.get(
                                            parts.addressbytes_prefix_to_addressindex.data(),
                                            prefix.clone(),
                                        )? {
                                            dbg!(prev);
                                            return Err(eyre!("addressbytes_prefix_to_addressindex collision, expect none"));
                                        }
                                    }
                                }

                                Ok((
                                    txoutindex_local,
                                    (txout, txindexvout, addresstype, addressbytes_res, addressindex_slice_opt),
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

                (txid_prefix_to_txid_and_block_txindex_and_prev_txindex_handle.join(), txin_or_addressindextxoutindex_vec_handle.join(), txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle.join())
            });

            let txid_prefix_to_txid_and_block_txindex_and_prev_txindex = txid_prefix_to_txid_and_block_txindex_and_prev_txindex_join_handle.ok().context("Expect txid_prefix_to_txid_and_block_txindex_and_prev_txindex_join_handle to join")??;

            let txin_or_addressindextxoutindex_vec = txin_or_addressindextxoutindex_vec_handle.ok().context("Export txin_or_addressindextxoutindex_vec_handle to join")??;

            let txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt = txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle.ok().context("Expect txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle to join")??;

            let mut new_txindexvout_to_addressindextxoutindex: BTreeMap<Txindexvout, Addressindextxoutindex> = BTreeMap::new();

            txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt
                .into_iter()
                .try_for_each(|(txoutindex, (txout, txindexvout, addresstype, addressbytes_res, addressindex_slice_opt))| -> color_eyre::Result<()> {
                    let amount = Amount::from(txout.value);

                    if parts.txindexvout_to_txoutindex.needs(height) {
                        wtx.insert(
                            parts.txindexvout_to_txoutindex.data(),
                            Slice::from(txindexvout),
                            Slice::from(txoutindex),
                        );
                    }

                    vecdisks.txoutindex_to_amount.push_if_needed(
                        txoutindex,
                        amount,
                    )?;

                    let mut addressindex_local = addressindex;

                    if let Some(addressindex_slice) = addressindex_slice_opt {
                        addressindex_local = Addressindex::try_from(addressindex_slice)?;
                    } else {
                        vecdisks.addressindex_to_addresstype.push_if_needed(addressindex_local, addresstype)?;

                        // TODO: Create counter of other addresstypes instead
                        let addresstypeindex = Addresstypeindex::from(vecdisks.addresstype_to_addressvecdisk(addresstype).map_or(0, |vecdisk| vecdisk.len()));

                        vecdisks.addressindex_to_addresstypeindex.push_if_needed(addressindex_local, addresstypeindex)?;

                        if let Ok(addressbytes) = addressbytes_res {
                            if parts.addressbytes_prefix_to_addressindex.needs(height) {
                                wtx.insert(
                                    parts.addressbytes_prefix_to_addressindex.data(),
                                    AddressbytesPrefix::from(&addressbytes).clone(),
                                    Slice::from(addressindex_local),
                                );
                            }

                            vecdisks.push_addressbytes_if_needed(addresstypeindex, addressbytes)?;
                        }

                        addressindex.increment();
                    }

                    new_txindexvout_to_addressindextxoutindex.insert(txindexvout, Addressindextxoutindex::from((addressindex_local, txoutindex)));

                    vecdisks.txoutindex_to_addressindex.push_if_needed(
                        txoutindex,
                        addressindex_local,
                    )?;

                    if parts.addressindextxoutindex_in.needs(height) {
                        let addressindextxoutindex = Addressindextxoutindex::from((addressindex_local, txoutindex));
                        wtx.insert(
                            parts.addressindextxoutindex_in.data(),
                            Slice::from(addressindextxoutindex),
                            Slice::from(&[]),
                        );
                    }

                    Ok(())
                })?;

            if parts.addressindextxoutindex_out.needs(height) {
                txin_or_addressindextxoutindex_vec
                .into_iter()
                .map(|txin_or_addressindextxoutindex| -> color_eyre::Result<Addressindextxoutindex> {
                    match txin_or_addressindextxoutindex {
                        TxInOrAddressindextoutindex::Addressindextoutindex(addressindextxoutindex) => Ok(addressindextxoutindex),
                        TxInOrAddressindextoutindex::TxIn(txin) => {
                            let outpoint = txin.previous_output;
                            let txid = outpoint.txid;
                            let vout = outpoint.vout;
                            let index = txid_prefix_to_txid_and_block_txindex_and_prev_txindex
                                .get(&TxidPrefix::from(&txid))
                                .context("txid should be in same block")?.1;
                            let txindex_local = txindex + index;

                            let txindexvout = Txindexvout::from((txindex_local, vout));

                            new_txindexvout_to_addressindextxoutindex
                                .remove(&txindexvout)
                                .context("should have found addressindex from same block").inspect_err(|_| {
                                    dbg!(&new_txindexvout_to_addressindextxoutindex, txin, txindexvout, txid);
                                })
                        }
                    }
                })
                .try_for_each(|addressindextxoutindex| -> color_eyre::Result<()> {
                    wtx.insert(
                        parts.addressindextxoutindex_out.data(),
                        Slice::from(addressindextxoutindex?),
                        Slice::from(&[]),
                    );
                    Ok(())
                })?;
            }

            drop(new_txindexvout_to_addressindextxoutindex);

            let mut txindex_to_txid: BTreeMap<Txindex, Txid> = BTreeMap::default();

            txid_prefix_to_txid_and_block_txindex_and_prev_txindex.into_iter().try_for_each(
                |(txid_prefix, (txid, index, prev_txindex_opt))| -> color_eyre::Result<()> {
                    let txindex_local = txindex + index;

                    txindex_to_txid.insert(txindex_local, txid);

                    match prev_txindex_opt {
                        None => {
                            if parts.txid_prefix_to_txindex.needs(height) {
                                wtx.insert(parts.txid_prefix_to_txindex.data(), txid_prefix.clone(), Slice::from(txindex_local));
                            }
                        }
                        Some(prev_txindex_res) => {
                            let prev_txindex = prev_txindex_res?;

                            // In case if we start at an already parsed height
                            if txindex_local == prev_txindex {
                                return Ok(())
                            }

                            let len = vecdisks.txindex_to_txid.len();
                            // Ok if `get` is not par as should happen only twice
                            let prev_txid =
                                vecdisks.txindex_to_txid.get(prev_txindex)?
                                    .context("To have txid for txindex").inspect_err(|_| {
                                        dbg!(txindex_local, txid, len);
                                    })?;

                            // If another Txid needs to be added to the list
                            // We need to check that it's also a coinbase tx otherwise par_iter inputs needs to be updated
                            let only_known_dup_txids = [
                                Txid::from_str("d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599")?,
                                Txid::from_str("e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468")?,
                            ];

                            let is_dup = only_known_dup_txids.contains(prev_txid);

                            if !is_dup {
                                let prev_height = vecdisks.txindex_to_height.get(prev_txindex)?.expect("To have height");
                                dbg!(height, txid, txindex_local, prev_height, prev_txid, prev_txindex);
                                return Err(eyre!("Expect none"));
                            }
                        }
                    }

                    Ok(())
                },
            )?;

            txindex_to_txid.into_iter().try_for_each(|(txindex, txid)| -> color_eyre::Result<()> {
                vecdisks.txindex_to_txid.push_if_needed(txindex, txid)?;
                vecdisks.txindex_to_height.push_if_needed(txindex, height)?;
                Ok(())
            })?;

            vecdisks.height_to_last_txindex.push_if_needed(height, txindex.decremented())?;
            vecdisks.height_to_last_txoutindex.push_if_needed(height, txoutindex.decremented())?;
            vecdisks.height_to_last_addressindex.push_if_needed(height, addressindex.decremented())?;

            let should_snapshot = _height % 100 == 0 && !exit.active();
            if should_snapshot {
                export(&keyspace, wtx, &parts, &mut vecdisks, height)?;
                wtx_opt.replace(keyspace.write_tx());
            } else {
                wtx_opt.replace(wtx);
            }

            txindex += Txindex::from(tx_len);
            txoutindex += Txoutindex::from(outputs_len);

            Ok(())
        })?;

    dbg!(i.elapsed());

    pause();

    let wtx = wtx_opt.take().context("option should have wtx")?;
    export(&keyspace, wtx, &parts, &mut vecdisks, height)?;

    pause();

    dbg!(i.elapsed());

    Ok(())
}

fn pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
