use std::{
    collections::BTreeMap,
    io::{Read, Write},
    path::Path,
    str::FromStr,
    thread::{self},
};

use biter::{
    bitcoin::{Transaction, TxIn, TxOut, Txid},
    bitcoincore_rpc::{Auth, Client},
};

mod structs;

use color_eyre::eyre::{eyre, ContextCompat};
use rayon::prelude::*;
use structs::{
    Addressbytes, AddressbytesPrefix, Addressindex, Addressindextxoutindex, Addresstype, Addresstypeindex, Amount,
    BlockHashPrefix, Date, Exit, Height, Stores, Timestamp, TxidPrefix, Txindex, Txindexvout, Txoutindex, Vecs,
};

// https://github.com/romanz/electrs/blob/master/doc/schema.md

#[derive(Debug)]
enum TxInOrAddressindextoutindex<'a> {
    TxIn(&'a TxIn),
    Addressindextoutindex(Addressindextxoutindex),
}

const UNSAFE_BLOCKS: u32 = 100;
const DAILY_BLOCK_TARGET: usize = 144;
const SNAPSHOT_BLOCK_RANGE: usize = DAILY_BLOCK_TARGET * 10;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let i = std::time::Instant::now();

    let check_collisions = true;

    let data_dir = Path::new("../../bitcoin");
    let cookie = Path::new(data_dir).join(".cookie");
    let rpc = Client::new("http://localhost:8332", Auth::CookieFile(cookie))?;

    let exit = Exit::new();

    let path_database = Path::new("./database");
    let path_stores = path_database.join("stores");

    let stores = Stores::open(&path_stores)?;

    let mut vecs = Vecs::import(&path_database.join("vecs"))?;

    let mut height = vecs
        .min_height()
        .unwrap_or_default()
        .min(stores.min_height())
        .and_then(|h| h.checked_sub(UNSAFE_BLOCKS))
        .map(Height::from)
        .unwrap_or_default();

    let mut txindex = vecs
        .height_to_first_txindex
        .get(height)?
        .map(|v| *v)
        .unwrap_or(Txindex::default());

    let mut txoutindex = vecs
        .height_to_first_txoutindex
        .get(height)?
        .map(|v| *v)
        .unwrap_or(Txoutindex::default());

    let mut addressindex = vecs
        .height_to_first_addressindex
        .get(height)?
        .map(|v| *v)
        .unwrap_or(Addressindex::default());

    let export = |stores: Stores, vecs: &mut Vecs, height: Height| -> color_eyre::Result<()> {
        exit.block();
        println!("Exporting...");
        // Memory: 3.76 GB
        // Real Memory: 22.47 GB
        // Private Memory: 12.44 GB
        // if height > Height::from(400_000_u32) {
        //     pause();
        // }
        // vecs.reset_cache();
        // At: 403200
        // Memory: 3.78 GB
        // Real Memory: 12.65 GB
        // Private Memory: 11.39 GB
        // if height > Height::from(400_000_u32) {
        //     pause();
        // }
        vecs.flush(height)?;
        // At: 403200
        // Memory: 3.79 GB
        // Real Memory: 12.37 GB
        // Private Memory: 10.95 GB
        // Gone up wtf
        // if height > Height::from(400_000_u32) {
        //     pause();
        // }
        stores.export(height);
        println!("Export done");
        // At: 403200
        // Memory: 2.23 GB
        // Real Memory: 1.05 GB
        // Private Memory: 0.109 GB
        // if height > Height::from(400_000_u32) {
        //     pause();
        // }
        exit.unblock();
        Ok(())
    };

    let mut stores_opt = Some(stores);

    biter::new(data_dir, Some(height.into()), Some(400_000), rpc)
        .iter()
        .try_for_each(|(_height, block, blockhash)| -> color_eyre::Result<()> {
            println!("Processing block {_height}...");

            height = Height::from(_height);
            let timestamp = Timestamp::try_from(block.header.time)?;
            let date = Date::from(&timestamp);

            let mut stores = stores_opt.take().context("option should have wtx")?;

            if let Some(saved_blockhash) = vecs.height_to_blockhash.get(height)? {
                // if &blockhash != saved_blockhash {
                if &blockhash != saved_blockhash.as_ref() {
                    todo!("Rollback not implemented");
                    // parts.rollback_from(&mut wtx, height, &exit)?;
                }
            }

            if stores.blockhash_prefix_to_height.needs(height) {
                let blockhash_prefix = BlockHashPrefix::try_from(&blockhash)?;

                if check_collisions {
                    if let Some(prev_height) =
                        stores.blockhash_prefix_to_height.get(&blockhash_prefix)
                    {
                        dbg!(blockhash, prev_height);
                        return Err(eyre!("Collision, expect prefix to need be set yet"));
                    }
                }

                stores.blockhash_prefix_to_height.insert(blockhash_prefix,height);
            }

            vecs.height_to_blockhash.push_if_needed(height, blockhash)?;
            vecs.height_to_first_txindex.push_if_needed(height, txindex)?;
            vecs.height_to_first_txoutindex.push_if_needed(height, txoutindex)?;
            vecs.height_to_first_addressindex.push_if_needed(height, addressindex)?;
            vecs.height_to_timestamp.push_if_needed(height, timestamp)?;
            vecs.height_to_date.push_if_needed(height, date)?;

            let outputs = block
                .txdata
                .iter()
                .enumerate()
                .flat_map(|(index, tx)| {
                    tx.output
                        .iter()
                        .enumerate()
                        .map(move |(vout, txout)| (Txindex::from(index), vout as u32, txout, tx))
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

                        let prev_txindex_slice_opt = if check_collisions && stores.txid_prefix_to_txindex.needs(height) {
                            // Should only find collisions for two txids (duplicates), see below
                            stores.txid_prefix_to_txindex.get(&txid_prefix).cloned()
                        } else {
                            None
                        };

                        Ok((txid_prefix, (tx, txid, Txindex::from(index), prev_txindex_slice_opt)))
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

                            let txindex_local = if let Some(txindex_local) = stores.txid_prefix_to_txindex
                                .get(&TxidPrefix::try_from(&txid)?).and_then(|txindex_local| {
                                    // Checking if not finding txindex from the future
                                    (txindex_local < &txindex).then_some(txindex_local)
                                })
                            {
                                *txindex_local
                            } else {
                                return Ok(TxInOrAddressindextoutindex::TxIn(txin));
                            };

                            let txindexvout = Txindexvout::from((txindex_local, vout));

                            let txoutindex =
                                *stores.txindexvout_to_txoutindex.get(&txindexvout)
                                    .context("Expect txoutindex to not be none")
                                    .inspect_err(|_| {
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
                                vec.push(addressindextxoutindex?);
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

                let txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle = scope.spawn(|| {
                    outputs.into_par_iter().enumerate()
                        .map(
                            #[allow(clippy::type_complexity)]
                            |(block_txoutindex, (block_txindex, vout, txout, tx))| -> color_eyre::Result<(Txoutindex,
                            (&TxOut, Txindexvout, Addresstype, color_eyre::Result<Addressbytes>, Option<Addressindex>))> {
                                let txindex_local = txindex + block_txindex;
                                let txindexvout = Txindexvout::from((txindex_local, vout));
                                let txoutindex_local = txoutindex + Txoutindex::from(block_txoutindex);

                                let script = &txout.script_pubkey;

                                let addresstype = Addresstype::from(script);

                                let addressbytes_res = Addressbytes::try_from((script, addresstype)).inspect_err(|_| {
                                    // dbg!(&txout, height, txi, &tx.compute_txid());
                                });

                                let addressindex_opt = addressbytes_res.as_ref().ok().and_then(|addressbytes| {
                                    stores.addressbytes_prefix_to_addressindex.get(
                                        &AddressbytesPrefix::from((addressbytes, addresstype)),
                                    )
                                        .cloned()
                                        // Checking if not in the future
                                        .and_then(|addressindex_local| (addressindex_local < addressindex)
                                        .then_some(addressindex_local))
                                }); // OK

                                if let Some(Some(addressindex_local)) = check_collisions.then_some(addressindex_opt) {
                                    let addressbytes = addressbytes_res.as_ref().unwrap();

                                    let prev_addresstype = *vecs.addressindex_to_addresstype.get(
                                        addressindex_local,
                                    )?.context("Expect to have address type")?;

                                    let addresstypeindex = *vecs.addressindex_to_addresstypeindex.get(
                                        addressindex_local,
                                    )?.context("Expect to have address type index")?;
                                    // Good first time
                                    // Wrong after rerun

                                    let prev_addressbytes_opt= vecs.get_addressbytes(prev_addresstype, addresstypeindex)?;

                                    let prev_addressbytes = prev_addressbytes_opt.as_ref().context("Expect to have addressbytes")?;

                                    if (vecs.addressindex_to_addresstype.hasnt(addressindex_local) && addresstype != prev_addresstype) || (stores.addressbytes_prefix_to_addressindex.needs(height) && prev_addressbytes != addressbytes) {
                                        let txid = tx.compute_txid();
                                        dbg!(_height, txid, vout, block_txindex, addresstype, prev_addresstype, prev_addressbytes, addressbytes, addressindex, addressindex_local, addresstypeindex, txout, AddressbytesPrefix::from((addressbytes, addresstype)), AddressbytesPrefix::from((prev_addressbytes, prev_addresstype)));
                                        panic!()
                                    }
                                }

                                Ok((
                                    txoutindex_local,
                                    (txout, txindexvout, addresstype, addressbytes_res, addressindex_opt),
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

            let mut already_added_addressbytes_prefix: BTreeMap<AddressbytesPrefix, Addressindex> = BTreeMap::new();

            txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt
                .into_iter()
                .try_for_each(|(txoutindex, (txout, txindexvout, addresstype, addressbytes_res, addressindex_opt))| -> color_eyre::Result<()> {
                    let amount = Amount::from(txout.value);

                    stores.txindexvout_to_txoutindex.insert_if_needed(
                        txindexvout,
                        txoutindex,
                        height,
                    );

                    vecs.txoutindex_to_amount.push_if_needed(
                        txoutindex,
                        amount,
                    )?;

                    let mut addressindex_local = addressindex;

                    let mut addressbytes_prefix = None;

                    if let Some(addressindex) = addressindex_opt.or_else(|| addressbytes_res.as_ref().ok().and_then(|addressbytes| {
                        // Check if address was first seen before in this iterator
                        // Example: https://mempool.space/address/046a0765b5865641ce08dd39690aade26dfbf5511430ca428a3089261361cef170e3929a68aee3d8d4848b0c5111b0a37b82b86ad559fd2a745b44d8e8d9dfdc0c
                        addressbytes_prefix.replace(AddressbytesPrefix::from((addressbytes, addresstype)));
                        already_added_addressbytes_prefix.get(
                        addressbytes_prefix.as_ref().unwrap(),
                        ).cloned()
                    })) {
                        addressindex_local = addressindex;
                    } else {
                        addressindex.increment();

                        // TODO: Create counter of other addresstypes instead
                        let addresstypeindex = Addresstypeindex::from(vecs.addresstype_to_addressbytes(addresstype).map_or(0, |vec| vec.len()));

                        vecs.addressindex_to_addresstype.push_if_needed(addressindex_local, addresstype)?;

                        vecs.addressindex_to_addresstypeindex.push_if_needed(addressindex_local, addresstypeindex)?;

                        if let Ok(addressbytes) = addressbytes_res {
                            let addressbytes_prefix = addressbytes_prefix.unwrap();

                            // if addressindex_local == Addressindex::from(257905_u32) || addressbytes_prefix == AddressbytesPrefix::from(
                            //     [
                            //         116_u8,
                            //         86,
                            //         96,
                            //         52,
                            //         2,
                            //         87,
                            //         151,
                            //         177,
                            //     ],
                            // ) {
                            //     dbg!(addressindex_local, addressbytes, addressbytes_prefix, addresstypeindex);
                            //     panic!();
                            // }

                            already_added_addressbytes_prefix.insert(addressbytes_prefix.clone(), addressindex_local);

                            stores.addressbytes_prefix_to_addressindex.insert_if_needed(
                                addressbytes_prefix,
                                addressindex_local,
                                height
                            );

                            vecs.push_addressbytes_if_needed(addresstypeindex, addressbytes)?;
                        }
                    }

                    let addressindextxoutindex = Addressindextxoutindex::from((addressindex_local, txoutindex));

                    new_txindexvout_to_addressindextxoutindex.insert(txindexvout, addressindextxoutindex);

                    vecs.txoutindex_to_addressindex.push_if_needed(
                        txoutindex,
                        addressindex_local,
                    )?;

                    stores.addressindextxoutindex_in.insert_if_needed(
                        addressindextxoutindex,
                        (),
                        height,
                    );

                    Ok(())
                })?;

            drop(already_added_addressbytes_prefix);

            if stores.addressindextxoutindex_out.needs(height) {
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
                                .context("txid should be in same block")?.2;
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
                    stores.addressindextxoutindex_out.insert(
                        addressindextxoutindex?,
                        (),
                    );
                    Ok(())
                })?;
            }

            drop(new_txindexvout_to_addressindextxoutindex);

            let mut txindex_to_tx_and_txid: BTreeMap<Txindex, (&Transaction, Txid)> = BTreeMap::default();

            txid_prefix_to_txid_and_block_txindex_and_prev_txindex.into_iter().try_for_each(
                |(txid_prefix, (tx, txid, index, prev_txindex_opt))| -> color_eyre::Result<()> {
                    let txindex_local = txindex + index;

                    txindex_to_tx_and_txid.insert(txindex_local, (tx, txid));

                    match prev_txindex_opt {
                        None => {
                            stores.txid_prefix_to_txindex.insert_if_needed(txid_prefix, txindex_local, height);
                        },
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

                            // #[allow(clippy::redundant_locals)]
                            // let prev_txid = prev_txid;
                            let prev_txid = prev_txid.as_ref();

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

            txindex_to_tx_and_txid.into_iter().try_for_each(|(txindex, (tx, txid))| -> color_eyre::Result<()> {
                vecs.txindex_to_txversion.push_if_needed(txindex, tx.version)?;
                vecs.txindex_to_txid.push_if_needed(txindex, txid)?;
                vecs.txindex_to_height.push_if_needed(txindex, height)?;
                vecs.txindex_to_inputcount.push_if_needed(txindex, tx.input.len() as u32)?;
                vecs.txindex_to_outputcount.push_if_needed(txindex, tx.output.len() as u32)?;
                Ok(())
            })?;

            vecs.height_to_last_txindex.push_if_needed(height, txindex.decremented())?;
            vecs.height_to_last_txoutindex.push_if_needed(height, txoutindex.decremented())?;
            vecs.height_to_last_addressindex.push_if_needed(height, addressindex.decremented())?;

            let should_snapshot = _height % SNAPSHOT_BLOCK_RANGE == 0 && !exit.active();
            if should_snapshot {
                export(stores, &mut vecs, height)?;
                stores_opt.replace(Stores::open(&path_stores)?);
            } else {
                stores_opt.replace(stores);
            }

            txindex += Txindex::from(tx_len);
            txoutindex += Txoutindex::from(outputs_len);

            Ok(())
        })?;

    let stores = stores_opt.take().context("option should have wtx")?;
    export(stores, &mut vecs, height)?;

    dbg!(i.elapsed());

    pause();

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
