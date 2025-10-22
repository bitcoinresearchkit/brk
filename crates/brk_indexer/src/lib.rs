#![doc = include_str!("../README.md")]

use std::{path::Path, str::FromStr, thread, time::Instant};

use bitcoin::{TxIn, TxOut};
use brk_error::{Error, Result};
use brk_store::AnyStore;
use brk_structs::{
    AddressBytes, AddressBytesHash, BlockHashPrefix, Height, OutPoint, OutputType, Sats,
    StoredBool, Timestamp, TxInIndex, TxIndex, TxOutIndex, Txid, TxidPrefix, TypeIndex,
    TypeIndexAndOutPoint, TypeIndexAndTxIndex, Unit, Version, Vin, Vout,
};
use log::{error, info};
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use vecdb::{AnyVec, Exit, GenericStoredVec, Reader, VecIterator};
mod indexes;
mod stores_v2;
// mod stores_v3;
mod vecs;

pub use indexes::*;
pub use stores_v2::*;
// pub use stores_v3::*;
pub use vecs::*;

// One version for all data sources
// Increment on **change _OR_ addition**
const VERSION: Version = Version::new(22);
const SNAPSHOT_BLOCK_RANGE: usize = 1_000;
const COLLISIONS_CHECKED_UP_TO: Height = Height::new(0);

#[derive(Clone)]
pub struct Indexer {
    pub vecs: Vecs,
    pub stores: Stores,
}

impl Indexer {
    pub fn forced_import(outputs_dir: &Path) -> Result<Self> {
        info!("Importing indexer...");

        let path = outputs_dir.join("indexed");

        let (vecs, stores) = thread::scope(|s| -> Result<_> {
            let vecs = s.spawn(|| -> Result<_> {
                let vecs = Vecs::forced_import(&path, VERSION)?;
                info!("Imported vecs");
                Ok(vecs)
            });

            let stores = Stores::forced_import(&path, VERSION)?;
            info!("Imported stores");

            Ok((vecs.join().unwrap()?, stores))
        })?;

        Ok(Self { vecs, stores })
    }

