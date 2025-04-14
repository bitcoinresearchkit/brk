#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
    thread,
};

use brk_core::{
    AddressHash, Addressbytes, Addressindex, Addresstype, BlockHash, BlockHashPrefix, Height, Sats,
    Timestamp, Txid, TxidPrefix, Txindex, Txinindex, Txoutindex, Vin, Vout, setrlimit,
};
pub use brk_parser::*;

use bitcoin::{Transaction, TxIn, TxOut};
use brk_exit::Exit;
use brk_vec::Compressed;
use color_eyre::eyre::{ContextCompat, eyre};
use log::info;
use rayon::prelude::*;
mod indexes;
mod stores;
mod vecs;

pub use indexes::*;
pub use stores::*;
pub use vecs::*;

const SNAPSHOT_BLOCK_RANGE: usize = 1000;
const COLLISIONS_CHECKED_UP_TO: u32 = 890_000;

#[derive(Clone)]
pub struct Indexer {
    path: PathBuf,
    vecs: Option<Vecs>,
    stores: Option<Stores>,
    check_collisions: bool,
    compressed: Compressed,
}

impl Indexer {
    pub fn new(
        indexes_dir: PathBuf,
        compressed: bool,
        check_collisions: bool,
    ) -> color_eyre::Result<Self> {
        setrlimit()?;
        Ok(Self {
            path: indexes_dir,
            vecs: None,
            stores: None,
            compressed: Compressed::from(compressed),
            check_collisions,
        })
    }

    pub fn import_vecs(&mut self) -> color_eyre::Result<()> {
        self.vecs = Some(Vecs::import(&self.path.join("vecs"), self.compressed)?);
        Ok(())
    }

    /// Do NOT import multiple times are things will break !!!
    /// Clone struct instead
    pub fn import_stores(&mut self) -> color_eyre::Result<()> {
        self.stores = Some(Stores::import(&self.path.join("stores"))?);
        Ok(())
    }

