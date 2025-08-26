#![doc = include_str!("../README.md")]

use std::{collections::BTreeMap, path::Path, str::FromStr, thread, time::Instant};

use bitcoin::{Transaction, TxIn, TxOut};
use brk_error::{Error, Result};

use brk_parser::Parser;
use brk_store::AnyStore;
use brk_structs::{
    AddressBytes, AddressBytesHash, BlockHash, BlockHashPrefix, Height, InputIndex, OutputIndex,
    OutputType, Sats, StoredBool, Timestamp, TxIndex, Txid, TxidPrefix, TypeIndex,
    TypeIndexWithOutputindex, Unit, Version, Vin, Vout,
};
use log::{error, info};
use rayon::prelude::*;
use vecdb::{AnyVec, Database, Exit, GenericStoredVec, PAGE_SIZE, Reader, VecIterator};
mod indexes;
mod stores;
mod vecs;

pub use indexes::*;
pub use stores::*;
pub use vecs::*;

const SNAPSHOT_BLOCK_RANGE: usize = 1_000;
const COLLISIONS_CHECKED_UP_TO: Height = Height::new(909_150);
const VERSION: Version = Version::ONE;

#[derive(Clone)]
pub struct Indexer {
    pub db: Database,
    pub vecs: Vecs,
    pub stores: Stores,
}

impl Indexer {
    pub fn forced_import(outputs_dir: &Path) -> Result<Self> {
        info!("Importing indexer...");

        let db = Database::open(&outputs_dir.join("indexed/vecs"))?;
        db.set_min_len(PAGE_SIZE * 50_000_000)?;
        info!("Opened database");

        let vecs = Vecs::forced_import(&db, VERSION + Version::ZERO)?;
        info!("Imported vecs");

        let stores =
            Stores::forced_import(&outputs_dir.join("indexed/stores"), VERSION + Version::ZERO)?;
        info!("Imported stores");

        Ok(Self { vecs, stores, db })
    }

    pub fn index(
        &mut self,
        parser: &Parser,
        rpc: &'static bitcoincore_rpc::Client,
        exit: &Exit,
        check_collisions: bool,
    ) -> Result<Indexes> {
        let db = self.db.clone();

        // dbg!(self.db.regions().id_to_index());
        // dbg!(self.db.layout());

        let starting_indexes = Indexes::try_from((&mut self.vecs, &self.stores, rpc))
            .unwrap_or_else(|_report| Indexes::default());

        // dbg!(&starting_indexes);

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
                let _lock = exit.lock();
                let i = Instant::now();
                stores.commit(height).unwrap();
                info!("Commited stores in {}s", i.elapsed().as_secs());
                let i = Instant::now();
                vecs.flush(height)?;
                info!("Flushed vecs in {}s", i.elapsed().as_secs());
                let i = Instant::now();
                db.flush()?;
                info!("Flushed db in {}s", i.elapsed().as_secs());
                Ok(())
            };

        let mut txindex_to_first_outputindex_reader_opt = None;
        let mut p2pk65addressindex_to_p2pk65bytes_reader_opt = None;
        let mut p2pk33addressindex_to_p2pk33bytes_reader_opt = None;
        let mut p2pkhaddressindex_to_p2pkhbytes_reader_opt = None;
        let mut p2shaddressindex_to_p2shbytes_reader_opt = None;
        let mut p2wpkhaddressindex_to_p2wpkhbytes_reader_opt = None;
        let mut p2wshaddressindex_to_p2wshbytes_reader_opt = None;
        let mut p2traddressindex_to_p2trbytes_reader_opt = None;
        let mut p2aaddressindex_to_p2abytes_reader_opt = None;