    pub fn index(
        &mut self,
        reader: &brk_reader::Reader,
        rpc: &'static bitcoincore_rpc::Client,
        exit: &Exit,
        check_collisions: bool,
    ) -> Result<Indexes> {
        let starting_indexes = Indexes::try_from((&mut self.vecs, &self.stores, rpc))
            .unwrap_or_else(|_report| Indexes::default());

        let lock = exit.lock();
        self.stores
            .rollback_if_needed(&mut self.vecs, &starting_indexes)?;
        self.vecs.rollback_if_needed(&starting_indexes)?;
        drop(lock);

        let vecs = &mut self.vecs;
        let stores = &mut self.stores;

        // Cloned because we want to return starting indexes for the computer
        let mut idxs = starting_indexes.clone();

        let start = Some(idxs.height);
        let end = None;

        if starting_indexes.height > Height::try_from(rpc)?
            || end.is_some_and(|end| starting_indexes.height > end)
        {
            info!("Up to date, nothing to index.");
            return Ok(starting_indexes);
        }

        info!("Started indexing...");

        let should_export = |height: Height, rem: bool| -> bool {
            height != 0 && (height % SNAPSHOT_BLOCK_RANGE == 0) != rem
        };

        let export =
            |stores: &mut Stores, vecs: &mut Vecs, height: Height, exit: &Exit| -> Result<()> {
                info!("Exporting...");
                // std::process::exit(0);
                let _lock = exit.lock();
                let i = Instant::now();
                stores.commit(height).unwrap();
                info!("Commited stores in {}s", i.elapsed().as_secs());
                let i = Instant::now();
                vecs.flush(height)?;
                info!("Flushed vecs in {}s", i.elapsed().as_secs());
                let i = Instant::now();
                info!("Flushed db in {}s", i.elapsed().as_secs());
                Ok(())
            };

        let mut readers = Readers::new(vecs);
        let mut already_added_addressbyteshash: FxHashMap<AddressBytesHash, TypeIndex> =
            FxHashMap::default();
        let mut same_block_spent_outpoints: FxHashSet<OutPoint> = FxHashSet::default();
        let mut same_block_output_info: FxHashMap<OutPoint, (OutputType, TypeIndex)> =
            FxHashMap::default();

        // TODO: CHECK PREV HASH

        for block in reader.read(start, end).iter() {
            // let i_tot = Instant::now();
            already_added_addressbyteshash.clear();
            same_block_spent_outpoints.clear();
            same_block_output_info.clear();

            let height = block.height();
            let blockhash = block.hash();

            info!("Indexing block {height}...");

            idxs.height = height;

            // Used to check rapidhash collisions
            let check_collisions = check_collisions && height > COLLISIONS_CHECKED_UP_TO;

            let blockhash_prefix = BlockHashPrefix::from(blockhash);

            if stores
                .blockhashprefix_to_height
                .get(&blockhash_prefix)?
                .is_some_and(|prev_height| *prev_height != height)
            {
                error!("BlockHash: {blockhash}");
                return Err(Error::Str("Collision, expect prefix to need be set yet"));
            }

            idxs.push_if_needed(vecs)?;

            stores
                .blockhashprefix_to_height
                .insert_if_needed(blockhash_prefix, height, height);

            stores.height_to_coinbase_tag.insert_if_needed(
                height,
                block.coinbase_tag().into(),
                height,
            );

            vecs.height_to_blockhash
                .push_if_needed(height, blockhash.clone())?;
            vecs.height_to_difficulty
                .push_if_needed(height, block.header.difficulty_float().into())?;
            vecs.height_to_timestamp
                .push_if_needed(height, Timestamp::from(block.header.time))?;
            vecs.height_to_total_size
                .push_if_needed(height, block.total_size().into())?;
            vecs.height_to_weight
                .push_if_needed(height, block.weight().into())?;

            // let i = Instant::now();
            let txs = block
                .txdata
                .par_iter()
                .enumerate()
                .map(|(index, tx)| {
                    // par_iter due to compute_txid being costly
                    let txid = Txid::from(tx.compute_txid());

                    let txid_prefix = TxidPrefix::from(&txid);

                    let prev_txindex_opt =
                        if check_collisions && stores.txidprefix_to_txindex.needs(height) {
                            // Should only find collisions for two txids (duplicates), see below
                            stores.txidprefix_to_txindex.get(&txid_prefix)?.map(|v| *v)
                        } else {
                            None
                        };

                    Ok((
                        idxs.txindex + TxIndex::from(index),
                        tx,
                        txid,
                        txid_prefix,
                        prev_txindex_opt,
                    ))
                })
                .collect::<Result<Vec<_>>>()?;
            // println!("txs = : {:?}", i.elapsed());

            // let i = Instant::now();
            let txid_prefix_to_txindex = txs
                .iter()
                .map(|(txindex, _, _, prefix, _)| (*prefix, txindex))
                .collect::<FxHashMap<_, _>>();
            let txins = block
                .txdata
                .iter()
                .enumerate()
                .flat_map(|(index, tx)| tx
                    .input
                    .iter()
                    .enumerate()
                    .map(move |(vin, txin)| (TxIndex::from(index), Vin::from(vin), txin, tx))
                )
                .collect::<Vec<_>>()
                .into_par_iter()
                .enumerate()
                .map(|(block_txinindex, (block_txindex, vin, txin, tx))| -> Result<(TxInIndex, InputSource)> {
                    let txindex = idxs.txindex + block_txindex;
                    let txinindex = idxs.txinindex + TxInIndex::from(block_txinindex);

                    if tx.is_coinbase() {
                        return Ok((txinindex, InputSource::SameBlock((txindex, txin, vin, OutPoint::COINBASE))));
                    }

                    let outpoint = txin.previous_output;
                    let txid = Txid::from(outpoint.txid);
                    let txid_prefix = TxidPrefix::from(&txid);

                    let prev_txindex = if let Some(txindex) = stores
                        .txidprefix_to_txindex
                        .get(&txid_prefix)?
                        .map(|v| *v)
                        .and_then(|txindex| {
                            // Checking if not finding txindex from the future
                            (txindex < idxs.txindex).then_some(txindex)
                        }) {
                        txindex
                    } else {
                        let vout = Vout::from(outpoint.vout);

                        let prev_txindex = **txid_prefix_to_txindex
                            .get(&txid_prefix)
                            .ok_or(Error::Str("txid should be in same block")).inspect_err(|_| {
                                dbg!(&txs);
                                // panic!();
                            })?;

                        let outpoint = OutPoint::new(prev_txindex, vout);

                        return Ok((txinindex, InputSource::SameBlock((txindex, txin, vin, outpoint))));
                    };

                    let vout = Vout::from(outpoint.vout);

                    let txoutindex = vecs.txindex_to_first_txoutindex.get_pushed_or_read(prev_txindex, &readers.txindex_to_first_txoutindex)?
                        .ok_or(Error::Str("Expect txoutindex to not be none"))
                        .inspect_err(|_| {
                            dbg!(outpoint.txid, prev_txindex, vout);
                        })?.into_owned()
                        + vout;

                    let outpoint = OutPoint::new(prev_txindex, vout);

                    let outputtype = vecs.txoutindex_to_outputtype.get_pushed_or_read(txoutindex, &readers.txoutindex_to_outputtype)?
                        .ok_or(Error::Str("Expect outputtype to not be none"))?.into_owned();

                    let mut tuple = (
                        vin,
                        txindex,
                        outpoint,
                        None
                    );

                    // Rare but happens
                    // https://mempool.space/tx/8ebe1df6ebf008f7ec42ccd022478c9afaec3ca0444322243b745aa2e317c272#flow=&vin=89
                    if outputtype.is_address() {
                        let typeindex = vecs
                            .txoutindex_to_typeindex
                            .get_pushed_or_read(txoutindex, &readers.txoutindex_to_typeindex)?
                            .ok_or(Error::Str("Expect typeindex to not be none"))?.into_owned();
                        tuple.3 = Some((outputtype, typeindex));
                    }

                    Ok((txinindex, InputSource::PreviousBlock(tuple)))
                })
                .collect::<Result<Vec<_>>>()?;
            drop(txid_prefix_to_txindex);
            // println!("txinindex_and_txindata = : {:?}", i.elapsed());

            // let i = Instant::now();
            same_block_spent_outpoints.extend(txins.iter().filter_map(|(_, input_source)| {
                let InputSource::SameBlock((_, _, _, outpoint)) = input_source else {
                    return None;
                };
                if !outpoint.is_coinbase() {
                    Some(*outpoint)
                } else {
                    None
                }
            }));
            // println!("same_block_spent_outpoints = : {:?}", i.elapsed());

            // let i = Instant::now();
            let txouts = block
                .txdata
                .iter()
                .enumerate()
                .flat_map(|(index, tx)| {
                    tx.output.iter().enumerate().map(move |(vout, txout)| {
                        (TxIndex::from(index), Vout::from(vout), txout, tx)
                    })
                })
                .collect::<Vec<_>>()
                .into_par_iter()
                .enumerate()
                .map(
                    #[allow(clippy::type_complexity)]
                    |(block_txoutindex, (block_txindex, vout, txout, tx))| -> Result<(
                        TxOutIndex,
                        &TxOut,
                        TxIndex,
                        Vout,
                        OutputType,
                        Option<(AddressBytes, AddressBytesHash)>,
                        Option<TypeIndex>,
                    )> {
                        let txindex = idxs.txindex + block_txindex;
                        let txoutindex = idxs.txoutindex + TxOutIndex::from(block_txoutindex);

                        let script = &txout.script_pubkey;

                        let outputtype = OutputType::from(script);

                        let mut tuple = (txoutindex, txout, txindex, vout, outputtype, None, None);

                        if outputtype.is_not_address() {
                            return Ok(tuple);
                        }

                        let address_bytes = AddressBytes::try_from((script, outputtype)).unwrap();

                        let address_hash = AddressBytesHash::from(&address_bytes);

                        let typeindex_opt = stores
                            .addressbyteshash_to_typeindex
                            .get(&address_hash)
                            .unwrap()
                            .map(|v| *v)
                            // Checking if not in the future (in case we started before the last processed block)
                            .and_then(|typeindex_local| {
                                (typeindex_local < idxs.to_typeindex(outputtype))
                                    .then_some(typeindex_local)
                            });

                        tuple.5 = Some((address_bytes, address_hash));
                        tuple.6 = typeindex_opt;

                        if check_collisions && let Some(typeindex) = typeindex_opt {
                            // unreachable!();

                            let prev_addressbytes_opt = match outputtype {
                                OutputType::P2PK65 => vecs
                                    .p2pk65addressindex_to_p2pk65bytes
                                    .get_pushed_or_read(
                                        typeindex.into(),
                                        &readers.p2pk65addressindex_to_p2pk65bytes,
                                    )?
                                    .map(|v| AddressBytes::from(v.into_owned())),
                                OutputType::P2PK33 => vecs
                                    .p2pk33addressindex_to_p2pk33bytes
                                    .get_pushed_or_read(
                                        typeindex.into(),
                                        &readers.p2pk33addressindex_to_p2pk33bytes,
                                    )?
                                    .map(|v| AddressBytes::from(v.into_owned())),
                                OutputType::P2PKH => vecs
                                    .p2pkhaddressindex_to_p2pkhbytes
                                    .get_pushed_or_read(
                                        typeindex.into(),
                                        &readers.p2pkhaddressindex_to_p2pkhbytes,
                                    )?
                                    .map(|v| AddressBytes::from(v.into_owned())),
                                OutputType::P2SH => vecs
                                    .p2shaddressindex_to_p2shbytes
                                    .get_pushed_or_read(
                                        typeindex.into(),
                                        &readers.p2shaddressindex_to_p2shbytes,
                                    )?
                                    .map(|v| AddressBytes::from(v.into_owned())),
                                OutputType::P2WPKH => vecs
                                    .p2wpkhaddressindex_to_p2wpkhbytes
                                    .get_pushed_or_read(
                                        typeindex.into(),
                                        &readers.p2wpkhaddressindex_to_p2wpkhbytes,
                                    )?
                                    .map(|v| AddressBytes::from(v.into_owned())),
                                OutputType::P2WSH => vecs
                                    .p2wshaddressindex_to_p2wshbytes
                                    .get_pushed_or_read(
                                        typeindex.into(),
                                        &readers.p2wshaddressindex_to_p2wshbytes,
                                    )?
                                    .map(|v| AddressBytes::from(v.into_owned())),
                                OutputType::P2TR => vecs
                                    .p2traddressindex_to_p2trbytes
                                    .get_pushed_or_read(
                                        typeindex.into(),
                                        &readers.p2traddressindex_to_p2trbytes,
                                    )?
                                    .map(|v| AddressBytes::from(v.into_owned())),
                                OutputType::P2A => vecs
                                    .p2aaddressindex_to_p2abytes
                                    .get_pushed_or_read(
                                        typeindex.into(),
                                        &readers.p2aaddressindex_to_p2abytes,
                                    )?
                                    .map(|v| AddressBytes::from(v.into_owned())),
                                _ => {
                                    unreachable!()
                                }
                            };
                            let prev_addressbytes = prev_addressbytes_opt
                                .as_ref()
                                .ok_or(Error::Str("Expect to have addressbytes"))?;

                            let address_bytes = &tuple.5.as_ref().unwrap().0;

                            if stores.addressbyteshash_to_typeindex.needs(height)
                                && prev_addressbytes != address_bytes
                            {
                                let txid = tx.compute_txid();
                                dbg!(
                                    height,
                                    txid,
                                    vout,
                                    block_txindex,
                                    outputtype,
                                    prev_addressbytes,
                                    address_bytes,
                                    &idxs,
                                    typeindex,
                                    typeindex,
                                    txout,
                                    AddressBytesHash::from(address_bytes),
                                );
                                panic!()
                            }
                        }

                        Ok(tuple)
                    },
                )
                .collect::<Result<Vec<_>>>()?;
            // println!("txouts = : {:?}", i.elapsed());

            let outputs_len = txouts.len();
            let inputs_len = txins.len();
            let tx_len = block.txdata.len();

            // let i = Instant::now();
            txouts
                .into_iter()
                .try_for_each(|data| -> Result<()> {
                    let (
                        txoutindex,
                        txout,
                        txindex,
                        vout,
                        outputtype,
                        addressbytes_opt,
                        typeindex_opt,
                    ) = data;

                    let sats = Sats::from(txout.value);

                    if vout.is_zero() {
                        vecs.txindex_to_first_txoutindex
                            .push_if_needed(txindex, txoutindex)?;
                    }

                    vecs.txoutindex_to_value.push_if_needed(txoutindex, sats)?;

                    vecs.txoutindex_to_txindex
                        .push_if_needed(txoutindex, txindex)?;

                    vecs.txoutindex_to_outputtype
                        .push_if_needed(txoutindex, outputtype)?;

                    let typeindex = if let Some(ti) = typeindex_opt {
                        ti
                    } else if let Some((address_bytes, address_hash)) = addressbytes_opt {
                        if let Some(&ti) = already_added_addressbyteshash.get(&address_hash) {
                            ti
                        } else {
                            let ti = match outputtype {
                                OutputType::P2PK65 => idxs.p2pk65addressindex.copy_then_increment(),
                                OutputType::P2PK33 => idxs.p2pk33addressindex.copy_then_increment(),
                                OutputType::P2PKH => idxs.p2pkhaddressindex.copy_then_increment(),
                                OutputType::P2MS => {
                                    vecs.p2msoutputindex_to_txindex
                                        .push_if_needed(idxs.p2msoutputindex, txindex)?;
                                    idxs.p2msoutputindex.copy_then_increment()
                                }
                                OutputType::P2SH => idxs.p2shaddressindex.copy_then_increment(),
                                OutputType::OpReturn => {
                                    vecs.opreturnindex_to_txindex
                                        .push_if_needed(idxs.opreturnindex, txindex)?;
                                    idxs.opreturnindex.copy_then_increment()
                                }
                                OutputType::P2WPKH => idxs.p2wpkhaddressindex.copy_then_increment(),
                                OutputType::P2WSH => idxs.p2wshaddressindex.copy_then_increment(),
                                OutputType::P2TR => idxs.p2traddressindex.copy_then_increment(),
                                OutputType::P2A => idxs.p2aaddressindex.copy_then_increment(),
                                OutputType::Empty => {
                                    vecs.emptyoutputindex_to_txindex
                                        .push_if_needed(idxs.emptyoutputindex, txindex)?;
                                    idxs.emptyoutputindex.copy_then_increment()
                                }
                                OutputType::Unknown => {
                                    vecs.unknownoutputindex_to_txindex
                                        .push_if_needed(idxs.unknownoutputindex, txindex)?;
                                    idxs.unknownoutputindex.copy_then_increment()
                                }
                                _ => unreachable!(),
                            };

                            already_added_addressbyteshash.insert(address_hash, ti);
                            stores.addressbyteshash_to_typeindex.insert_if_needed(
                                address_hash,
                                ti,
                                height,
                            );
                            vecs.push_bytes_if_needed(ti, address_bytes)?;

                            ti
                        }
                    } else {
                        match outputtype {
                            OutputType::P2MS => {
                                vecs.p2msoutputindex_to_txindex
                                    .push_if_needed(idxs.p2msoutputindex, txindex)?;
                                idxs.p2msoutputindex.copy_then_increment()
                            }
                            OutputType::OpReturn => {
                                vecs.opreturnindex_to_txindex
                                    .push_if_needed(idxs.opreturnindex, txindex)?;
                                idxs.opreturnindex.copy_then_increment()
                            }
                            OutputType::Empty => {
                                vecs.emptyoutputindex_to_txindex
                                    .push_if_needed(idxs.emptyoutputindex, txindex)?;
                                idxs.emptyoutputindex.copy_then_increment()
                            }
                            OutputType::Unknown => {
                                vecs.unknownoutputindex_to_txindex
                                    .push_if_needed(idxs.unknownoutputindex, txindex)?;
                                idxs.unknownoutputindex.copy_then_increment()
                            }
                            _ => unreachable!(),
                        }
                    };

                    vecs.txoutindex_to_typeindex
                        .push_if_needed(txoutindex, typeindex)?;

                    if outputtype.is_unspendable() {
                        return Ok(());
                    } else if outputtype.is_address() {
                        stores
                            .addresstype_to_typeindex_and_txindex
                            .get_mut(outputtype)
                            .unwrap()
                            .insert_if_needed(
                                TypeIndexAndTxIndex::from((typeindex, txindex)),
                                Unit,
                                height,
                            );
                    }

                    let outpoint = OutPoint::new(txindex, vout);

                    if !same_block_spent_outpoints.contains(&outpoint) {
                        if outputtype.is_address() {
                            stores
                                .addresstype_to_typeindex_and_unspentoutpoint
                                .get_mut(outputtype)
                                .unwrap()
                                .insert_if_needed(
                                    TypeIndexAndOutPoint::from((typeindex, outpoint)),
                                    Unit,
                                    height,
                                );
                        }
                    } else {
                        same_block_output_info.insert(outpoint, (outputtype, typeindex));
                    }

                    Ok(())
                })?;
            // println!(
            //     "txouts.into_iter() = : {:?}",
            // i.elapsed()
            // );

            // let i = Instant::now();
            txins
                .into_iter()
                .map(
                    #[allow(clippy::type_complexity)]
                    |(txinindex, input_source)| -> Result<(
                        TxInIndex,
                        Vin,
                        TxIndex,
                        OutPoint,
                        Option<(OutputType, TypeIndex)>,
                    )> {
                        if let InputSource::PreviousBlock((
                            vin,
                            txindex,
                            outpoint,
                            outputtype_typeindex_opt,
                        )) = input_source
                        {
                            return Ok((
                                txinindex,
                                vin,
                                txindex,
                                outpoint,
                                outputtype_typeindex_opt,
                            ));
                        }

                        let InputSource::SameBlock((txindex, txin, vin, outpoint)) = input_source
                        else {
                            unreachable!()
                        };

                        let mut tuple = (txinindex, vin, txindex, outpoint, None);

                        if outpoint.is_coinbase() {
                            return Ok(tuple);
                        }

                        let outputtype_typeindex = same_block_output_info
                            .remove(&outpoint)
                            .ok_or(Error::Str("should have found addressindex from same block"))
                            .inspect_err(|_| {
                                dbg!(&same_block_output_info, txin);
                            })?;

                        if outputtype_typeindex.0.is_address() {
                            tuple.4 = Some(outputtype_typeindex);
                        }

                        Ok(tuple)
                    },
                )
                .try_for_each(|res| -> Result<()> {
                    let (txinindex, vin, txindex, outpoint, outputtype_typeindex_opt) = res?;

                    if vin.is_zero() {
                        vecs.txindex_to_first_txinindex
                            .push_if_needed(txindex, txinindex)?;
                    }

                    vecs.txinindex_to_outpoint
                        .push_if_needed(txinindex, outpoint)?;

                    let Some((outputtype, typeindex)) = outputtype_typeindex_opt else {
                        return Ok(());
                    };

                    stores
                        .addresstype_to_typeindex_and_txindex
                        .get_mut_unwrap(outputtype)
                        .insert_if_needed(
                            TypeIndexAndTxIndex::from((typeindex, txindex)),
                            Unit,
                            height,
                        );

                    stores
                        .addresstype_to_typeindex_and_unspentoutpoint
                        .get_mut_unwrap(outputtype)
                        .remove_if_needed(
                            TypeIndexAndOutPoint::from((typeindex, outpoint)),
                            height,
                        );

                    Ok(())
                })?;
            // println!("txins.into_iter(): {:?}", i.elapsed());

            // let i = Instant::now();
            if check_collisions {
                let mut txindex_to_txid_iter = vecs.txindex_to_txid.into_iter();
                txs.iter()
                    .try_for_each(|(txindex, _, _, _, prev_txindex_opt)| -> Result<()> {
                        let Some(prev_txindex) = prev_txindex_opt else {
                            return Ok(());
                        };

                        // In case if we start at an already parsed height
                        if txindex == prev_txindex {
                            return Ok(());
                        }

                        let len = vecs.txindex_to_txid.len();
                        // Ok if `get` is not par as should happen only twice
                        let prev_txid = txindex_to_txid_iter
                            .get(*prev_txindex)
                            .ok_or(Error::Str("To have txid for txindex"))
                            .inspect_err(|_| {
                                dbg!(txindex, len);
                            })?;

                        let prev_txid = prev_txid.as_ref();

                        // If another Txid needs to be added to the list
                        // We need to check that it's also a coinbase tx otherwise par_iter inputs needs to be updated
                        let only_known_dup_txids = [
                            bitcoin::Txid::from_str(
                                "d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599",
                            )
                            .unwrap()
                            .into(),
                            bitcoin::Txid::from_str(
                                "e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468",
                            )
                            .unwrap()
                            .into(),
                        ];

                        let is_dup = only_known_dup_txids.contains(prev_txid);

                        if !is_dup {
                            dbg!(height, txindex, prev_txid, prev_txindex);
                            return Err(Error::Str("Expect none"));
                        }

                        Ok(())
                    })?;
            }
            // println!("txindex_to_tx_and_txid = : {:?}", i.elapsed());

            // let i = Instant::now();
            txs.into_iter().try_for_each(
                |(txindex, tx, txid, txid_prefix, prev_txindex_opt)| -> Result<()> {
                    if prev_txindex_opt.is_none() {
                        stores
                            .txidprefix_to_txindex
                            .insert_if_needed(txid_prefix, txindex, height);
                    }

                    vecs.txindex_to_height.push_if_needed(txindex, height)?;
                    vecs.txindex_to_txversion
                        .push_if_needed(txindex, tx.version.into())?;
                    vecs.txindex_to_txid.push_if_needed(txindex, txid)?;
                    vecs.txindex_to_rawlocktime
                        .push_if_needed(txindex, tx.lock_time.into())?;
                    vecs.txindex_to_base_size
                        .push_if_needed(txindex, tx.base_size().into())?;
                    vecs.txindex_to_total_size
                        .push_if_needed(txindex, tx.total_size().into())?;
                    vecs.txindex_to_is_explicitly_rbf
                        .push_if_needed(txindex, StoredBool::from(tx.is_explicitly_rbf()))?;

                    Ok(())
                },
            )?;
            // println!("txindex_to_tx_and_txid.into_iter(): {:?}", i.elapsed());

            idxs.txindex += TxIndex::from(tx_len);
            idxs.txinindex += TxInIndex::from(inputs_len);
            idxs.txoutindex += TxOutIndex::from(outputs_len);

            // println!("full block: {:?}", i_tot.elapsed());

            if should_export(height, false) {
                drop(readers);
                export(stores, vecs, height, exit)?;
                readers = Readers::new(vecs);
            }
        }

        drop(readers);

        if should_export(idxs.height, true) {
            export(stores, vecs, idxs.height, exit)?;
        }

        // let i = Instant::now();
        self.vecs.punch_holes()?;
        // info!("Punched holes in db in {}s", i.elapsed().as_secs());

        Ok(starting_indexes)
    }

