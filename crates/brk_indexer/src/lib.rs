#![doc = include_str!("../README.md")]

use std::{path::Path, str::FromStr, thread, time::Instant};

use bitcoin::{TxIn, TxOut};
use brk_error::{Error, Result};
use brk_iterator::Blocks;
use brk_rpc::Client;
use brk_store::AnyStore;
use brk_types::{
    AddressBytes, AddressBytesHash, AddressTypeAddressIndexOutPoint,
    AddressTypeAddressIndexTxIndex, BlockHashPrefix, Height, OutPoint, OutputType, Sats,
    StoredBool, Timestamp, TxInIndex, TxIndex, TxOutIndex, Txid, TxidPrefix, TypeIndex, Unit,
    Version, Vin, Vout,
};
use log::{error, info};
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use vecdb::{AnyVec, Exit, GenericStoredVec, Reader, VecIteratorExtended};
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
const VERSION: Version = Version::new(23);
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

    pub fn index(&mut self, blocks: &Blocks, client: &Client, exit: &Exit) -> Result<Indexes> {
        self.index_(blocks, client, exit, false)
    }

    pub fn checked_index(
        &mut self,
        blocks: &Blocks,
        client: &Client,
        exit: &Exit,
    ) -> Result<Indexes> {
        self.index_(blocks, client, exit, true)
    }

    fn index_(
        &mut self,
        blocks: &Blocks,
        client: &Client,
        exit: &Exit,
        check_collisions: bool,
    ) -> Result<Indexes> {
        let last_blockhash = self.vecs.height_to_blockhash.iter()?.last();
        let (starting_indexes, prev_hash) = if let Some(hash) = last_blockhash {
            let (height, hash) = client.get_closest_valid_height(hash)?;
            let starting_indexes =
                Indexes::from((height.incremented(), &mut self.vecs, &self.stores));
            if starting_indexes.height > client.get_last_height()? {
                info!("Up to date, nothing to index.");
                return Ok(starting_indexes);
            }
            (starting_indexes, Some(hash))
        } else {
            (Indexes::default(), None)
        };

        let lock = exit.lock();
        self.stores
            .rollback_if_needed(&mut self.vecs, &starting_indexes)?;
        self.vecs.rollback_if_needed(&starting_indexes)?;
        drop(lock);

        // Cloned because we want to return starting indexes for the computer
        let mut indexes = starting_indexes.clone();

        let should_export = |height: Height, rem: bool| -> bool {
            height != 0 && (height % SNAPSHOT_BLOCK_RANGE == 0) != rem
        };

        let export = move |stores: &mut Stores, vecs: &mut Vecs, height: Height| -> Result<()> {
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

        let mut readers = Readers::new(&self.vecs);

        let vecs = &mut self.vecs;
        let stores = &mut self.stores;

        for block in blocks.after(prev_hash)? {
            // let i_tot = Instant::now();

            let height = block.height();
            let blockhash = block.hash();

            info!("Indexing block {height}...");

            indexes.height = height;

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

            indexes.push_if_needed(vecs)?;

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
                        indexes.txindex + TxIndex::from(index),
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
                    let txindex = indexes.txindex + block_txindex;
                    let txinindex = indexes.txinindex + TxInIndex::from(block_txinindex);

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
                            (txindex < indexes.txindex).then_some(txindex)
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

                    let txoutindex = vecs.txindex_to_first_txoutindex.get_pushed_or_read_with(prev_txindex, &readers.txindex_to_first_txoutindex)?
                        .ok_or(Error::Str("Expect txoutindex to not be none"))
                        .inspect_err(|_| {
                            dbg!(outpoint.txid, prev_txindex, vout);
                        })?
                        + vout;

                    let outpoint = OutPoint::new(prev_txindex, vout);

                    let outputtype = vecs.txoutindex_to_outputtype.get_pushed_or_read_with(txoutindex, &readers.txoutindex_to_outputtype)?
                        .ok_or(Error::Str("Expect outputtype to not be none"))?;

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
                            .get_pushed_or_read_with(txoutindex, &readers.txoutindex_to_typeindex)?
                            .ok_or(Error::Str("Expect typeindex to not be none"))?;
                        tuple.3 = Some((outputtype, typeindex));
                    }

                    Ok((txinindex, InputSource::PreviousBlock(tuple)))
                })
                .collect::<Result<Vec<_>>>()?;
            drop(txid_prefix_to_txindex);
            // println!("txinindex_and_txindata = : {:?}", i.elapsed());

            // let i = Instant::now();
            let same_block_spent_outpoints: FxHashSet<OutPoint> = txins
                .iter()
                .filter_map(|(_, input_source)| {
                    let InputSource::SameBlock((_, _, _, outpoint)) = input_source else {
                        return None;
                    };
                    if !outpoint.is_coinbase() {
                        Some(*outpoint)
                    } else {
                        None
                    }
                })
                .collect();
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
                        let txindex = indexes.txindex + block_txindex;
                        let txoutindex = indexes.txoutindex + TxOutIndex::from(block_txoutindex);

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
                                (typeindex_local < indexes.to_typeindex(outputtype))
                                    .then_some(typeindex_local)
                            });

                        tuple.5 = Some((address_bytes, address_hash));
                        tuple.6 = typeindex_opt;

                        if check_collisions && let Some(typeindex) = typeindex_opt {
                            // unreachable!();

                            let prev_addressbytes_opt = match outputtype {
                                OutputType::P2PK65 => vecs
                                    .p2pk65addressindex_to_p2pk65bytes
                                    .get_pushed_or_read_with(
                                        typeindex.into(),
                                        &readers.p2pk65addressindex_to_p2pk65bytes,
                                    )?
                                    .map(AddressBytes::from),
                                OutputType::P2PK33 => vecs
                                    .p2pk33addressindex_to_p2pk33bytes
                                    .get_pushed_or_read_with(
                                        typeindex.into(),
                                        &readers.p2pk33addressindex_to_p2pk33bytes,
                                    )?
                                    .map(AddressBytes::from),
                                OutputType::P2PKH => vecs
                                    .p2pkhaddressindex_to_p2pkhbytes
                                    .get_pushed_or_read_with(
                                        typeindex.into(),
                                        &readers.p2pkhaddressindex_to_p2pkhbytes,
                                    )?
                                    .map(AddressBytes::from),
                                OutputType::P2SH => vecs
                                    .p2shaddressindex_to_p2shbytes
                                    .get_pushed_or_read_with(
                                        typeindex.into(),
                                        &readers.p2shaddressindex_to_p2shbytes,
                                    )?
                                    .map(AddressBytes::from),
                                OutputType::P2WPKH => vecs
                                    .p2wpkhaddressindex_to_p2wpkhbytes
                                    .get_pushed_or_read_with(
                                        typeindex.into(),
                                        &readers.p2wpkhaddressindex_to_p2wpkhbytes,
                                    )?
                                    .map(AddressBytes::from),
                                OutputType::P2WSH => vecs
                                    .p2wshaddressindex_to_p2wshbytes
                                    .get_pushed_or_read_with(
                                        typeindex.into(),
                                        &readers.p2wshaddressindex_to_p2wshbytes,
                                    )?
                                    .map(AddressBytes::from),
                                OutputType::P2TR => vecs
                                    .p2traddressindex_to_p2trbytes
                                    .get_pushed_or_read_with(
                                        typeindex.into(),
                                        &readers.p2traddressindex_to_p2trbytes,
                                    )?
                                    .map(AddressBytes::from),
                                OutputType::P2A => vecs
                                    .p2aaddressindex_to_p2abytes
                                    .get_pushed_or_read_with(
                                        typeindex.into(),
                                        &readers.p2aaddressindex_to_p2abytes,
                                    )?
                                    .map(AddressBytes::from),
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
                                    &indexes,
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
            let mut already_added_addressbyteshash: FxHashMap<AddressBytesHash, TypeIndex> =
                FxHashMap::default();
            let mut same_block_output_info: FxHashMap<OutPoint, (OutputType, TypeIndex)> =
                FxHashMap::default();
            for (txoutindex, txout, txindex, vout, outputtype, addressbytes_opt, typeindex_opt) in
                txouts
            {
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
                            OutputType::P2PK65 => indexes.p2pk65addressindex.copy_then_increment(),
                            OutputType::P2PK33 => indexes.p2pk33addressindex.copy_then_increment(),
                            OutputType::P2PKH => indexes.p2pkhaddressindex.copy_then_increment(),
                            OutputType::P2MS => {
                                vecs.p2msoutputindex_to_txindex
                                    .push_if_needed(indexes.p2msoutputindex, txindex)?;
                                indexes.p2msoutputindex.copy_then_increment()
                            }
                            OutputType::P2SH => indexes.p2shaddressindex.copy_then_increment(),
                            OutputType::OpReturn => {
                                vecs.opreturnindex_to_txindex
                                    .push_if_needed(indexes.opreturnindex, txindex)?;
                                indexes.opreturnindex.copy_then_increment()
                            }
                            OutputType::P2WPKH => indexes.p2wpkhaddressindex.copy_then_increment(),
                            OutputType::P2WSH => indexes.p2wshaddressindex.copy_then_increment(),
                            OutputType::P2TR => indexes.p2traddressindex.copy_then_increment(),
                            OutputType::P2A => indexes.p2aaddressindex.copy_then_increment(),
                            OutputType::Empty => {
                                vecs.emptyoutputindex_to_txindex
                                    .push_if_needed(indexes.emptyoutputindex, txindex)?;
                                indexes.emptyoutputindex.copy_then_increment()
                            }
                            OutputType::Unknown => {
                                vecs.unknownoutputindex_to_txindex
                                    .push_if_needed(indexes.unknownoutputindex, txindex)?;
                                indexes.unknownoutputindex.copy_then_increment()
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
                                .push_if_needed(indexes.p2msoutputindex, txindex)?;
                            indexes.p2msoutputindex.copy_then_increment()
                        }
                        OutputType::OpReturn => {
                            vecs.opreturnindex_to_txindex
                                .push_if_needed(indexes.opreturnindex, txindex)?;
                            indexes.opreturnindex.copy_then_increment()
                        }
                        OutputType::Empty => {
                            vecs.emptyoutputindex_to_txindex
                                .push_if_needed(indexes.emptyoutputindex, txindex)?;
                            indexes.emptyoutputindex.copy_then_increment()
                        }
                        OutputType::Unknown => {
                            vecs.unknownoutputindex_to_txindex
                                .push_if_needed(indexes.unknownoutputindex, txindex)?;
                            indexes.unknownoutputindex.copy_then_increment()
                        }
                        _ => unreachable!(),
                    }
                };

                vecs.txoutindex_to_typeindex
                    .push_if_needed(txoutindex, typeindex)?;

                if outputtype.is_unspendable() {
                    continue;
                } else if outputtype.is_address() {
                    let addresstype = outputtype;
                    let addressindex = typeindex;

                    stores
                        .addresstype_to_addressindex_and_txindex
                        .insert_if_needed(
                            AddressTypeAddressIndexTxIndex::from((
                                addresstype,
                                addressindex,
                                txindex,
                            )),
                            Unit,
                            height,
                        );
                }

                let outpoint = OutPoint::new(txindex, vout);

                if same_block_spent_outpoints.contains(&outpoint) {
                    same_block_output_info.insert(outpoint, (outputtype, typeindex));
                } else if outputtype.is_address() {
                    let addresstype = outputtype;
                    let addressindex = typeindex;

                    stores
                        .addresstype_to_addressindex_and_unspentoutpoint
                        .insert_if_needed(
                            AddressTypeAddressIndexOutPoint::from((
                                addresstype,
                                addressindex,
                                outpoint,
                            )),
                            Unit,
                            height,
                        );
                }
            }
            // println!(
            //     "txouts.into_iter() = : {:?}",
            // i.elapsed()
            // );

            // let i = Instant::now();
            for (txinindex, input_source) in txins {
                let (vin, txindex, outpoint, addresstype_addressindex_opt) = match input_source {
                    InputSource::PreviousBlock(tuple) => tuple,
                    InputSource::SameBlock((txindex, txin, vin, outpoint)) => {
                        let mut tuple = (vin, txindex, outpoint, None);
                        if outpoint.is_coinbase() {
                            tuple
                        } else {
                            let outputtype_typeindex = same_block_output_info
                                .remove(&outpoint)
                                .ok_or(Error::Str("should have found addressindex from same block"))
                                .inspect_err(|_| {
                                    dbg!(&same_block_output_info, txin);
                                })?;
                            if outputtype_typeindex.0.is_address() {
                                tuple.3 = Some(outputtype_typeindex);
                            }
                            (tuple.0, tuple.1, tuple.2, tuple.3)
                        }
                    }
                };

                if vin.is_zero() {
                    vecs.txindex_to_first_txinindex
                        .push_if_needed(txindex, txinindex)?;
                }

                vecs.txinindex_to_outpoint
                    .push_if_needed(txinindex, outpoint)?;

                let Some((addresstype, addressindex)) = addresstype_addressindex_opt else {
                    continue;
                };

                stores
                    .addresstype_to_addressindex_and_txindex
                    .insert_if_needed(
                        AddressTypeAddressIndexTxIndex::from((addresstype, addressindex, txindex)),
                        Unit,
                        height,
                    );

                stores
                    .addresstype_to_addressindex_and_unspentoutpoint
                    .remove_if_needed(
                        AddressTypeAddressIndexOutPoint::from((
                            addresstype,
                            addressindex,
                            outpoint,
                        )),
                        height,
                    );
            }
            // println!("txins.into_iter(): {:?}", i.elapsed());

            // let i = Instant::now();
            if check_collisions {
                let mut txindex_to_txid_iter = vecs.txindex_to_txid.into_iter();
                for (txindex, _, _, _, prev_txindex_opt) in txs.iter() {
                    let Some(prev_txindex) = prev_txindex_opt else {
                        continue;
                    };

                    // In case if we start at an already parsed height
                    if txindex == prev_txindex {
                        continue;
                    }

                    let len = vecs.txindex_to_txid.len();
                    // Ok if `get` is not par as should happen only twice
                    let prev_txid = txindex_to_txid_iter
                        .get(*prev_txindex)
                        .ok_or(Error::Str("To have txid for txindex"))
                        .inspect_err(|_| {
                            dbg!(txindex, len);
                        })?;

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

                    let is_dup = only_known_dup_txids.contains(&prev_txid);

                    if !is_dup {
                        dbg!(height, txindex, prev_txid, prev_txindex);
                        return Err(Error::Str("Expect none"));
                    }
                }
            }
            // println!("txindex_to_tx_and_txid = : {:?}", i.elapsed());

            // let i = Instant::now();
            for (txindex, tx, txid, txid_prefix, prev_txindex_opt) in txs {
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
            }
            // println!("txindex_to_tx_and_txid.into_iter(): {:?}", i.elapsed());

            indexes.txindex += TxIndex::from(tx_len);
            indexes.txinindex += TxInIndex::from(inputs_len);
            indexes.txoutindex += TxOutIndex::from(outputs_len);

            // println!("full block: {:?}", i_tot.elapsed());

            if should_export(height, false) {
                drop(readers);
                export(stores, vecs, height)?;
                readers = Readers::new(vecs);
            }
        }

        drop(readers);

        if should_export(indexes.height, true) {
            export(stores, vecs, indexes.height)?;
        }

        // let i = Instant::now();
        self.vecs.compact()?;
        // info!("Punched holes in db in {}s", i.elapsed().as_secs());

        Ok(starting_indexes)
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
    fn new(vecs: &Vecs) -> Self {
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
