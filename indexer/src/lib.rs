use std::{
    collections::BTreeMap,
    io::{Read, Write},
    path::Path,
    str::FromStr,
    thread::{self, sleep},
    time::Duration,
};

pub use biter::*;

use bitcoin::{Transaction, TxIn, TxOut, Txid};
use color_eyre::eyre::{eyre, ContextCompat};
use exit::Exit;
use rayon::prelude::*;
use storable_vec::CACHED_GETS;

mod storage;
mod structs;

pub use storage::{AnyStorableVec, StorableVec, Store, StoreMeta};
pub use structs::Version;

use storage::{Fjalls, StorableVecs};
pub use structs::{
    AddressHash, Addressbytes, Addressindex, Addresstype, Amount, BlockHashPrefix, Height, Timestamp, TxidPrefix,
    Txindex, Txinindex, Txoutindex, Vin, Vout,
};

const UNSAFE_BLOCKS: u32 = 100;
const SNAPSHOT_BLOCK_RANGE: usize = 1000;

pub struct Indexer<const MODE: u8> {
    pub vecs: StorableVecs<MODE>,
    pub trees: Fjalls,
}

impl<const MODE: u8> Indexer<MODE> {
    pub fn import(indexes_dir: &Path) -> color_eyre::Result<Self> {
        let vecs = StorableVecs::import(&indexes_dir.join("vecs"))?;
        let trees = Fjalls::import(&indexes_dir.join("fjall"))?;
        Ok(Self { vecs, trees })
    }
}