    pub fn static_clone(&self) -> &'static Self {
        Box::leak(Box::new(self.clone()))
    }
}

#[derive(Debug)]
enum InputSource<'a> {
    PreviousBlock((Vin, TxIndex, OutPoint, Option<(OutputType, TypeIndex)>)),
    SameBlock((TxIndex, &'a TxIn, Vin, OutPoint)),
}

struct Readers {
    txindex_to_first_txoutindex: Reader<'static>,
    txoutindex_to_outputtype: Reader<'static>,
    txoutindex_to_typeindex: Reader<'static>,
    p2pk65addressindex_to_p2pk65bytes: Reader<'static>,
    p2pk33addressindex_to_p2pk33bytes: Reader<'static>,
    p2pkhaddressindex_to_p2pkhbytes: Reader<'static>,
    p2shaddressindex_to_p2shbytes: Reader<'static>,
    p2wpkhaddressindex_to_p2wpkhbytes: Reader<'static>,
    p2wshaddressindex_to_p2wshbytes: Reader<'static>,
    p2traddressindex_to_p2trbytes: Reader<'static>,
    p2aaddressindex_to_p2abytes: Reader<'static>,
}

impl Readers {
    fn new(vecs: &mut Vecs) -> Self {
        Self {
            txindex_to_first_txoutindex: vecs.txindex_to_first_txoutindex.create_static_reader(),
            txoutindex_to_outputtype: vecs.txoutindex_to_outputtype.create_static_reader(),
            txoutindex_to_typeindex: vecs.txoutindex_to_typeindex.create_static_reader(),
            p2pk65addressindex_to_p2pk65bytes: vecs
                .p2pk65addressindex_to_p2pk65bytes
                .create_static_reader(),
            p2pk33addressindex_to_p2pk33bytes: vecs
                .p2pk33addressindex_to_p2pk33bytes
                .create_static_reader(),
            p2pkhaddressindex_to_p2pkhbytes: vecs
                .p2pkhaddressindex_to_p2pkhbytes
                .create_static_reader(),
            p2shaddressindex_to_p2shbytes: vecs
                .p2shaddressindex_to_p2shbytes
                .create_static_reader(),
            p2wpkhaddressindex_to_p2wpkhbytes: vecs
                .p2wpkhaddressindex_to_p2wpkhbytes
                .create_static_reader(),
            p2wshaddressindex_to_p2wshbytes: vecs
                .p2wshaddressindex_to_p2wshbytes
                .create_static_reader(),
            p2traddressindex_to_p2trbytes: vecs
                .p2traddressindex_to_p2trbytes
                .create_static_reader(),
            p2aaddressindex_to_p2abytes: vecs.p2aaddressindex_to_p2abytes.create_static_reader(),
        }
    }
}