    pub fn index(
        &mut self,
        parser: &Parser,
        rpc: &'static rpc::Client,
        exit: &Exit,
    ) -> color_eyre::Result<Indexes> {
        let starting_indexes = Indexes::try_from((
            self.vecs.as_mut().unwrap(),
            self.stores.as_ref().unwrap(),
            rpc,
        ))
        .unwrap_or_else(|_report| {
            let indexes = Indexes::default();
            indexes.push_if_needed(self.vecs.as_mut().unwrap()).unwrap();
            indexes
        });

        exit.block();
        self.stores
            .as_mut()
            .unwrap()
            .rollback_if_needed(self.vecs.as_mut().unwrap(), &starting_indexes)?;
        self.vecs
            .as_mut()
            .unwrap()
            .rollback_if_needed(&starting_indexes)?;
        exit.release();

        let vecs = self.vecs.as_mut().unwrap();
        let stores = self.stores.as_mut().unwrap();

        // Cloned because we want to return starting indexes for the computer
        let mut idxs = starting_indexes.clone();

        let start = Some(idxs.height);
        let end = None;

        if starting_indexes.height > Height::try_from(rpc)?
            || end.is_some_and(|end| starting_indexes.height > end)
        {
            return Ok(starting_indexes);
        }

        info!("Started indexing...");

        let export_if_needed = |stores: &mut Stores,
                                vecs: &mut Vecs,
                                height: Height,
                                rem: bool,
                                exit: &Exit|
         -> color_eyre::Result<()> {
            if height == 0 || (height % SNAPSHOT_BLOCK_RANGE != 0) != rem || exit.triggered() {
                return Ok(());
            }

            info!("Exporting...");
            exit.block();
            stores.commit(height)?;
            vecs.flush(height)?;
            exit.release();
            Ok(())
        };

        parser.parse(start, end).iter().try_for_each(
            |(height, block, blockhash)| -> color_eyre::Result<()> {
                info!("Indexing block {height}...");

                idxs.height = height;

                // Used to check rapidhash collisions
                let check_collisions = self.check_collisions && height > Height::new(COLLISIONS_CHECKED_UP_TO);

                let blockhash = BlockHash::from(blockhash);
                let blockhash_prefix = BlockHashPrefix::from(&blockhash);

                if stores
                    .blockhash_prefix_to_height
                    .get(&blockhash_prefix)?
                    .is_some_and(|prev_height| *prev_height != height)
                {
                    dbg!(blockhash);
                    return Err(eyre!("Collision, expect prefix to need be set yet"));
                }

                idxs.push_if_needed(vecs)?;

                stores
                    .blockhash_prefix_to_height
                    .insert_if_needed(blockhash_prefix, height, height);

                vecs.height_to_blockhash.push_if_needed(height, blockhash)?;
                vecs.height_to_difficulty
                    .push_if_needed(height, block.header.difficulty_float())?;
                vecs.height_to_timestamp
                    .push_if_needed(height, Timestamp::from(block.header.time))?;
                vecs.height_to_total_size.push_if_needed(height, block.total_size().into())?;
                vecs.height_to_weight.push_if_needed(height, block.weight().into())?;

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
                                    let txid = Txid::from(tx.compute_txid());

                                    let txid_prefix = TxidPrefix::from(&txid);

                                    let prev_txindex_opt =
                                        if check_collisions && stores.txid_prefix_to_txindex.needs(height) {
                                            // Should only find collisions for two txids (duplicates), see below
                                            stores.txid_prefix_to_txindex.get(&txid_prefix)?.map(|v| *v)
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
                                let txindex = idxs.txindex + block_txindex;
                                let txinindex = idxs.txinindex + Txinindex::from(block_txinindex);

                                let outpoint = txin.previous_output;
                                let txid = Txid::from(outpoint.txid);

                                if tx.is_coinbase() {
                                    return Ok((txinindex, InputSource::SameBlock((tx, txindex, txin, vin))));
                                }

                                let prev_txindex = if let Some(txindex) = stores
                                    .txid_prefix_to_txindex
                                    .get(&TxidPrefix::from(&txid))?
                                    .map(|v| *v)
                                    .and_then(|txindex| {
                                        // Checking if not finding txindex from the future
                                        (txindex < idxs.txindex).then_some(txindex)
                                    }) {
                                    txindex
                                } else {
                                    // dbg!(indexes.txindex + block_txindex, txindex, txin, vin);
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
                                        brk_core::Result<Addressbytes>,
                                        Option<Addressindex>,
                                        &Transaction,
                                    ),
                                )> {
                                    let txindex = idxs.txindex + block_txindex;
                                    let txoutindex = idxs.txoutindex + Txoutindex::from(block_txoutindex);

                                    let script = &txout.script_pubkey;

                                    let addresstype = Addresstype::from(script);

                                    let addressbytes_res =
                                        Addressbytes::try_from((script, addresstype)).inspect_err(|_| {
                                            // dbg!(&txout, height, txi, &tx.compute_txid());
                                        });

                                    let addressindex_opt = addressbytes_res.as_ref().ok().and_then(|addressbytes| {
                                        stores
                                            .addresshash_to_addressindex
                                            .get(&AddressHash::from((addressbytes, addresstype)))
                                            .unwrap()
                                            .map(|v| *v)
                                            // Checking if not in the future
                                            .and_then(|addressindex_local| {
                                                (addressindex_local < idxs.addressindex).then_some(addressindex_local)
                                            })
                                    });

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

                                        let prev_addressbytes_opt =
                                            vecs.get_addressbytes(prev_addresstype, addresstypeindex)?;

                                        let prev_addressbytes =
                                            prev_addressbytes_opt.as_ref().context("Expect to have addressbytes")?;

                                        if (vecs.addressindex_to_addresstype.hasnt(addressindex)?
                                            && addresstype != prev_addresstype)
                                            || (stores.addresshash_to_addressindex.needs(height)
                                                && prev_addressbytes != addressbytes)
                                        {
                                            let txid = tx.compute_txid();
                                            dbg!(
                                                height,
                                                txid,
                                                vout,
                                                block_txindex,
                                                addresstype,
                                                prev_addresstype,
                                                prev_addressbytes,
                                                addressbytes,
                                                idxs.addressindex,
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
                        let sats = Sats::from(txout.value);

                        if vout.is_zero() {
                            vecs.txindex_to_first_txoutindex.push_if_needed(txindex, txoutindex)?;
                        }

                        vecs.txoutindex_to_value.push_if_needed(txoutindex, sats)?;

                        vecs.txoutindex_to_height
                            .push_if_needed(txoutindex, height)?;

                        let mut addressindex = idxs.addressindex;

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
                            idxs.addressindex.increment();

                            let addresstypeindex = match addresstype {
                                Addresstype::Empty => {
                                    vecs.emptyindex_to_height
                                        .push_if_needed(idxs.emptyindex, height)?;
                                    idxs.emptyindex.copy_then_increment()
                                },
                                Addresstype::Multisig => {
                                    vecs.multisigindex_to_height.push_if_needed(idxs.multisigindex, height)?;
                                    idxs.multisigindex.copy_then_increment()
                                },
                                Addresstype::OpReturn => {
                                    vecs.opreturnindex_to_height.push_if_needed(idxs.opreturnindex, height)?;
                                    idxs.opreturnindex.copy_then_increment()
                                },
                                Addresstype::PushOnly => {
                                    vecs.pushonlyindex_to_height.push_if_needed(idxs.pushonlyindex, height)?;
                                    idxs.pushonlyindex.copy_then_increment()
                                },
                                Addresstype::Unknown => {
                                    vecs.unknownindex_to_height.push_if_needed(idxs.unknownindex, height)?;
                                    idxs.unknownindex.copy_then_increment()
                                },
                                Addresstype::P2PK65 => {
                                    vecs.p2pk65index_to_height.push_if_needed(idxs.p2pk65index, height)?;
                                    idxs.p2pk65index.copy_then_increment()
                                },
                                Addresstype::P2PK33 => {
                                    vecs.p2pk33index_to_height.push_if_needed(idxs.p2pk33index, height)?;
                                    idxs.p2pk33index.copy_then_increment()
                                },
                                Addresstype::P2PKH => {
                                    vecs.p2pkhindex_to_height.push_if_needed(idxs.p2pkhindex, height)?;
                                    idxs.p2pkhindex.copy_then_increment()
                                },
                                Addresstype::P2SH => {
                                    vecs.p2shindex_to_height.push_if_needed(idxs.p2shindex, height)?;
                                    idxs.p2shindex.copy_then_increment()
                                },
                                Addresstype::P2WPKH => {
                                    vecs.p2wpkhindex_to_height.push_if_needed(idxs.p2wpkhindex, height)?;
                                    idxs.p2wpkhindex.copy_then_increment()
                                },
                                Addresstype::P2WSH => {
                                    vecs.p2wshindex_to_height.push_if_needed(idxs.p2wshindex, height)?;
                                    idxs.p2wshindex.copy_then_increment()
                                },
                                Addresstype::P2TR => {
                                    vecs.p2trindex_to_height.push_if_needed(idxs.p2trindex, height)?;
                                    idxs.p2trindex.copy_then_increment()
                                },
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

                                stores.addresshash_to_addressindex.insert_if_needed(
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
                                    let txid = Txid::from(outpoint.txid);
                                    let vout = Vout::from(outpoint.vout);

                                    let block_txindex = txid_prefix_to_txid_and_block_txindex_and_prev_txindex
                                        .get(&TxidPrefix::from(&txid))
                                        .context("txid should be in same block").inspect_err(|_| {
                                            dbg!(&txid_prefix_to_txid_and_block_txindex_and_prev_txindex);
                                            // panic!();
                                        })?
                                        .2;
                                    let prev_txindex = idxs.txindex + block_txindex;

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

                        vecs.txinindex_to_height
                            .push_if_needed(txinindex, height)?;


                        Ok(())
                    })?;

                drop(new_txindexvout_to_txoutindex);

                let mut txindex_to_tx_and_txid: BTreeMap<Txindex, (&Transaction, Txid)> = BTreeMap::default();

                txid_prefix_to_txid_and_block_txindex_and_prev_txindex
                    .into_iter()
                    .try_for_each(
                        |(txid_prefix, (tx, txid, index, prev_txindex_opt))| -> color_eyre::Result<()> {
                            let txindex = idxs.txindex + index;

                            txindex_to_tx_and_txid.insert(txindex, (tx, txid));

                            match prev_txindex_opt {
                                None => {
                                    stores
                                        .txid_prefix_to_txindex
                                        .insert_if_needed(txid_prefix, txindex, height);
                                }
                                Some(prev_txindex) => {
                                    // In case if we start at an already parsed height
                                    if txindex == prev_txindex {
                                        return Ok(());
                                    }

                                    if !check_collisions {
                                        return Ok(())
                                    }

                                    let len = vecs.txindex_to_txid.len();
                                    // Ok if `get` is not par as should happen only twice
                                    let prev_txid = vecs
                                        .txindex_to_txid
                                        .get(prev_txindex)?
                                        .context("To have txid for txindex")
                                        .inspect_err(|_| {
                                            dbg!(txindex, len);
                                        })?;

                                    let prev_txid = prev_txid.as_ref();

                                    // If another Txid needs to be added to the list
                                    // We need to check that it's also a coinbase tx otherwise par_iter inputs needs to be updated
                                    let only_known_dup_txids = [
                                        bitcoin::Txid::from_str(
                                            "d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599",
                                        )?
                                        .into(),
                                        bitcoin::Txid::from_str(
                                            "e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468",
                                        )?
                                        .into(),
                                    ];

                                    let is_dup = only_known_dup_txids.contains(prev_txid);

                                    if !is_dup {
                                        let prev_height =
                                            vecs.txindex_to_height.get(prev_txindex)?.expect("To have height");
                                        dbg!(height, txindex, prev_height, prev_txid, prev_txindex);
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
                        vecs.txindex_to_txversion.push_if_needed(txindex, tx.version.into())?;
                        vecs.txindex_to_txid.push_if_needed(txindex, txid)?;
                        vecs.txindex_to_height.push_if_needed(txindex, height)?;
                        vecs.txindex_to_locktime.push_if_needed(txindex, tx.lock_time.into())?;
                        vecs.txindex_to_base_size.push_if_needed(txindex, tx.base_size())?;
                        vecs.txindex_to_total_size.push_if_needed(txindex, tx.total_size())?;
                        vecs.txindex_to_is_explicitly_rbf.push_if_needed(txindex, tx.is_explicitly_rbf())?;
                        Ok(())
                    })?;

                idxs.txindex += Txindex::from(tx_len);
                idxs.txinindex += Txinindex::from(inputs_len);
                idxs.txoutindex += Txoutindex::from(outputs_len);

                export_if_needed(stores, vecs, height, false, exit)?;

                Ok(())
            },
        )?;

        export_if_needed(stores, vecs, idxs.height, true, exit)?;

        stores.rotate_memtables();

        Ok(starting_indexes)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn vecs(&self) -> &Vecs {
        self.vecs.as_ref().unwrap()
    }

    pub fn mut_vecs(&mut self) -> &mut Vecs {
        self.vecs.as_mut().unwrap()
    }

    pub fn stores(&self) -> &Stores {
        self.stores.as_ref().unwrap()
    }

    pub fn mut_stores(&mut self) -> &mut Stores {
        self.stores.as_mut().unwrap()
    }
}

#[derive(Debug)]
enum InputSource<'a> {
    PreviousBlock((Vin, Txindex, Txoutindex)),
    SameBlock((&'a Transaction, Txindex, &'a TxIn, Vin)),
}