        let reset_readers =
            |vecs: &mut Vecs,
             txindex_to_first_outputindex_reader_opt: &mut Option<Reader<'static>>,
             p2pk65addressindex_to_p2pk65bytes_reader_opt: &mut Option<Reader<'static>>,
             p2pk33addressindex_to_p2pk33bytes_reader_opt: &mut Option<Reader<'static>>,
             p2pkhaddressindex_to_p2pkhbytes_reader_opt: &mut Option<Reader<'static>>,
             p2shaddressindex_to_p2shbytes_reader_opt: &mut Option<Reader<'static>>,
             p2wpkhaddressindex_to_p2wpkhbytes_reader_opt: &mut Option<Reader<'static>>,
             p2wshaddressindex_to_p2wshbytes_reader_opt: &mut Option<Reader<'static>>,
             p2traddressindex_to_p2trbytes_reader_opt: &mut Option<Reader<'static>>,
             p2aaddressindex_to_p2abytes_reader_opt: &mut Option<Reader<'static>>| {
                txindex_to_first_outputindex_reader_opt
                    .replace(vecs.txindex_to_first_outputindex.create_static_reader());
                p2pk65addressindex_to_p2pk65bytes_reader_opt.replace(
                    vecs.p2pk65addressindex_to_p2pk65bytes
                        .create_static_reader(),
                );
                p2pk33addressindex_to_p2pk33bytes_reader_opt.replace(
                    vecs.p2pk33addressindex_to_p2pk33bytes
                        .create_static_reader(),
                );
                p2pkhaddressindex_to_p2pkhbytes_reader_opt
                    .replace(vecs.p2pkhaddressindex_to_p2pkhbytes.create_static_reader());
                p2shaddressindex_to_p2shbytes_reader_opt
                    .replace(vecs.p2shaddressindex_to_p2shbytes.create_static_reader());
                p2wpkhaddressindex_to_p2wpkhbytes_reader_opt.replace(
                    vecs.p2wpkhaddressindex_to_p2wpkhbytes
                        .create_static_reader(),
                );
                p2wshaddressindex_to_p2wshbytes_reader_opt
                    .replace(vecs.p2wshaddressindex_to_p2wshbytes.create_static_reader());
                p2traddressindex_to_p2trbytes_reader_opt
                    .replace(vecs.p2traddressindex_to_p2trbytes.create_static_reader());
                p2aaddressindex_to_p2abytes_reader_opt
                    .replace(vecs.p2aaddressindex_to_p2abytes.create_static_reader());
            };

        reset_readers(
            vecs,
            &mut txindex_to_first_outputindex_reader_opt,
            &mut p2pk65addressindex_to_p2pk65bytes_reader_opt,
            &mut p2pk33addressindex_to_p2pk33bytes_reader_opt,
            &mut p2pkhaddressindex_to_p2pkhbytes_reader_opt,
            &mut p2shaddressindex_to_p2shbytes_reader_opt,
            &mut p2wpkhaddressindex_to_p2wpkhbytes_reader_opt,
            &mut p2wshaddressindex_to_p2wshbytes_reader_opt,
            &mut p2traddressindex_to_p2trbytes_reader_opt,
            &mut p2aaddressindex_to_p2abytes_reader_opt,
        );

        parser.parse(start, end).iter().try_for_each(
            |(height, block, blockhash)| -> Result<()> {
                info!("Indexing block {height}...");

                idxs.height = height;

                let txindex_to_first_outputindex_reader = txindex_to_first_outputindex_reader_opt.as_ref().unwrap();
                let p2pk65addressindex_to_p2pk65bytes_reader = p2pk65addressindex_to_p2pk65bytes_reader_opt.as_ref().unwrap();
                let p2pk33addressindex_to_p2pk33bytes_reader = p2pk33addressindex_to_p2pk33bytes_reader_opt.as_ref().unwrap();
                let p2pkhaddressindex_to_p2pkhbytes_reader = p2pkhaddressindex_to_p2pkhbytes_reader_opt.as_ref().unwrap();
                let p2shaddressindex_to_p2shbytes_reader = p2shaddressindex_to_p2shbytes_reader_opt.as_ref().unwrap();
                let p2wpkhaddressindex_to_p2wpkhbytes_reader = p2wpkhaddressindex_to_p2wpkhbytes_reader_opt.as_ref().unwrap();
                let p2wshaddressindex_to_p2wshbytes_reader = p2wshaddressindex_to_p2wshbytes_reader_opt.as_ref().unwrap();
                let p2traddressindex_to_p2trbytes_reader = p2traddressindex_to_p2trbytes_reader_opt.as_ref().unwrap();
                let p2aaddressindex_to_p2abytes_reader = p2aaddressindex_to_p2abytes_reader_opt.as_ref().unwrap();

                // Used to check rapidhash collisions
                let check_collisions = check_collisions && height > COLLISIONS_CHECKED_UP_TO ;

                let blockhash = BlockHash::from(blockhash);
                let blockhash_prefix = BlockHashPrefix::from(&blockhash);

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

                vecs.height_to_blockhash.push_if_needed(height, blockhash)?;
                vecs.height_to_difficulty
                    .push_if_needed(height, block.header.difficulty_float().into())?;
                vecs.height_to_timestamp
                    .push_if_needed(height, Timestamp::from(block.header.time))?;
                vecs.height_to_total_size.push_if_needed(height, block.total_size().into())?;
                vecs.height_to_weight.push_if_needed(height, block.weight().into())?;

                let (
                    txid_prefix_to_txid_and_block_txindex_and_prev_txindex_join_handle,
                    input_source_vec_handle,
                    outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt_handle,
                ) = thread::scope(|scope| {
                    let txid_prefix_to_txid_and_block_txindex_and_prev_txindex_handle =
                        scope.spawn(|| -> Result<_> {
                            block
                                .txdata
                                .iter()
                                .enumerate()
                                .map(|(index, tx)| {
                                    let txid = Txid::from(tx.compute_txid());

                                    let txid_prefix = TxidPrefix::from(&txid);

                                    let prev_txindex_opt =
                                        if check_collisions && stores.txidprefix_to_txindex.needs(height) {
                                            // Should only find collisions for two txids (duplicates), see below
                                            stores.txidprefix_to_txindex.get(&txid_prefix)?.map(|v| *v)
                                        } else {
                                            None
                                        };

                                    Ok((txid_prefix, (tx, txid, TxIndex::from(index), prev_txindex_opt)))
                                })
                                .collect::<Result<BTreeMap<_, _>>>()
                        });

                    let input_source_vec_handle = scope.spawn(|| {
                        let inputs = block
                            .txdata
                            .iter()
                            .enumerate()
                            .flat_map(|(index, tx)| {
                                tx.input
                                    .iter()
                                    .enumerate()
                                    .map(move |(vin, txin)| (TxIndex::from(index), Vin::from(vin), txin, tx))
                            })
                            .collect::<Vec<_>>();

                        inputs
                            .into_par_iter()
                            .enumerate()
                            .map(|(block_inputindex, (block_txindex, vin, txin, tx))| -> Result<(InputIndex, InputSource)> {
                                let txindex = idxs.txindex + block_txindex;
                                let inputindex = idxs.inputindex + InputIndex::from(block_inputindex);

                                let outpoint = txin.previous_output;
                                let txid = Txid::from(outpoint.txid);

                                if tx.is_coinbase() {
                                    return Ok((inputindex, InputSource::SameBlock((tx, txindex, txin, vin))));
                                }

                                let prev_txindex = if let Some(txindex) = stores
                                    .txidprefix_to_txindex
                                    .get(&TxidPrefix::from(&txid))?
                                    .map(|v| *v)
                                    .and_then(|txindex| {
                                        // Checking if not finding txindex from the future
                                        (txindex < idxs.txindex).then_some(txindex)
                                    }) {
                                    txindex
                                } else {
                                    // dbg!(indexes.txindex + block_txindex, txindex, txin, vin);
                                    return Ok((inputindex, InputSource::SameBlock((tx, txindex, txin, vin))));
                                };

                                let vout = Vout::from(outpoint.vout);

                                let outputindex = vecs.txindex_to_first_outputindex.get_or_read(prev_txindex, txindex_to_first_outputindex_reader)?
                                    .ok_or(Error::Str("Expect outputindex to not be none"))
                                    .inspect_err(|_| {
                                        dbg!(outpoint.txid, prev_txindex, vout);
                                    })?.into_owned()
                                    + vout;

                                Ok((inputindex, InputSource::PreviousBlock((
                                    vin,
                                    txindex,
                                    outputindex,
                                ))))
                            })
                            .try_fold(BTreeMap::new, |mut map, tuple| -> Result<_> {
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

                    let outputs = block
                        .txdata
                        .iter()
                        .enumerate()
                        .flat_map(|(index, tx)| {
                            tx.output
                                .iter()
                                .enumerate()
                                .map(move |(vout, txout)| (TxIndex::from(index), Vout::from(vout), txout, tx))
                        }).collect::<Vec<_>>();

                    let outputindex_to_txout_outputtype_addressbytes_res_addressindex = outputs.into_par_iter()
                        .enumerate()
                        .map(
                            #[allow(clippy::type_complexity)]
                            |(block_outputindex, (block_txindex, vout, txout, tx))| -> Result<(
                                OutputIndex,
                                (
                                    &TxOut,
                                    TxIndex,
                                    Vout,
                                    OutputType,
                                    Result<AddressBytes>,
                                    Option<TypeIndex>,
                                    &Transaction,
                                ),
                            )> {
                                let txindex = idxs.txindex + block_txindex;
                                let outputindex = idxs.outputindex + OutputIndex::from(block_outputindex);

                                let script = &txout.script_pubkey;

                                let outputtype = OutputType::from(script);

                                let address_bytes_res =
                                    AddressBytes::try_from((script, outputtype)).inspect_err(|_| {
                                        // dbg!(&txout, height, txi, &tx.compute_txid());
                                    });

                                let typeindex_opt = address_bytes_res.as_ref().ok().and_then(|addressbytes| {
                                    stores
                                        .addressbyteshash_to_typeindex
                                        .get(&AddressBytesHash::from((addressbytes, outputtype)))
                                        .unwrap()
                                        .map(|v| *v)
                                        // Checking if not in the future
                                        .and_then(|typeindex_local| {
                                            (typeindex_local < idxs.typeindex(outputtype)).then_some(typeindex_local)
                                        })
                                });

                                if let Some(Some(typeindex)) = check_collisions.then_some(typeindex_opt) {
                                    let addressbytes = address_bytes_res.as_ref().unwrap();

                                    let prev_addressbytes_opt = match outputtype {
                                        OutputType::P2PK65 => vecs
                                            .p2pk65addressindex_to_p2pk65bytes
                                            .get_or_read(typeindex.into(), p2pk65addressindex_to_p2pk65bytes_reader)?
                                            .map(|v| AddressBytes::from(v.into_owned())),
                                        OutputType::P2PK33 => vecs
                                            .p2pk33addressindex_to_p2pk33bytes
                                            .get_or_read(typeindex.into(), p2pk33addressindex_to_p2pk33bytes_reader)?
                                            .map(|v| AddressBytes::from(v.into_owned())),
                                        OutputType::P2PKH => vecs
                                            .p2pkhaddressindex_to_p2pkhbytes
                                            .get_or_read(typeindex.into(), p2pkhaddressindex_to_p2pkhbytes_reader)?
                                            .map(|v| AddressBytes::from(v.into_owned())),
                                        OutputType::P2SH => vecs
                                            .p2shaddressindex_to_p2shbytes
                                            .get_or_read(typeindex.into(), p2shaddressindex_to_p2shbytes_reader)?
                                            .map(|v| AddressBytes::from(v.into_owned())),
                                        OutputType::P2WPKH => vecs
                                            .p2wpkhaddressindex_to_p2wpkhbytes
                                            .get_or_read(typeindex.into(), p2wpkhaddressindex_to_p2wpkhbytes_reader)?
                                            .map(|v| AddressBytes::from(v.into_owned())),
                                        OutputType::P2WSH => vecs
                                            .p2wshaddressindex_to_p2wshbytes
                                            .get_or_read(typeindex.into(), p2wshaddressindex_to_p2wshbytes_reader)?
                                            .map(|v| AddressBytes::from(v.into_owned())),
                                        OutputType::P2TR => vecs
                                            .p2traddressindex_to_p2trbytes
                                            .get_or_read(typeindex.into(), p2traddressindex_to_p2trbytes_reader)?
                                            .map(|v| AddressBytes::from(v.into_owned())),
                                        OutputType::P2A => vecs
                                            .p2aaddressindex_to_p2abytes
                                            .get_or_read(typeindex.into(), p2aaddressindex_to_p2abytes_reader)?
                                            .map(|v| AddressBytes::from(v.into_owned())),
                                        _ => {
                                            unreachable!()
                                        }
                                    };
                                    let prev_addressbytes =
                                        prev_addressbytes_opt.as_ref().ok_or(Error::Str("Expect to have addressbytes"))?;

                                    if stores.addressbyteshash_to_typeindex.needs(height)
                                            && prev_addressbytes != addressbytes
                                    {
                                        let txid = tx.compute_txid();
                                        dbg!(
                                            height,
                                            txid,
                                            vout,
                                            block_txindex,
                                            outputtype,
                                            prev_addressbytes,
                                            addressbytes,
                                            &idxs,
                                            typeindex,
                                            typeindex,
                                            txout,
                                            AddressBytesHash::from((addressbytes, outputtype)),
                                        );
                                        panic!()
                                    }
                                }

                                Ok((
                                    outputindex,
                                    (
                                        txout,
                                        txindex,
                                        vout,
                                        outputtype,
                                        address_bytes_res,
                                        typeindex_opt,
                                        tx,
                                    ),
                                ))
                            },
                        )
                        .try_fold(BTreeMap::new, |mut map, tuple| -> Result<_> {
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
                        });

                    (
                        txid_prefix_to_txid_and_block_txindex_and_prev_txindex_handle.join(),
                        input_source_vec_handle.join(),
                        outputindex_to_txout_outputtype_addressbytes_res_addressindex,
                    )
                });

                let txid_prefix_to_txid_and_block_txindex_and_prev_txindex =
                    txid_prefix_to_txid_and_block_txindex_and_prev_txindex_join_handle
                        .map_err(|_|
                            Error::Str("Expect txid_prefix_to_txid_and_block_txindex_and_prev_txindex_join_handle to join")
                        )??;

                let input_source_vec = input_source_vec_handle
                    .map_err(|_|
                        Error::Str("Export input_source_vec_handle to join")
                    )??;

                let outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt =
                    outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt_handle
                        .map_err(|_|
                            Error::Str("Expect outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt_handle to join")
                        )?;

                let outputs_len = outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt.len();
                let inputs_len = input_source_vec.len();
                let tx_len = block.txdata.len();

                let mut new_txindexvout_to_outputindex: BTreeMap<
                    (TxIndex, Vout),
                    OutputIndex,
                > = BTreeMap::new();

                let mut already_added_addressbyteshash: BTreeMap<AddressBytesHash, TypeIndex> = BTreeMap::new();

                outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt
                .into_iter()
                .try_for_each(
                    |(
                        outputindex,
                        (txout, txindex, vout, outputtype, addressbytes_res, typeindex_opt, _tx),
                    )|
                     -> Result<()> {
                        let sats = Sats::from(txout.value);

                        if vout.is_zero() {
                            vecs.txindex_to_first_outputindex.push_if_needed(txindex, outputindex)?;
                        }

                        vecs.outputindex_to_value.push_if_needed(outputindex, sats)?;

                        vecs.outputindex_to_outputtype
                            .push_if_needed(outputindex, outputtype)?;

                        let mut addressbyteshash = None;

                        let typeindex;

                        if let Some(typeindex_local) = typeindex_opt.or_else(|| {
                            addressbytes_res.as_ref().ok().and_then(|addressbytes| {
                                // Check if address was first seen before in this iterator
                                // Example: https://mempool.space/address/046a0765b5865641ce08dd39690aade26dfbf5511430ca428a3089261361cef170e3929a68aee3d8d4848b0c5111b0a37b82b86ad559fd2a745b44d8e8d9dfdc0c
                                addressbyteshash.replace(AddressBytesHash::from((addressbytes, outputtype)));
                                already_added_addressbyteshash
                                    .get(addressbyteshash.as_ref().unwrap())
                                    .cloned()
                            })
                        }) {
                            typeindex = typeindex_local;
                        } else {
                            typeindex = match outputtype {
                                OutputType::P2PK65 => {
                                    idxs.p2pk65addressindex.copy_then_increment()
                                },
                                OutputType::P2PK33 => {
                                    idxs.p2pk33addressindex.copy_then_increment()
                                },
                                OutputType::P2PKH => {
                                    idxs.p2pkhaddressindex.copy_then_increment()
                                },
                                OutputType::P2MS => {
                                    vecs.p2msoutputindex_to_txindex.push_if_needed(idxs.p2msoutputindex, txindex)?;
                                    idxs.p2msoutputindex.copy_then_increment()
                                },
                                OutputType::P2SH => {
                                    idxs.p2shaddressindex.copy_then_increment()
                                },
                                OutputType::OpReturn => {
                                    vecs.opreturnindex_to_txindex.push_if_needed(idxs.opreturnindex, txindex)?;
                                    idxs.opreturnindex.copy_then_increment()
                                },
                                OutputType::P2WPKH => {
                                    idxs.p2wpkhaddressindex.copy_then_increment()
                                },
                                OutputType::P2WSH => {
                                    idxs.p2wshaddressindex.copy_then_increment()
                                },
                                OutputType::P2TR => {
                                    idxs.p2traddressindex.copy_then_increment()
                                },
                                OutputType::P2A => {
                                    idxs.p2aaddressindex.copy_then_increment()
                                },
                                OutputType::Empty => {
                                    vecs.emptyoutputindex_to_txindex
                                        .push_if_needed(idxs.emptyoutputindex, txindex)?;
                                    idxs.emptyoutputindex.copy_then_increment()
                                },
                                OutputType::Unknown => {
                                    vecs.unknownoutputindex_to_txindex.push_if_needed(idxs.unknownoutputindex, txindex)?;
                                    idxs.unknownoutputindex.copy_then_increment()
                                },
                                _ => unreachable!()
                            };

                            if let Ok(addressbytes) = addressbytes_res {
                                let addressbyteshash = addressbyteshash.unwrap();

                                already_added_addressbyteshash
                                    .insert(addressbyteshash, typeindex);

                                stores.addressbyteshash_to_typeindex.insert_if_needed(
                                    addressbyteshash,
                                    typeindex,
                                    height,
                                );

                                vecs.push_bytes_if_needed(typeindex, addressbytes)?;
                            }
                        }

                        vecs.outputindex_to_typeindex
                            .push_if_needed(outputindex, typeindex)?;

                        new_txindexvout_to_outputindex
                            .insert((txindex, vout), outputindex);

                        if outputtype.is_address() {
                            stores.addresstype_to_typeindex_with_outputindex.get_mut(outputtype).unwrap().insert_if_needed(TypeIndexWithOutputindex::from((typeindex, outputindex)), Unit, height);
                        }

                        Ok(())
                    },
                )?;

                drop(already_added_addressbyteshash);

                input_source_vec
                    .into_iter()
                    .map(
                        #[allow(clippy::type_complexity)]
                        |(inputindex, input_source)| -> Result<(
                            InputIndex, Vin, TxIndex, OutputIndex
                        )> {
                            match input_source {
                                InputSource::PreviousBlock((vin, txindex, outputindex)) => Ok((inputindex, vin, txindex, outputindex)),
                                InputSource::SameBlock((tx, txindex, txin, vin)) => {
                                    if tx.is_coinbase() {
                                        return Ok((inputindex, vin, txindex, OutputIndex::COINBASE));
                                    }

                                    let outpoint = txin.previous_output;
                                    let txid = Txid::from(outpoint.txid);
                                    let vout = Vout::from(outpoint.vout);

                                    let block_txindex = txid_prefix_to_txid_and_block_txindex_and_prev_txindex
                                        .get(&TxidPrefix::from(&txid))
                                        .ok_or(Error::Str("txid should be in same block")).inspect_err(|_| {
                                            dbg!(&txid_prefix_to_txid_and_block_txindex_and_prev_txindex);
                                            // panic!();
                                        })?
                                        .2;
                                    let prev_txindex = idxs.txindex + block_txindex;

                                    let prev_outputindex = new_txindexvout_to_outputindex
                                        .remove(&(prev_txindex, vout))
                                        .ok_or(Error::Str("should have found addressindex from same block"))
                                        .inspect_err(|_| {
                                            dbg!(&new_txindexvout_to_outputindex, txin, prev_txindex, vout, txid);
                                        })?;

                                    Ok((inputindex, vin, txindex, prev_outputindex))
                                }
                            }
                        },
                    )
                    .try_for_each(|res| -> Result<()> {
                        let (inputindex, vin, txindex, outputindex) = res?;

                        if vin.is_zero() {
                            vecs.txindex_to_first_inputindex.push_if_needed(txindex, inputindex)?;
                        }

                        vecs.inputindex_to_outputindex.push_if_needed(inputindex, outputindex)?;

                        Ok(())
                    })?;

                drop(new_txindexvout_to_outputindex);

                let mut txindex_to_tx_and_txid: BTreeMap<TxIndex, (&Transaction, Txid)> = BTreeMap::default();

                let mut txindex_to_txid_iter = vecs
                    .txindex_to_txid.into_iter();

                txid_prefix_to_txid_and_block_txindex_and_prev_txindex
                    .into_iter()
                    .try_for_each(
                        |(txid_prefix, (tx, txid, index, prev_txindex_opt))| -> Result<()> {
                            let txindex = idxs.txindex + index;

                            txindex_to_tx_and_txid.insert(txindex, (tx, txid));

                            match prev_txindex_opt {
                                None => {
                                    stores
                                        .txidprefix_to_txindex
                                        .insert_if_needed(txid_prefix, txindex, height);
                                }
                                Some(prev_txindex) => {
                                    // In case if we start at an already parsed height
                                    if txindex == prev_txindex {
                                        return Ok(());
                                    }

                                    if !check_collisions {
                                        return Ok(());
                                    }

                                    let len = vecs.txindex_to_txid.len();
                                    // Ok if `get` is not par as should happen only twice
                                    let prev_txid = txindex_to_txid_iter
                                        .get(prev_txindex)
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
                                }
                            }

                            Ok(())
                        },
                    )?;

                drop(txindex_to_txid_iter);

                txindex_to_tx_and_txid
                    .into_iter()
                    .try_for_each(|(txindex, (tx, txid))| -> Result<()> {
                        vecs.txindex_to_txversion.push_if_needed(txindex, tx.version.into())?;
                        vecs.txindex_to_txid.push_if_needed(txindex, txid)?;
                        vecs.txindex_to_rawlocktime.push_if_needed(txindex, tx.lock_time.into())?;
                        vecs.txindex_to_base_size.push_if_needed(txindex, tx.base_size().into())?;
                        vecs.txindex_to_total_size.push_if_needed(txindex, tx.total_size().into())?;
                        vecs.txindex_to_is_explicitly_rbf.push_if_needed(txindex, StoredBool::from(tx.is_explicitly_rbf()))?;
                        Ok(())
                    })?;

                idxs.txindex += TxIndex::from(tx_len);
                idxs.inputindex += InputIndex::from(inputs_len);
                idxs.outputindex += OutputIndex::from(outputs_len);


                if should_export(height, false) {
                    txindex_to_first_outputindex_reader_opt.take();
                    p2pk65addressindex_to_p2pk65bytes_reader_opt.take();
                    p2pk33addressindex_to_p2pk33bytes_reader_opt.take();
                    p2pkhaddressindex_to_p2pkhbytes_reader_opt.take();
                    p2shaddressindex_to_p2shbytes_reader_opt.take();
                    p2wpkhaddressindex_to_p2wpkhbytes_reader_opt.take();
                    p2wshaddressindex_to_p2wshbytes_reader_opt.take();
                    p2traddressindex_to_p2trbytes_reader_opt.take();
                    p2aaddressindex_to_p2abytes_reader_opt.take();

                    export(stores, vecs, height, exit)?;

                    reset_readers(
                        vecs,
                        &mut txindex_to_first_outputindex_reader_opt,
                        &mut p2pk65addressindex_to_p2pk65bytes_reader_opt,
                        &mut p2pk33addressindex_to_p2pk33bytes_reader_opt,
                        &mut p2pkhaddressindex_to_p2pkhbytes_reader_opt,
                        &mut p2shaddressindex_to_p2shbytes_reader_opt,
                        &mut p2wpkhaddressindex_to_p2wpkhbytes_reader_opt,
                        &mut p2wshaddressindex_to_p2wshbytes_reader_opt,
                        &mut p2traddressindex_to_p2trbytes_reader_opt,
                        &mut p2aaddressindex_to_p2abytes_reader_opt,
                    );
                }

                Ok(())
            },
        )?;

        txindex_to_first_outputindex_reader_opt.take();
        p2pk65addressindex_to_p2pk65bytes_reader_opt.take();
        p2pk33addressindex_to_p2pk33bytes_reader_opt.take();
        p2pkhaddressindex_to_p2pkhbytes_reader_opt.take();
        p2shaddressindex_to_p2shbytes_reader_opt.take();
        p2wpkhaddressindex_to_p2wpkhbytes_reader_opt.take();
        p2wshaddressindex_to_p2wshbytes_reader_opt.take();
        p2traddressindex_to_p2trbytes_reader_opt.take();
        p2aaddressindex_to_p2abytes_reader_opt.take();

        if should_export(idxs.height, true) {
            export(stores, vecs, idxs.height, exit)?;
        }

        let i = Instant::now();
        db.punch_holes()?;
        info!("Punched holes in db in {}s", i.elapsed().as_secs());

        Ok(starting_indexes)
    }

    pub fn static_clone(&self) -> &'static Self {
        Box::leak(Box::new(self.clone()))
    }
}

#[derive(Debug)]
enum InputSource<'a> {
    PreviousBlock((Vin, TxIndex, OutputIndex)),
    SameBlock((&'a Transaction, TxIndex, &'a TxIn, Vin)),
}
