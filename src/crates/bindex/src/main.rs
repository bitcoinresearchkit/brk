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
// use heed3::{Database, EnvOpenOptions};

mod structs;

use color_eyre::eyre::{eyre, ContextCompat};
use rayon::prelude::*;
use structs::{
    Addressbytes, AddressbytesPrefix, Addressindex, Addressindextxoutindex, Addresstype, Addresstypeindex, Amount,
    BlockHashPrefix, Databases, Exit, Height, StorableVecs, TxidPrefix, Txindex, Txindexvout, Txoutindex,
};

// https://github.com/fjall-rs/fjall/discussions/72
// https://github.com/romanz/electrs/blob/master/doc/schema.md

#[derive(Debug)]
enum TxInOrAddressindextoutindex<'a> {
    TxIn(&'a TxIn),
    Addressindextoutindex(Addressindextxoutindex),
}

const MONTHLY_BLOCK_TARGET: usize = 144 * 30;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let i = std::time::Instant::now();

    let check_collisions = true;

    let data_dir = Path::new("../../../../bitcoin");
    let cookie = Path::new(data_dir).join(".cookie");
    let rpc = Client::new("http://localhost:8332", Auth::CookieFile(cookie))?;

    let exit = Exit::new();

    let path_database = Path::new("./database");

    let mut vecs = StorableVecs::import(path_database)?;

    // let env = unsafe { EnvOpenOptions::new().open(Path::new("./heed"))? };
    // let mut wtxn = env.write_txn()?;

    // let addressbytes_prefix_to_addressindex: Database<AddressbytesPrefix, Addressindex> =
    //     env.create_database(&mut wtxn, Some("addressbytes_prefix_to_addressindex"))?;
    // let addressindextxoutindex_in: Database<Addressindextxoutindex, ()> =
    //     env.create_database(&mut wtxn, Some("addressindextxoutindex_in"))?;
    // let addressindextxoutindex_out: Database<Addressindextxoutindex, ()> =
    //     env.create_database(&mut wtxn, Some("addressindextxoutindex_out"))?;
    // let blockhash_prefix_to_height: Database<BlockHashPrefix, Height> =
    //     env.create_database(&mut wtxn, Some("blockhash_prefix_to_height"))?;
    // let txid_prefix_to_txindex: Database<TxidPrefix, Txindex> =
    //     env.create_database(&mut wtxn, Some("txid_prefix_to_txindex"))?;
    // let txindexvout_to_txoutindex: Database<Txindexvout, Txoutindex> =
    //     env.create_database(&mut wtxn, Some("txindexvout_to_txoutindex"))?;

    let databases = Databases::open(path_database)?;

    let mut height = Height::from(0_u32);

    let mut txindex = vecs
        .height_to_first_txindex
        .get(height)?
        .cloned()
        .unwrap_or(Txindex::default());

    let mut txoutindex = vecs
        .height_to_first_txoutindex
        .get(height)?
        .cloned()
        .unwrap_or(Txoutindex::default());

    let mut addressindex = vecs
        .height_to_first_addressindex
        .get(height)?
        .cloned()
        .unwrap_or(Addressindex::default());

    let export = |databases: Databases, vecdisks: &mut StorableVecs, height: Height| -> color_eyre::Result<()> {
        exit.block();
        println!("Exporting...");
        databases.export();
        vecdisks.flush()?;
        println!("Export done");
        exit.unblock();
        Ok(())
    };

    let mut databases_opt = Some(databases);

    biter::new(data_dir, Some(height.into()), Some(400_000), rpc)
        .iter()
        .try_for_each(|(_height, block, blockhash)| -> color_eyre::Result<()> {
            println!("Processing block {_height}...");

            height = Height::from(_height);

            let mut databases = databases_opt.take().context("option should have wtx")?;

            // if let Some(saved_blockhash) = vecdisks.height_to_blockhash.get(height)? {
            //     if &blockhash != saved_blockhash {
            //         parts.rollback_from(&mut wtx, height, &exit)?;
            //     } else {
            //         wtx_opt.replace(wtx);
            //         return Ok(());
            //     }
            // }

            // if parts.blockhash_prefix_to_height.needs(height) {
            let blockhash_prefix = BlockHashPrefix::try_from(&blockhash)?;

            // if check_collisions {
            //     if let Some(prev_height) =
            //         databases.blockhash_prefix_to_height.get(&blockhash_prefix)
            //     {
            //         dbg!(blockhash, prev_height);
            //         return Err(eyre!("Collision, expect prefix to need be set yet"));
            //     }
            // }

            databases.blockhash_prefix_to_height.insert(blockhash_prefix,height);
            // blockhash_prefix_to_height.put(&mut wtxn, &blockhash_prefix,&height);
            // }

            vecs.height_to_blockhash.push_if_needed(height, blockhash)?;
            vecs.height_to_first_txindex.push_if_needed(height, txindex)?;
            vecs.height_to_first_txoutindex.push_if_needed(height, txoutindex)?;
            vecs.height_to_first_addressindex.push_if_needed(height, addressindex)?;

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

                        let txid_prefix = TxidPrefix::try_from(&txid)?;

                        let prev_txindex_slice_opt = if check_collisions {
                            // Should only find collisions for two txids (duplicates), see below
                            databases.txid_prefix_to_txindex.get(&txid_prefix).cloned()
                        } else {
                            None
                        };

                        Ok((txid_prefix, (txid, Txindex::from(index), prev_txindex_slice_opt)))
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

                            let txindex_local = if let Some(txindex_local) = databases.txid_prefix_to_txindex
                                .get(&TxidPrefix::try_from(&txid)?)
                            {
                                *txindex_local
                            } else {
                                return Ok(TxInOrAddressindextoutindex::TxIn(txin));
                            };

                            let txindexvout = Txindexvout::from((txindex_local, vout));

                            let txoutindex =
                                *databases.txindexvout_to_txoutindex.get(&txindexvout)
                                    .context("Expect txoutindex to not be none")
                                    .inspect_err(|_| {
                                        // let height = vecdisks.txindex_to_height.get(txindex.into()).expect("txindex_to_height get not fail")
                                        // .expect("Expect height for txindex");
                                        dbg!(outpoint.txid, txindex_local, vout, txindexvout);
                                    })?;

                            let addressindex = *vecs.txoutindex_to_addressindex.get(txoutindex)?
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

                                // if parts.addressindextxoutindex_out.needs(height) {
                                    vec.push(addressindextxoutindex?);
                                // }

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
                (&TxOut, Txindexvout, Addresstype, color_eyre::Result<Addressbytes>, Option<Addressindex>)>> {
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
                                    databases.addressbytes_prefix_to_addressindex.get(
                                        &AddressbytesPrefix::try_from(addressbytes).unwrap(),
                                    ).cloned()
                                });

                                let is_new_address = addressindex_slice_opt.is_none();

                                if check_collisions && is_new_address {
                                    if let Ok(addressbytes) = &addressbytes_res {
                                        if let Some(prev) = databases.addressbytes_prefix_to_addressindex.get(
                                            &AddressbytesPrefix::try_from(addressbytes)?,
                                        ) {
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
                .try_for_each(|(txoutindex, (txout, txindexvout, addresstype, addressbytes_res, addressindex_opt))| -> color_eyre::Result<()> {
                    let amount = Amount::from(txout.value);

                    // if parts.txindexvout_to_txoutindex.needs(height) {
                        databases.txindexvout_to_txoutindex.insert(
                            txindexvout,
                            txoutindex,
                        );
                    // }

                    vecs.txoutindex_to_amount.push_if_needed(
                        txoutindex,
                        amount,
                    )?;

                    let mut addressindex_local = addressindex;

                    if let Some(addressindex) = addressindex_opt {
                        addressindex_local = addressindex;
                    } else {
                        vecs.addressindex_to_addresstype.push_if_needed(addressindex_local, addresstype)?;

                        // TODO: Create counter of other addresstypes instead
                        let addresstypeindex = Addresstypeindex::from(vecs.addresstype_to_addressbytes(addresstype).map_or(0, |vecdisk| vecdisk.len()));

                        vecs.addressindex_to_addresstypeindex.push_if_needed(addressindex_local, addresstypeindex)?;

                        if let Ok(addressbytes) = addressbytes_res {
                            // if parts.addressbytes_prefix_to_addressindex.needs(height) {
                                databases.addressbytes_prefix_to_addressindex.insert(
                                    AddressbytesPrefix::try_from(&addressbytes)?,
                                    addressindex_local,
                                );
                            // }

                            vecs.push_addressbytes_if_needed(addresstypeindex, addressbytes)?;
                        }

                        addressindex.increment();
                    }

                    new_txindexvout_to_addressindextxoutindex.insert(txindexvout, Addressindextxoutindex::from((addressindex_local, txoutindex)));

                    vecs.txoutindex_to_addressindex.push_if_needed(
                        txoutindex,
                        addressindex_local,
                    )?;

                    // if parts.addressindextxoutindex_in.needs(height) {
                        let addressindextxoutindex = Addressindextxoutindex::from((addressindex_local, txoutindex));
                        databases.addressindextxoutindex_in.insert(
                            addressindextxoutindex,
                           (),
                        );
                    // }

                    Ok(())
                })?;

            // if parts.addressindextxoutindex_out.needs(height) {
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
                                .get(&TxidPrefix::try_from(&txid)?)
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
                    databases.addressindextxoutindex_out.insert(
                        addressindextxoutindex?,
                        (),
                    );
                    Ok(())
                })?;
            // }

            drop(new_txindexvout_to_addressindextxoutindex);

            let mut txindex_to_txid: BTreeMap<Txindex, Txid> = BTreeMap::default();

            txid_prefix_to_txid_and_block_txindex_and_prev_txindex.into_iter().try_for_each(
                |(txid_prefix, (txid, index, prev_txindex_opt))| -> color_eyre::Result<()> {
                    let txindex_local = txindex + index;

                    txindex_to_txid.insert(txindex_local, txid);

                    match prev_txindex_opt {
                        None => {
                            // if parts.txid_prefix_to_txindex.needs(height) {
                            databases.txid_prefix_to_txindex.insert(txid_prefix, txindex_local);
                            // }
                        }
                        Some(prev_txindex) => {
                            // In case if we start at an already parsed height
                            if txindex_local == prev_txindex {
                                return Ok(())
                            }

                            let len = vecs.txindex_to_txid.len();
                            // Ok if `get` is not par as should happen only twice
                            let prev_txid =
                                vecs.txindex_to_txid.get(prev_txindex)?
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
                                let prev_height = vecs.txindex_to_height.get(prev_txindex)?.expect("To have height");
                                dbg!(height, txid, txindex_local, prev_height, prev_txid, prev_txindex);
                                return Err(eyre!("Expect none"));
                            }
                        }
                    }

                    Ok(())
                },
            )?;

            txindex_to_txid.into_iter().try_for_each(|(txindex, txid)| -> color_eyre::Result<()> {
                vecs.txindex_to_txid.push_if_needed(txindex, txid)?;
                vecs.txindex_to_height.push_if_needed(txindex, height)?;
                Ok(())
            })?;

            vecs.height_to_last_txindex.push_if_needed(height, txindex.decremented())?;
            vecs.height_to_last_txoutindex.push_if_needed(height, txoutindex.decremented())?;
            vecs.height_to_last_addressindex.push_if_needed(height, addressindex.decremented())?;

            let should_snapshot = _height % MONTHLY_BLOCK_TARGET == 0 && !exit.active();
            if should_snapshot {
                export(databases, &mut vecs, height)?;
                databases_opt.replace(Databases::open(path_database)?);
            } else {
                databases_opt.replace(databases);
            }

            txindex += Txindex::from(tx_len);
            txoutindex += Txoutindex::from(outputs_len);

            Ok(())
        })?;

    dbg!(i.elapsed());

    pause();

    let databases = databases_opt.take().context("option should have wtx")?;
    export(databases, &mut vecs, height)?;

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
