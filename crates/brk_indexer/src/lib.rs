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
    AddressBytes, AddressBytesHash, BlockHash, BlockHashPrefix, Height, InputIndex, OutputIndex,
    OutputType, OutputTypeIndex, Sats, Timestamp, TxIndex, Txid, TxidPrefix, Vin, Vout, setrlimit,
};
pub use brk_parser::*;

use bitcoin::{Transaction, TxIn, TxOut};
use brk_exit::Exit;
use brk_vec::{Compressed, DynamicVec, VecIterator};
use color_eyre::eyre::{ContextCompat, eyre};
use fjall::TransactionalKeyspace;
use log::{error, info};
use rayon::prelude::*;
mod indexes;
mod stores;
mod vecs;

pub use indexes::*;
pub use stores::*;
pub use vecs::*;

const SNAPSHOT_BLOCK_RANGE: usize = 1000;
const COLLISIONS_CHECKED_UP_TO: u32 = 893_000;

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
        outputs_dir: &Path,
        compressed: bool,
        check_collisions: bool,
    ) -> color_eyre::Result<Self> {
        setrlimit()?;
        Ok(Self {
            path: outputs_dir.to_owned(),
            vecs: None,
            stores: None,
            compressed: Compressed::from(compressed),
            check_collisions,
        })
    }

    pub fn import_vecs(&mut self) -> color_eyre::Result<()> {
        self.vecs = Some(Vecs::forced_import(
            &self.path.join("vecs/indexed"),
            self.compressed,
        )?);
        Ok(())
    }

    /// Do NOT import multiple times are things will break !!!
    /// Clone struct instead
    pub fn import_stores(&mut self) -> color_eyre::Result<()> {
        self.stores = Some(Stores::forced_import(&self.path.join("stores"))?);
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
                    .blockhashprefix_to_height
                    .get(&blockhash_prefix)?
                    .is_some_and(|prev_height| *prev_height != height)
                {
                    error!("BlockHash: {blockhash}");
                    return Err(eyre!("Collision, expect prefix to need be set yet"));
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

                let outputs = block
                    .txdata
                    .iter()
                    .enumerate()
                    .flat_map(|(index, tx)| {
                        tx.output
                            .iter()
                            .enumerate()
                            .map(move |(vout, txout)| (TxIndex::from(index), Vout::from(vout), txout, tx))
                    })
                    .collect::<Vec<_>>();

                let tx_len = block.txdata.len();
                let outputs_len = outputs.len();
                let inputs_len = inputs.len();

                let (
                    txid_prefix_to_txid_and_block_txindex_and_prev_txindex_join_handle,
                    input_source_vec_handle,
                    outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt_handle,
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
                                        if check_collisions && stores.txidprefix_to_txindex.needs(height) {
                                            // Should only find collisions for two txids (duplicates), see below
                                            stores.txidprefix_to_txindex.get(&txid_prefix)?.map(|v| *v)
                                        } else {
                                            None
                                        };

                                    Ok((txid_prefix, (tx, txid, TxIndex::from(index), prev_txindex_opt)))
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
                        let txindex_to_first_outputindex_mmap = vecs
                            .txindex_to_first_outputindex.vec().mmap().load();

                        inputs
                            .into_par_iter()
                            .enumerate()
                            .map(|(block_inputindex, (block_txindex, vin, txin, tx))| -> color_eyre::Result<(InputIndex, InputSource)> {
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

                                let outputindex = vecs.txindex_to_first_outputindex.get_or_read(prev_txindex, &txindex_to_first_outputindex_mmap)?
                                    .context("Expect outputindex to not be none")
                                    .inspect_err(|_| {
                                        dbg!(outpoint.txid, prev_txindex, vout);
                                    })?.into_inner()
                                    + vout;

                                Ok((inputindex, InputSource::PreviousBlock((
                                    vin,
                                    txindex,
                                    outputindex,
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

                    let outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt_handle = scope.spawn(|| {
                        let p2pk65index_to_p2pk65bytes_mmap = vecs
                            .p2pk65index_to_p2pk65bytes.vec().mmap().load();
                        let p2pk33index_to_p2pk33bytes_mmap = vecs.p2pk33index_to_p2pk33bytes.vec().mmap().load();
                        let p2pkhindex_to_p2pkhbytes_mmap = vecs.p2pkhindex_to_p2pkhbytes.vec().mmap().load();
                        let p2shindex_to_p2shbytes_mmap = vecs.p2shindex_to_p2shbytes.vec().mmap().load();
                        let p2wpkhindex_to_p2wpkhbytes_mmap = vecs.p2wpkhindex_to_p2wpkhbytes.vec().mmap().load();
                        let p2wshindex_to_p2wshbytes_mmap = vecs.p2wshindex_to_p2wshbytes.vec().mmap().load();
                        let p2trindex_to_p2trbytes_mmap = vecs.p2trindex_to_p2trbytes.vec().mmap().load();
                       let p2aindex_to_p2abytes_mmap = vecs.p2aindex_to_p2abytes.vec().mmap().load();

                        outputs
                            .into_par_iter()
                            .enumerate()
                            .map(
                                #[allow(clippy::type_complexity)]
                                |(block_outputindex, (block_txindex, vout, txout, tx))| -> color_eyre::Result<(
                                    OutputIndex,
                                    (
                                        &TxOut,
                                        TxIndex,
                                        Vout,
                                        OutputType,
                                        brk_core::Result<AddressBytes>,
                                        Option<OutputTypeIndex>,
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

                                    let outputtypeindex_opt = address_bytes_res.as_ref().ok().and_then(|addressbytes| {
                                        stores
                                            .addressbyteshash_to_outputtypeindex
                                            .get(&AddressBytesHash::from((addressbytes, outputtype)))
                                            .unwrap()
                                            .map(|v| *v)
                                            // Checking if not in the future
                                            .and_then(|outputtypeindex_local| {
                                                (outputtypeindex_local < idxs.outputtypeindex(outputtype)).then_some(outputtypeindex_local)
                                            })
                                    });

                                    if let Some(Some(outputtypeindex)) = check_collisions.then_some(outputtypeindex_opt) {
                                        let addressbytes = address_bytes_res.as_ref().unwrap();

                                        let prev_addressbytes_opt = match outputtype {
                                            OutputType::P2PK65 => vecs
                                                .p2pk65index_to_p2pk65bytes
                                                .get_or_read(outputtypeindex.into(), &p2pk65index_to_p2pk65bytes_mmap)?
                                                .map(|v| AddressBytes::from(v.into_inner())),
                                            OutputType::P2PK33 => vecs
                                                .p2pk33index_to_p2pk33bytes
                                                .get_or_read(outputtypeindex.into(), &p2pk33index_to_p2pk33bytes_mmap)?
                                                .map(|v| AddressBytes::from(v.into_inner())),
                                            OutputType::P2PKH => vecs
                                                .p2pkhindex_to_p2pkhbytes
                                                .get_or_read(outputtypeindex.into(), &p2pkhindex_to_p2pkhbytes_mmap)?
                                                .map(|v| AddressBytes::from(v.into_inner())),
                                            OutputType::P2SH => vecs
                                                .p2shindex_to_p2shbytes
                                                .get_or_read(outputtypeindex.into(), &p2shindex_to_p2shbytes_mmap)?
                                                .map(|v| AddressBytes::from(v.into_inner())),
                                            OutputType::P2WPKH => vecs
                                                .p2wpkhindex_to_p2wpkhbytes
                                                .get_or_read(outputtypeindex.into(), &p2wpkhindex_to_p2wpkhbytes_mmap)?
                                                .map(|v| AddressBytes::from(v.into_inner())),
                                            OutputType::P2WSH => vecs
                                                .p2wshindex_to_p2wshbytes
                                                .get_or_read(outputtypeindex.into(), &p2wshindex_to_p2wshbytes_mmap)?
                                                .map(|v| AddressBytes::from(v.into_inner())),
                                            OutputType::P2TR => vecs
                                                .p2trindex_to_p2trbytes
                                                .get_or_read(outputtypeindex.into(), &p2trindex_to_p2trbytes_mmap)?
                                                .map(|v| AddressBytes::from(v.into_inner())),
                                            OutputType::P2A => vecs
                                                .p2aindex_to_p2abytes
                                                .get_or_read(outputtypeindex.into(), &p2aindex_to_p2abytes_mmap)?
                                                .map(|v| AddressBytes::from(v.into_inner())),
                                            OutputType::Empty | OutputType::OpReturn | OutputType::P2MS | OutputType::Unknown => {
                                                unreachable!()
                                            }
                                        };
                                        let prev_addressbytes =
                                            prev_addressbytes_opt.as_ref().context("Expect to have addressbytes")?;

                                        if stores.addressbyteshash_to_outputtypeindex.needs(height)
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
                                                outputtypeindex,
                                                outputtypeindex,
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
                                            outputtypeindex_opt,
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
                        outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt_handle.join(),
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

                let outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt =
                    outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt_handle
                        .ok()
                        .context(
                            "Expect outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt_handle to join",
                        )??;

                let mut new_txindexvout_to_outputindex: BTreeMap<
                    (TxIndex, Vout),
                    OutputIndex,
                > = BTreeMap::new();

                let mut already_added_addressbyteshash: BTreeMap<AddressBytesHash, OutputTypeIndex> = BTreeMap::new();

                outputindex_to_txout_outputtype_addressbytes_res_addressindex_opt
                .into_iter()
                .try_for_each(
                    |(
                        outputindex,
                        (txout, txindex, vout, outputtype, addressbytes_res, outputtypeindex_opt, _tx),
                    )|
                     -> color_eyre::Result<()> {
                        let sats = Sats::from(txout.value);

                        if vout.is_zero() {
                            vecs.txindex_to_first_outputindex.push_if_needed(txindex, outputindex)?;
                        }

                        vecs.outputindex_to_value.push_if_needed(outputindex, sats)?;

                        vecs.outputindex_to_outputtype
                            .push_if_needed(outputindex, outputtype)?;

                        let mut addressbyteshash = None;

                        let outputtypeindex;

                        if let Some(outputtypeindex_local) = outputtypeindex_opt.or_else(|| {
                            addressbytes_res.as_ref().ok().and_then(|addressbytes| {
                                // Check if address was first seen before in this iterator
                                // Example: https://mempool.space/address/046a0765b5865641ce08dd39690aade26dfbf5511430ca428a3089261361cef170e3929a68aee3d8d4848b0c5111b0a37b82b86ad559fd2a745b44d8e8d9dfdc0c
                                addressbyteshash.replace(AddressBytesHash::from((addressbytes, outputtype)));
                                already_added_addressbyteshash
                                    .get(addressbyteshash.as_ref().unwrap())
                                    .cloned()
                            })
                        }) {
                            outputtypeindex = outputtypeindex_local;
                        } else {
                            outputtypeindex = match outputtype {
                                OutputType::P2PK65 => {
                                    idxs.p2pk65index.copy_then_increment()
                                },
                                OutputType::P2PK33 => {
                                    idxs.p2pk33index.copy_then_increment()
                                },
                                OutputType::P2PKH => {
                                    idxs.p2pkhindex.copy_then_increment()
                                },
                                OutputType::P2MS => {
                                    vecs.p2msindex_to_txindex.push_if_needed(idxs.p2msindex, txindex)?;
                                    idxs.p2msindex.copy_then_increment()
                                },
                                OutputType::P2SH => {
                                    idxs.p2shindex.copy_then_increment()
                                },
                                OutputType::OpReturn => {
                                    vecs.opreturnindex_to_txindex.push_if_needed(idxs.opreturnindex, txindex)?;
                                    idxs.opreturnindex.copy_then_increment()
                                },
                                OutputType::P2WPKH => {
                                    idxs.p2wpkhindex.copy_then_increment()
                                },
                                OutputType::P2WSH => {
                                    idxs.p2wshindex.copy_then_increment()
                                },
                                OutputType::P2TR => {
                                    idxs.p2trindex.copy_then_increment()
                                },
                                OutputType::P2A => {
                                    idxs.p2aindex.copy_then_increment()
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
                            };

                            if let Ok(addressbytes) = addressbytes_res {
                                let addressbyteshash = addressbyteshash.unwrap();

                                already_added_addressbyteshash
                                    .insert(addressbyteshash, outputtypeindex);

                                stores.addressbyteshash_to_outputtypeindex.insert_if_needed(
                                    addressbyteshash,
                                    outputtypeindex,
                                    height,
                                );

                                vecs.push_bytes_if_needed(outputtypeindex, addressbytes)?;
                            }
                        }

                        vecs.outputindex_to_outputtypeindex
                            .push_if_needed(outputindex, outputtypeindex)?;

                        new_txindexvout_to_outputindex
                            .insert((txindex, vout), outputindex);

                        Ok(())
                    },
                )?;

                drop(already_added_addressbyteshash);

                input_source_vec
                    .into_iter()
                    .map(
                        #[allow(clippy::type_complexity)]
                        |(inputindex, input_source)| -> color_eyre::Result<(
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
                                        .context("txid should be in same block").inspect_err(|_| {
                                            dbg!(&txid_prefix_to_txid_and_block_txindex_and_prev_txindex);
                                            // panic!();
                                        })?
                                        .2;
                                    let prev_txindex = idxs.txindex + block_txindex;

                                    let prev_outputindex = new_txindexvout_to_outputindex
                                        .remove(&(prev_txindex, vout))
                                        .context("should have found addressindex from same block")
                                        .inspect_err(|_| {
                                            dbg!(&new_txindexvout_to_outputindex, txin, prev_txindex, vout, txid);
                                        })?;

                                    Ok((inputindex, vin, txindex, prev_outputindex))
                                }
                            }
                        },
                    )
                    .try_for_each(|res| -> color_eyre::Result<()> {
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
                    .txindex_to_txid.iter();

                txid_prefix_to_txid_and_block_txindex_and_prev_txindex
                    .into_iter()
                    .try_for_each(
                        |(txid_prefix, (tx, txid, index, prev_txindex_opt))| -> color_eyre::Result<()> {
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
                                        dbg!(height, txindex, prev_txid, prev_txindex);
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
                        vecs.txindex_to_rawlocktime.push_if_needed(txindex, tx.lock_time.into())?;
                        vecs.txindex_to_base_size.push_if_needed(txindex, tx.base_size().into())?;
                        vecs.txindex_to_total_size.push_if_needed(txindex, tx.total_size().into())?;
                        vecs.txindex_to_is_explicitly_rbf.push_if_needed(txindex, tx.is_explicitly_rbf())?;
                        Ok(())
                    })?;

                idxs.txindex += TxIndex::from(tx_len);
                idxs.inputindex += InputIndex::from(inputs_len);
                idxs.outputindex += OutputIndex::from(outputs_len);

                export_if_needed(stores, vecs, height, false, exit)?;

                Ok(())
            },
        )?;

        export_if_needed(stores, vecs, idxs.height, true, exit)?;

        stores.rotate_memtables();

        Ok(starting_indexes)
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

    pub fn keyspace(&self) -> &TransactionalKeyspace {
        &self.stores().keyspace
    }
}

#[derive(Debug)]
enum InputSource<'a> {
    PreviousBlock((Vin, TxIndex, OutputIndex)),
    SameBlock((&'a Transaction, TxIndex, &'a TxIn, Vin)),
}