impl Indexer<CACHED_GETS> {
    pub fn index(&mut self, bitcoin_dir: &Path, rpc: rpc::Client, exit: &Exit) -> color_eyre::Result<()> {
        let check_collisions = true;

        let vecs = &mut self.vecs;
        let trees = &mut self.trees;

        let mut height = vecs
            .min_height()
            .unwrap_or_default()
            .min(trees.min_height())
            .and_then(|h| h.checked_sub(UNSAFE_BLOCKS))
            .map(Height::from)
            .unwrap_or_default();

        let mut txindex_global = vecs.height_to_first_txindex.get_or_default(height)?;
        let mut txinindex_global = vecs.height_to_first_txinindex.get_or_default(height)?;
        let mut txoutindex_global = vecs.height_to_first_txoutindex.get_or_default(height)?;
        let mut addressindex_global = vecs.height_to_first_addressindex.get_or_default(height)?;
        let mut emptyindex_global = vecs.height_to_first_emptyindex.get_or_default(height)?;
        let mut multisigindex_global = vecs.height_to_first_multisigindex.get_or_default(height)?;
        let mut opreturnindex_global = vecs.height_to_first_opreturnindex.get_or_default(height)?;
        let mut pushonlyindex_global = vecs.height_to_first_pushonlyindex.get_or_default(height)?;
        let mut unknownindex_global = vecs.height_to_first_unknownindex.get_or_default(height)?;
        let mut p2pk33index_global = vecs.height_to_first_p2pk33index.get_or_default(height)?;
        let mut p2pk65index_global = vecs.height_to_first_p2pk65index.get_or_default(height)?;
        let mut p2pkhindex_global = vecs.height_to_first_p2pkhindex.get_or_default(height)?;
        let mut p2shindex_global = vecs.height_to_first_p2shindex.get_or_default(height)?;
        let mut p2trindex_global = vecs.height_to_first_p2trindex.get_or_default(height)?;
        let mut p2wpkhindex_global = vecs.height_to_first_p2wpkhindex.get_or_default(height)?;
        let mut p2wshindex_global = vecs.height_to_first_p2wshindex.get_or_default(height)?;

        let export =
            |trees: &mut Fjalls, vecs: &mut StorableVecs<CACHED_GETS>, height: Height| -> color_eyre::Result<()> {
                println!("Exporting...");

                exit.block();

                thread::scope(|scope| -> color_eyre::Result<()> {
                    let vecs_handle = scope.spawn(|| vecs.flush(height));
                    trees.commit(height)?;
                    vecs_handle.join().unwrap()?;
                    Ok(())
                })?;

                exit.unblock();

                Ok(())
            };

        biter::new(bitcoin_dir, Some(height.into()), None, rpc)
            .iter()
            .try_for_each(|(_height, block, blockhash)| -> color_eyre::Result<()> {
                println!("Processing block {_height}...");

                height = Height::from(_height);

                if let Some(saved_blockhash) = vecs.height_to_blockhash.get(height)? {
                    if &blockhash != saved_blockhash.as_ref() {
                        todo!("Rollback not implemented");
                        // trees.rollback_from(&mut rtx, height, &exit)?;
                    }
                }

                let blockhash_prefix = BlockHashPrefix::try_from(&blockhash)?;

                if trees
                    .blockhash_prefix_to_height
                    .get(&blockhash_prefix)?
                    .is_some_and(|prev_height| *prev_height != height)
                {
                    dbg!(blockhash);
                    return Err(eyre!("Collision, expect prefix to need be set yet"));
                }

                trees
                    .blockhash_prefix_to_height
                    .insert_if_needed(blockhash_prefix, height, height);

                vecs.height_to_blockhash.push_if_needed(height, blockhash)?;
                vecs.height_to_difficulty.push_if_needed(height, block.header.difficulty_float())?;
                vecs.height_to_timestamp.push_if_needed(height, Timestamp::try_from(block.header.time)?)?;
                vecs.height_to_size.push_if_needed(height, block.total_size())?;
                vecs.height_to_weight.push_if_needed(height, block.weight())?;
                vecs.height_to_first_txindex.push_if_needed(height, txindex_global)?;
                vecs.height_to_first_txinindex
                    .push_if_needed(height, txinindex_global)?;
                vecs.height_to_first_txoutindex
                    .push_if_needed(height, txoutindex_global)?;
                vecs.height_to_first_addressindex
                    .push_if_needed(height, addressindex_global)?;
                vecs.height_to_first_emptyindex
                    .push_if_needed(height, emptyindex_global)?;
                vecs.height_to_first_multisigindex
                    .push_if_needed(height, multisigindex_global)?;
                vecs.height_to_first_opreturnindex
                    .push_if_needed(height, opreturnindex_global)?;
                vecs.height_to_first_pushonlyindex
                    .push_if_needed(height, pushonlyindex_global)?;
                vecs.height_to_first_unknownindex
                    .push_if_needed(height, unknownindex_global)?;
                vecs.height_to_first_p2pk33index.push_if_needed(height, p2pk33index_global)?;
                vecs.height_to_first_p2pk65index.push_if_needed(height, p2pk65index_global)?;
                vecs.height_to_first_p2pkhindex.push_if_needed(height, p2pkhindex_global)?;
                vecs.height_to_first_p2shindex.push_if_needed(height, p2shindex_global)?;
                vecs.height_to_first_p2trindex.push_if_needed(height, p2trindex_global)?;
                vecs.height_to_first_p2wpkhindex.push_if_needed(height, p2wpkhindex_global)?;
                vecs.height_to_first_p2wshindex.push_if_needed(height, p2wshindex_global)?;

                let inputs = block
                    .txdata
                    .iter()
                    .enumerate()
                    .flat_map(|(index, tx)| {
                        tx.input
                            .iter()
                            .enumerate()
                            .map(move |(vin, txin)| (Txindex::from(index), Vin::from(vin), txin, tx))
                    })
                    .collect::<Vec<_>>();

                let outputs = block
                    .txdata
                    .iter()
                    .enumerate()
                    .flat_map(|(index, tx)| {
                        tx.output
                            .iter()
                            .enumerate()
                            .map(move |(vout, txout)| (Txindex::from(index), Vout::from(vout), txout, tx))
                    })
                    .collect::<Vec<_>>();

                let tx_len = block.txdata.len();
                let outputs_len = outputs.len();
                let inputs_len = inputs.len();

                let (
                    txid_prefix_to_txid_and_block_txindex_and_prev_txindex_join_handle,
                    input_source_vec_handle,
                    txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle,
                ) = thread::scope(|scope| {
                    let txid_prefix_to_txid_and_block_txindex_and_prev_txindex_handle =
                        scope.spawn(|| -> color_eyre::Result<_> {
                            block
                                .txdata
                                .par_iter()
                                .enumerate()
                                .map(|(index, tx)| -> color_eyre::Result<_> {
                                    let txid = tx.compute_txid();

                                    let txid_prefix = TxidPrefix::try_from(&txid)?;

                                    let prev_txindex_opt =
                                        if check_collisions && trees.txid_prefix_to_txindex.needs(height) {
                                            // Should only find collisions for two txids (duplicates), see below
                                            trees.txid_prefix_to_txindex.get(&txid_prefix)?.map(|v| *v)
                                        } else {
                                            None
                                        };

                                    Ok((txid_prefix, (tx, txid, Txindex::from(index), prev_txindex_opt)))
                                })
                                .try_fold(BTreeMap::new, |mut map, tuple| {
                                    let (key, value) = tuple?;
                                    map.insert(key, value);
                                    Ok(map)
                                })
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

                    let input_source_vec_handle = scope.spawn(|| {
                        inputs
                            .into_par_iter()
                            .enumerate()
                            .map(|(block_txinindex, (block_txindex, vin, txin, tx))| -> color_eyre::Result<(Txinindex, InputSource)> {
                                let txindex = txindex_global + block_txindex;
                                let txinindex = txinindex_global + Txinindex::from(block_txinindex);

                                // dbg!((txindex, txinindex, vin));

                                let outpoint = txin.previous_output;

                                if tx.is_coinbase() {
                                    return Ok((txinindex, InputSource::SameBlock((tx, txindex, txin, vin))));
                                }

                                let prev_txindex = if let Some(txindex) = trees
                                    .txid_prefix_to_txindex
                                    .get(&TxidPrefix::try_from(&outpoint.txid)?)?
                                    .map(|v| *v)
                                    .and_then(|txindex| {
                                        // Checking if not finding txindex from the future
                                        (txindex < txindex_global).then_some(txindex)
                                    }) {
                                    txindex
                                } else {
                                    // dbg!(txindex_global + block_txindex, txindex, txin, vin);
                                    return Ok((txinindex, InputSource::SameBlock((tx, txindex, txin, vin))));
                                };

                                let vout = Vout::from(outpoint.vout);

                                let txoutindex = *vecs
                                    .txindex_to_first_txoutindex
                                    .get(prev_txindex)?
                                    .context("Expect txoutindex to not be none")
                                    .inspect_err(|_| {
                                        dbg!(outpoint.txid, prev_txindex, vout);
                                    })?
                                    + vout;

                                Ok((txinindex, InputSource::PreviousBlock((
                                    vin,
                                    txindex,
                                    txoutindex,
                                ))))
                            })
                            .try_fold(BTreeMap::new, |mut map, tuple| -> color_eyre::Result<_> {
                                let (key, value) = tuple?;
                                map.insert(key, value);
                                Ok(map)
                            })
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

                    let txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle = scope.spawn(|| {
                        outputs
                            .into_par_iter()
                            .enumerate()
                            .map(
                                #[allow(clippy::type_complexity)]
                                |(block_txoutindex, (block_txindex, vout, txout, tx))| -> color_eyre::Result<(
                                    Txoutindex,
                                    (
                                        &TxOut,
                                        Txindex,
                                        Vout,
                                        Addresstype,
                                        color_eyre::Result<Addressbytes>,
                                        Option<Addressindex>,
                                        &Transaction,
                                    ),
                                )> {
                                    let txindex = txindex_global + block_txindex;
                                    let txoutindex = txoutindex_global + Txoutindex::from(block_txoutindex);

                                    let script = &txout.script_pubkey;

                                    let addresstype = Addresstype::from(script);

                                    let addressbytes_res =
                                        Addressbytes::try_from((script, addresstype)).inspect_err(|_| {
                                            // dbg!(&txout, height, txi, &tx.compute_txid());
                                        });

                                    let addressindex_opt = addressbytes_res.as_ref().ok().and_then(|addressbytes| {
                                        trees
                                            .addresshash_to_addressindex
                                            .get(&AddressHash::from((addressbytes, addresstype)))
                                            .unwrap()
                                            .map(|v| *v)
                                            // Checking if not in the future
                                            .and_then(|addressindex_local| {
                                                (addressindex_local < addressindex_global).then_some(addressindex_local)
                                            })
                                    }); // OK

                                    if let Some(Some(addressindex)) = check_collisions.then_some(addressindex_opt) {
                                        let addressbytes = addressbytes_res.as_ref().unwrap();

                                        let prev_addresstype = *vecs
                                            .addressindex_to_addresstype
                                            .get(addressindex)?
                                            .context("Expect to have address type")?;

                                        let addresstypeindex = *vecs
                                            .addressindex_to_addresstypeindex
                                            .get(addressindex)?
                                            .context("Expect to have address type index")?;
                                        // Good first time
                                        // Wrong after rerun

                                        let prev_addressbytes_opt =
                                            vecs.get_addressbytes(prev_addresstype, addresstypeindex)?;

                                        let prev_addressbytes =
                                            prev_addressbytes_opt.as_ref().context("Expect to have addressbytes")?;

                                        if (vecs.addressindex_to_addresstype.hasnt(addressindex)?
                                            && addresstype != prev_addresstype)
                                            || (trees.addresshash_to_addressindex.needs(height)
                                                && prev_addressbytes != addressbytes)
                                        {
                                            let txid = tx.compute_txid();
                                            dbg!(
                                                _height,
                                                txid,
                                                vout,
                                                block_txindex,
                                                addresstype,
                                                prev_addresstype,
                                                prev_addressbytes,
                                                addressbytes,
                                                addressindex_global,
                                                addressindex,
                                                addresstypeindex,
                                                txout,
                                                AddressHash::from((addressbytes, addresstype)),
                                                AddressHash::from((prev_addressbytes, prev_addresstype))
                                            );
                                            panic!()
                                        }
                                    }

                                    Ok((
                                        txoutindex,
                                        (
                                            txout,
                                            txindex,
                                            vout,
                                            addresstype,
                                            addressbytes_res,
                                            addressindex_opt,
                                            tx,
                                        ),
                                    ))
                                },
                            )
                            .try_fold(BTreeMap::new, |mut map, tuple| -> color_eyre::Result<_> {
                                let (key, value) = tuple?;
                                map.insert(key, value);
                                Ok(map)
                            })
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

                    (
                        txid_prefix_to_txid_and_block_txindex_and_prev_txindex_handle.join(),
                        input_source_vec_handle.join(),
                        txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle.join(),
                    )
                });

                let txid_prefix_to_txid_and_block_txindex_and_prev_txindex =
                    txid_prefix_to_txid_and_block_txindex_and_prev_txindex_join_handle
                        .ok()
                        .context(
                            "Expect txid_prefix_to_txid_and_block_txindex_and_prev_txindex_join_handle to join",
                        )??;

                let input_source_vec = input_source_vec_handle
                    .ok()
                    .context("Export input_source_vec_handle to join")??;

                let txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt =
                    txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle
                        .ok()
                        .context(
                            "Expect txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt_handle to join",
                        )??;

                let mut new_txindexvout_to_txoutindex: BTreeMap<
                    (Txindex, Vout),
                    Txoutindex,
                > = BTreeMap::new();

                let mut already_added_addresshash: BTreeMap<AddressHash, Addressindex> = BTreeMap::new();

                txoutindex_to_txout_addresstype_addressbytes_res_addressindex_opt
                .into_iter()
                .try_for_each(
                    |(
                        txoutindex,
                        (txout, txindex, vout, addresstype, addressbytes_res, addressindex_opt, _tx),
                    )|
                     -> color_eyre::Result<()> {
                        let amount = Amount::from(txout.value);

                        if vout.is_zero() {
                            vecs.txindex_to_first_txoutindex.push_if_needed(txindex, txoutindex)?;
                        }

                        vecs.txoutindex_to_amount.push_if_needed(txoutindex, amount)?;

                        let mut addressindex = addressindex_global;

                        let mut addresshash = None;

                        if let Some(addressindex_local) = addressindex_opt.or_else(|| {
                            addressbytes_res.as_ref().ok().and_then(|addressbytes| {
                                // Check if address was first seen before in this iterator
                                // Example: https://mempool.space/address/046a0765b5865641ce08dd39690aade26dfbf5511430ca428a3089261361cef170e3929a68aee3d8d4848b0c5111b0a37b82b86ad559fd2a745b44d8e8d9dfdc0c
                                addresshash.replace(AddressHash::from((addressbytes, addresstype)));
                                already_added_addresshash
                                    .get(addresshash.as_ref().unwrap())
                                    .cloned()
                            })
                        }) {
                            addressindex = addressindex_local;
                        } else {
                            addressindex_global.increment();

                            let addresstypeindex = match addresstype {
                                Addresstype::Empty => emptyindex_global.clone_then_increment(),
                                Addresstype::Multisig => multisigindex_global.clone_then_increment(),
                                Addresstype::OpReturn => opreturnindex_global.clone_then_increment(),
                                Addresstype::PushOnly => pushonlyindex_global.clone_then_increment(),
                                Addresstype::Unknown => unknownindex_global.clone_then_increment(),
                                Addresstype::P2PK65 => p2pk65index_global.clone_then_increment(),
                                Addresstype::P2PK33 => p2pk33index_global.clone_then_increment(),
                                Addresstype::P2PKH => p2pkhindex_global.clone_then_increment(),
                                Addresstype::P2SH => p2shindex_global.clone_then_increment(),
                                Addresstype::P2WPKH => p2wpkhindex_global.clone_then_increment(),
                                Addresstype::P2WSH => p2wshindex_global.clone_then_increment(),
                                Addresstype::P2TR => p2trindex_global.clone_then_increment(),
                            };

                            vecs.addressindex_to_addresstype
                                .push_if_needed(addressindex, addresstype)?;

                            vecs.addressindex_to_addresstypeindex
                                .push_if_needed(addressindex, addresstypeindex)?;

                            vecs.addressindex_to_height
                                .push_if_needed(addressindex, height)?;

                            if let Ok(addressbytes) = addressbytes_res {
                                let addresshash = addresshash.unwrap();

                                already_added_addresshash
                                    .insert(addresshash, addressindex);

                                trees.addresshash_to_addressindex.insert_if_needed(
                                    addresshash,
                                    addressindex,
                                    height,
                                );

                                vecs.push_addressbytes_if_needed(addresstypeindex, addressbytes)?;
                            }
                        }

                        new_txindexvout_to_txoutindex
                            .insert((txindex, vout), txoutindex);

                        vecs.txoutindex_to_addressindex
                            .push_if_needed(txoutindex, addressindex)?;

                        Ok(())
                    },
                )?;

                drop(already_added_addresshash);

                input_source_vec
                    .into_iter()
                    .map(
                        #[allow(clippy::type_complexity)]
                        |(txinindex, input_source)| -> color_eyre::Result<(
                            Txinindex, Vin, Txindex, Txoutindex
                        )> {
                            match input_source {
                                InputSource::PreviousBlock((vin, txindex, txoutindex)) => Ok((txinindex, vin, txindex, txoutindex)),
                                InputSource::SameBlock((tx, txindex, txin, vin)) => {
                                    if tx.is_coinbase() {
                                        return Ok((txinindex, vin, txindex, Txoutindex::COINBASE));
                                    }

                                    let outpoint = txin.previous_output;
                                    let txid = outpoint.txid;
                                    let vout = Vout::from(outpoint.vout);

                                    let block_txindex = txid_prefix_to_txid_and_block_txindex_and_prev_txindex
                                        .get(&TxidPrefix::try_from(&txid)?)
                                        .context("txid should be in same block").inspect_err(|_| {
                                            dbg!(&txid_prefix_to_txid_and_block_txindex_and_prev_txindex);
                                            // panic!();
                                        })?
                                        .2;
                                    let prev_txindex = txindex_global + block_txindex;

                                    let prev_txoutindex = new_txindexvout_to_txoutindex
                                        .remove(&(prev_txindex, vout))
                                        .context("should have found addressindex from same block")
                                        .inspect_err(|_| {
                                            dbg!(&new_txindexvout_to_txoutindex, txin, prev_txindex, vout, txid);
                                        })?;

                                    Ok((txinindex, vin, txindex, prev_txoutindex))
                                }
                            }
                        },
                    )
                    .try_for_each(|res| -> color_eyre::Result<()> {
                        let (txinindex, vin, txindex, txoutindex) = res?;

                        if vin.is_zero() {
                            vecs.txindex_to_first_txinindex.push_if_needed(txindex, txinindex)?;
                        }

                        vecs.txinindex_to_txoutindex.push_if_needed(txinindex, txoutindex)?;

                        Ok(())
                    })?;

                drop(new_txindexvout_to_txoutindex);

                let mut txindex_to_tx_and_txid: BTreeMap<Txindex, (&Transaction, Txid)> = BTreeMap::default();

                txid_prefix_to_txid_and_block_txindex_and_prev_txindex
                    .into_iter()
                    .try_for_each(
                        |(txid_prefix, (tx, txid, index, prev_txindex_opt))| -> color_eyre::Result<()> {
                            let txindex = txindex_global + index;

                            txindex_to_tx_and_txid.insert(txindex, (tx, txid));

                            match prev_txindex_opt {
                                None => {
                                    trees
                                        .txid_prefix_to_txindex
                                        .insert_if_needed(txid_prefix, txindex, height);
                                }
                                Some(prev_txindex) => {
                                    // In case if we start at an already parsed height
                                    if txindex == prev_txindex {
                                        return Ok(());
                                    }

                                    let len = vecs.txindex_to_txid.len();
                                    // Ok if `get` is not par as should happen only twice
                                    let prev_txid = vecs
                                        .txindex_to_txid
                                        .get(prev_txindex)?
                                        .context("To have txid for txindex")
                                        .inspect_err(|_| {
                                            dbg!(txindex, txid, len);
                                        })?;

                                    // #[allow(clippy::redundant_locals)]
                                    // let prev_txid = prev_txid;
                                    let prev_txid = prev_txid.as_ref();

                                    // If another Txid needs to be added to the list
                                    // We need to check that it's also a coinbase tx otherwise par_iter inputs needs to be updated
                                    let only_known_dup_txids = [
                                        Txid::from_str(
                                            "d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599",
                                        )?,
                                        Txid::from_str(
                                            "e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468",
                                        )?,
                                    ];

                                    let is_dup = only_known_dup_txids.contains(prev_txid);

                                    if !is_dup {
                                        let prev_height =
                                            vecs.txindex_to_height.get(prev_txindex)?.expect("To have height");
                                        dbg!(height, txid, txindex, prev_height, prev_txid, prev_txindex);
                                        return Err(eyre!("Expect none"));
                                    }
                                }
                            }

                            Ok(())
                        },
                    )?;

                txindex_to_tx_and_txid
                    .into_iter()
                    .try_for_each(|(txindex, (tx, txid))| -> color_eyre::Result<()> {
                        vecs.txindex_to_txversion.push_if_needed(txindex, tx.version)?;
                        vecs.txindex_to_txid.push_if_needed(txindex, txid)?;
                        vecs.txindex_to_height.push_if_needed(txindex, height)?;
                        vecs.txindex_to_locktime.push_if_needed(txindex, tx.lock_time)?;
                        Ok(())
                    })?;

                txindex_global += Txindex::from(tx_len);
                txinindex_global += Txinindex::from(inputs_len);
                txoutindex_global += Txoutindex::from(outputs_len);

                let should_snapshot = _height != 0 && _height % SNAPSHOT_BLOCK_RANGE == 0 && !exit.active();
                if should_snapshot {
                    export(trees, vecs, height)?;
                }

                Ok(())
            })?;

        export(trees, vecs, height)?;

        sleep(Duration::from_millis(100));

        Ok(())
    }
}

#[derive(Debug)]
enum InputSource<'a> {
    PreviousBlock((Vin, Txindex, Txoutindex)),
    SameBlock((&'a Transaction, Txindex, &'a TxIn, Vin)),
}

#[allow(unused)]
fn pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
