use std::{cmp::Ordering, collections::BTreeMap, mem, path::Path, thread};

use brk_core::{
    AddressData, CheckedSub, DateIndex, Dollars, EmptyAddressData, Height, InputIndex, OutputIndex,
    OutputType, Result, Sats, StoredUsize, Version,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyVec, CollectableVec, Computation, EagerVec, Format, GenericStoredVec,
    StoredIndex, StoredVec, UnsafeSlice, VecIterator,
};
use log::info;
use rayon::prelude::*;

use crate::{
    BlockState, GroupedByAddressType, SupplyState, Transacted, stores::Stores, vecs::market,
};

use super::{
    Indexes, fetched,
    grouped::{ComputedValueVecsFromHeight, StorableVecGeneatorOptions},
    indexes, transactions,
};

mod address_cohort;
mod address_cohorts;
mod addresstype_to_typeindex_tree;
mod addresstype_to_typeindex_vec;
mod common;
mod r#trait;
mod utxo_cohort;
mod utxo_cohorts;
mod withaddressdatasource;

pub use addresstype_to_typeindex_tree::*;
pub use addresstype_to_typeindex_vec::*;
use r#trait::CohortVecs;
pub use withaddressdatasource::WithAddressDataSource;

const VERSION: Version = Version::new(5);

#[derive(Clone)]
pub struct Vecs {
    chain_state: StoredVec<Height, SupplyState>,

    pub height_to_unspendable_supply: EagerVec<Height, Sats>,
    pub indexes_to_unspendable_supply: ComputedValueVecsFromHeight,
    pub height_to_opreturn_supply: EagerVec<Height, Sats>,
    pub indexes_to_opreturn_supply: ComputedValueVecsFromHeight,
    // pub height_to_address_count: EagerVec<Height, StoredUsize>,
    // pub height_to_empty_address_count: EagerVec<Height, StoredUsize>,
    pub addresstype_to_height_to_address_count: GroupedByAddressType<EagerVec<Height, StoredUsize>>,
    pub addresstype_to_height_to_empty_address_count:
        GroupedByAddressType<EagerVec<Height, StoredUsize>>,
    pub utxo_vecs: utxo_cohorts::Vecs,
    pub address_vecs: address_cohorts::Vecs,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        version: Version,
        _computation: Computation,
        format: Format,
        fetched: Option<&fetched::Vecs>,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = fetched.is_some();

        let mut root_path = path.to_owned();
        root_path.pop();
        root_path.pop();
        let states_path = root_path.join("states");

        Ok(Self {
            chain_state: StoredVec::forced_import(
                &states_path,
                "chain",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,

            height_to_unspendable_supply: EagerVec::forced_import(
                path,
                "unspendable_supply",
                version + VERSION + Version::ZERO,
                format,
            )?,
            indexes_to_unspendable_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                "unspendable_supply",
                false,
                version + VERSION + Version::ZERO,
                format,
                StorableVecGeneatorOptions::default().add_last(),
                compute_dollars,
            )?,
            height_to_opreturn_supply: EagerVec::forced_import(
                path,
                "opreturn_supply",
                version + VERSION + Version::ZERO,
                format,
            )?,
            indexes_to_opreturn_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                "opreturn_supply",
                false,
                version + VERSION + Version::ZERO,
                format,
                StorableVecGeneatorOptions::default().add_last(),
                compute_dollars,
            )?,
            // height_to_address_count: EagerVec::forced_import(
            //     path,
            //     "address_count",
            //     version + VERSION + Version::ZERO,
            //     format,
            // )?,
            // height_to_empty_address_count: EagerVec::forced_import(
            //     path,
            //     "empty_address_count",
            //     version + VERSION + Version::ZERO,
            //     format,
            // )?,
            addresstype_to_height_to_address_count: GroupedByAddressType {
                p2pk65: EagerVec::forced_import(
                    path,
                    "p2pk65_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2pk33: EagerVec::forced_import(
                    path,
                    "p2pk33_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2pkh: EagerVec::forced_import(
                    path,
                    "p2pkh_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2sh: EagerVec::forced_import(
                    path,
                    "p2sh_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2wpkh: EagerVec::forced_import(
                    path,
                    "p2wpkh_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2wsh: EagerVec::forced_import(
                    path,
                    "p2wsh_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2tr: EagerVec::forced_import(
                    path,
                    "p2tr_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2a: EagerVec::forced_import(
                    path,
                    "p2a_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
            },
            addresstype_to_height_to_empty_address_count: GroupedByAddressType {
                p2pk65: EagerVec::forced_import(
                    path,
                    "p2pk65_empty_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2pk33: EagerVec::forced_import(
                    path,
                    "p2pk33_empty_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2pkh: EagerVec::forced_import(
                    path,
                    "p2pkh_empty_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2sh: EagerVec::forced_import(
                    path,
                    "p2sh_empty_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2wpkh: EagerVec::forced_import(
                    path,
                    "p2wpkh_empty_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2wsh: EagerVec::forced_import(
                    path,
                    "p2wsh_empty_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2tr: EagerVec::forced_import(
                    path,
                    "p2tr_empty_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
                p2a: EagerVec::forced_import(
                    path,
                    "p2a_empty_address_count",
                    version + VERSION + Version::ZERO,
                    format,
                )?,
            },
            utxo_vecs: utxo_cohorts::Vecs::forced_import(
                path,
                version,
                _computation,
                format,
                fetched,
                &states_path,
            )?,
            address_vecs: address_cohorts::Vecs::forced_import(
                path,
                version,
                _computation,
                format,
                fetched,
                &states_path,
            )?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        transactions: &transactions::Vecs,
        fetched: Option<&fetched::Vecs>,
        market: &market::Vecs,
        // Must take ownership as its indexes will be updated for this specific function
        starting_indexes: &mut Indexes,
        exit: &Exit,
        stores: &mut Stores,
    ) -> color_eyre::Result<()> {
        let height_to_first_outputindex = &indexer.vecs.height_to_first_outputindex;
        let height_to_first_inputindex = &indexer.vecs.height_to_first_inputindex;
        let height_to_output_count = transactions.indexes_to_output_count.height.unwrap_sum();
        let height_to_input_count = transactions.indexes_to_input_count.height.unwrap_sum();
        let inputindex_to_outputindex = &indexer.vecs.inputindex_to_outputindex;
        let outputindex_to_value = &indexer.vecs.outputindex_to_value;
        let txindex_to_height = &indexes.txindex_to_height;
        let height_to_timestamp_fixed = &indexes.height_to_timestamp_fixed;
        let outputindex_to_txindex = &indexes.outputindex_to_txindex;
        let outputindex_to_outputtype = &indexer.vecs.outputindex_to_outputtype;
        let outputindex_to_typeindex = &indexer.vecs.outputindex_to_typeindex;
        let height_to_unclaimed_rewards = transactions
            .indexes_to_unclaimed_rewards
            .sats
            .height
            .as_ref()
            .unwrap();
        let height_to_close = fetched
            .as_ref()
            .map(|fetched| &fetched.chainindexes_to_close.height);
        let dateindex_to_close = fetched
            .as_ref()
            .map(|fetched| fetched.timeindexes_to_close.dateindex.as_ref().unwrap());
        let height_to_date_fixed = &indexes.height_to_date_fixed;
        let dateindex_to_first_height = &indexes.dateindex_to_first_height;
        let dateindex_to_height_count = &indexes.dateindex_to_height_count;

        let inputindex_to_outputindex_mmap = inputindex_to_outputindex.mmap().load();
        let outputindex_to_value_mmap = outputindex_to_value.mmap().load();
        let outputindex_to_outputtype_mmap = outputindex_to_outputtype.mmap().load();
        let outputindex_to_typeindex_mmap = outputindex_to_typeindex.mmap().load();
        let outputindex_to_txindex_mmap = outputindex_to_txindex.mmap().load();
        let txindex_to_height_mmap = txindex_to_height.mmap().load();
        let height_to_close_mmap = height_to_close.map(|v| v.mmap().load());
        let height_to_timestamp_fixed_mmap = height_to_timestamp_fixed.mmap().load();

        let mut height_to_first_outputindex_iter = height_to_first_outputindex.into_iter();
        let mut height_to_first_inputindex_iter = height_to_first_inputindex.into_iter();
        let mut height_to_output_count_iter = height_to_output_count.into_iter();
        let mut height_to_input_count_iter = height_to_input_count.into_iter();
        let mut height_to_close_iter = height_to_close.as_ref().map(|v| v.into_iter());
        let mut height_to_unclaimed_rewards_iter = height_to_unclaimed_rewards.into_iter();
        let mut height_to_timestamp_fixed_iter = height_to_timestamp_fixed.into_iter();
        let mut dateindex_to_close_iter = dateindex_to_close.as_ref().map(|v| v.into_iter());
        let mut height_to_date_fixed_iter = height_to_date_fixed.into_iter();
        let mut dateindex_to_first_height_iter = dateindex_to_first_height.into_iter();
        let mut dateindex_to_height_count_iter = dateindex_to_height_count.into_iter();

        let base_version = Version::ZERO
            + height_to_first_outputindex.version()
            + height_to_first_inputindex.version()
            + height_to_timestamp_fixed.version()
            + height_to_output_count.version()
            + height_to_input_count.version()
            + inputindex_to_outputindex.version()
            + outputindex_to_value.version()
            + txindex_to_height.version()
            + outputindex_to_txindex.version()
            + outputindex_to_outputtype.version()
            + outputindex_to_typeindex.version()
            + height_to_unclaimed_rewards.version()
            + height_to_close
                .as_ref()
                .map_or(Version::ZERO, |v| v.version())
            + dateindex_to_close
                .as_ref()
                .map_or(Version::ZERO, |v| v.version())
            + height_to_date_fixed.version()
            + dateindex_to_first_height.version()
            + dateindex_to_height_count.version()
            + stores.as_slice().into_iter().map(|s| s.version()).sum();

        let mut separate_utxo_vecs = self.utxo_vecs.as_mut_separate_vecs();
        let mut separate_address_vecs = self.address_vecs.as_mut_separate_vecs();

        separate_utxo_vecs
            .par_iter_mut()
            .try_for_each(|(_, v)| v.validate_computed_versions(base_version))?;
        separate_address_vecs
            .par_iter_mut()
            .try_for_each(|(_, v)| v.validate_computed_versions(base_version))?;
        self.height_to_unspendable_supply
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_unspendable_supply.inner_version(),
            )?;
        self.height_to_opreturn_supply
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_opreturn_supply.inner_version(),
            )?;

        let mut chain_state: Vec<BlockState> = vec![];
        let mut chain_state_starting_height = Height::from(self.chain_state.len());

        let stateful_starting_height = match separate_utxo_vecs
            .par_iter_mut()
            .map(|(_, v)| v.starting_height())
            .min()
            .unwrap_or_default()
            .min(
                separate_address_vecs
                    .par_iter_mut()
                    .map(|(_, v)| v.starting_height())
                    .min()
                    .unwrap_or_default(),
            )
            .min(chain_state_starting_height)
            .min(stores.starting_height())
            .min(Height::from(self.height_to_unspendable_supply.len()))
            .min(Height::from(self.height_to_opreturn_supply.len()))
            .cmp(&chain_state_starting_height)
        {
            Ordering::Greater => unreachable!(),
            Ordering::Equal => {
                chain_state = self
                    .chain_state
                    .collect_range(None, None)?
                    .into_iter()
                    .enumerate()
                    .map(|(height, supply)| {
                        let height = Height::from(height);
                        let timestamp = height_to_timestamp_fixed_iter.unwrap_get_inner(height);
                        let price = height_to_close_iter
                            .as_mut()
                            .map(|i| *i.unwrap_get_inner(height));
                        BlockState {
                            timestamp,
                            price,
                            supply,
                        }
                    })
                    .collect::<Vec<_>>();
                chain_state_starting_height
            }
            Ordering::Less => Height::ZERO,
        };

        let starting_height = starting_indexes.height.min(stateful_starting_height);

        if starting_height.is_zero() {
            info!("Starting processing utxos from the start");

            // todo!("rollback instead");
            chain_state = vec![];
            chain_state_starting_height = Height::ZERO;

            stores.reset()?;

            separate_utxo_vecs
                .par_iter_mut()
                .try_for_each(|(_, v)| v.state.price_to_amount.reset())?;

            separate_address_vecs
                .par_iter_mut()
                .try_for_each(|(_, v)| v.state.inner.price_to_amount.reset())?;
        }

        if starting_height < Height::from(height_to_date_fixed.len()) {
            starting_indexes.update_from_height(starting_height, indexes);

            separate_utxo_vecs
                .par_iter_mut()
                .for_each(|(_, v)| v.init(starting_height));

            separate_address_vecs
                .par_iter_mut()
                .for_each(|(_, v)| v.init(starting_height));

            let mut unspendable_supply = if let Some(prev_height) = starting_height.decremented() {
                self.height_to_unspendable_supply
                    .into_iter()
                    .unwrap_get_inner(prev_height)
            } else {
                Sats::ZERO
            };
            let mut opreturn_supply = if let Some(prev_height) = starting_height.decremented() {
                self.height_to_opreturn_supply
                    .into_iter()
                    .unwrap_get_inner(prev_height)
            } else {
                Sats::ZERO
            };
            let mut addresstype_to_address_count = GroupedByAddressType::<usize>::from((
                &self.addresstype_to_height_to_address_count,
                starting_height,
            ));
            let mut addresstype_to_empty_address_count = GroupedByAddressType::<usize>::from((
                &self.addresstype_to_height_to_empty_address_count,
                starting_height,
            ));

            let mut height = starting_height;

            let mut addresstype_to_typeindex_to_sent_outputindex =
                AddressTypeToTypeIndexVec::<OutputIndex>::default();
            let mut addresstype_to_typeindex_to_received_outputindex =
                AddressTypeToTypeIndexVec::<OutputIndex>::default();
            let mut addresstype_to_typeindex_to_addressdata =
                AddressTypeToTypeIndexTree::<WithAddressDataSource<AddressData>>::default();
            let mut addresstype_to_typeindex_to_emptyaddressdata =
                AddressTypeToTypeIndexTree::<WithAddressDataSource<EmptyAddressData>>::default();

            (height.unwrap_to_usize()..height_to_date_fixed.len())
                .map(Height::from)
                .try_for_each(|_height| -> color_eyre::Result<()> {
                    height = _height;

                    self.utxo_vecs
                        .as_mut_separate_vecs()
                        .iter_mut()
                        .for_each(|(_, v)| v.state.reset_single_iteration_values());

                    self.address_vecs
                        .as_mut_separate_vecs()
                        .iter_mut()
                        .for_each(|(_, v)| v.state.reset_single_iteration_values());

                    info!("Processing chain at {height}...");

                    let timestamp = height_to_timestamp_fixed_iter.unwrap_get_inner(height);
                    let price = height_to_close_iter
                        .as_mut()
                        .map(|i| *i.unwrap_get_inner(height));
                    let first_outputindex = height_to_first_outputindex_iter
                        .unwrap_get_inner(height)
                        .unwrap_to_usize();
                    let first_inputindex = height_to_first_inputindex_iter
                        .unwrap_get_inner(height)
                        .unwrap_to_usize();
                    let output_count = height_to_output_count_iter.unwrap_get_inner(height);
                    let input_count = height_to_input_count_iter.unwrap_get_inner(height);

                    let ((mut height_to_sent, new_addresstype_to_typedindex_to_sent_outputindex, addresstype_to_typedindex_to_sent_data), (mut received, new_addresstype_to_typedindex_to_received_outputindex, addresstype_to_typedindex_to_received_data)) = thread::scope(|s| {
                        if chain_state_starting_height <= height {
                            s.spawn(|| {
                                self.utxo_vecs
                                    .tick_tock_next_block(&chain_state, timestamp);
                            });
                        }

                        let sent_handle = s.spawn(|| {
                            // Skip coinbase
                            (first_inputindex + 1..first_inputindex + *input_count)
                                .into_par_iter()
                                .map(InputIndex::from)
                                .map(|inputindex| {
                                    let outputindex = inputindex_to_outputindex
                                        .get_or_read(inputindex, &inputindex_to_outputindex_mmap)
                                        .unwrap()
                                        .unwrap()
                                        .into_owned();

                                    let value = outputindex_to_value
                                        .get_or_read(outputindex, &outputindex_to_value_mmap)
                                        .unwrap()
                                        .unwrap()
                                        .into_owned();

                                    let input_type = outputindex_to_outputtype
                                        .get_or_read(outputindex, &outputindex_to_outputtype_mmap)
                                        .unwrap()
                                        .unwrap()
                                        .into_owned();

                                    let typeindex = outputindex_to_typeindex
                                        .get_or_read(outputindex, &outputindex_to_typeindex_mmap)
                                        .unwrap()
                                        .unwrap()
                                        .into_owned();

                                    if input_type.is_unspendable() {
                                        unreachable!()
                                    }

                                    let addressdata_opt = if input_type.is_address() && !addresstype_to_typeindex_to_addressdata.get(input_type).unwrap().contains_key(&typeindex) {
                                        Some(WithAddressDataSource::FromAddressDataStore( stores.get_addressdata(input_type, typeindex).unwrap().unwrap()))
                                    } else {
                                        None
                                    };

                                    let input_txindex = outputindex_to_txindex
                                        .get_or_read(outputindex, &outputindex_to_txindex_mmap)
                                        .unwrap()
                                        .unwrap()
                                        .into_owned();

                                    let prev_height = txindex_to_height
                                        .get_or_read(input_txindex, &txindex_to_height_mmap)
                                        .unwrap()
                                        .unwrap()
                                        .into_owned();

                                    let prev_price = height_to_close.map(|m| *m.get_or_read(prev_height, height_to_close_mmap.as_ref().unwrap()).unwrap()
                                    .unwrap()
                                    .into_owned());

                                    let prev_timestamp = height_to_timestamp_fixed.get_or_read(prev_height, &height_to_timestamp_fixed_mmap)
                                    .unwrap()
                                    .unwrap()
                                    .into_owned();

                                    let blocks_old = height.unwrap_to_usize() - prev_height.unwrap_to_usize();

                                    let days_old = prev_timestamp
                                        .difference_in_days_between_float(timestamp);

                                    let older_than_hour = timestamp
                                        .checked_sub(prev_timestamp)
                                        .unwrap()
                                        .is_more_than_hour();

                                    (prev_height, value, input_type, typeindex, outputindex, addressdata_opt, prev_price, blocks_old, days_old, older_than_hour)
                                })
                                .fold(
                                    || {
                                        (
                                            BTreeMap::<Height, Transacted>::default(),
                                            AddressTypeToTypeIndexVec::<OutputIndex>::default(),
                                            AddressTypeToTypeIndexVec::<(Sats, Option<WithAddressDataSource<AddressData>>, Option<Dollars>, usize, f64, bool)>::default(),
                                        )
                                    },
                                    |(mut tree, mut vecs, mut vecs2), (height, value, input_type, typeindex, outputindex, addressdata_opt, prev_price, blocks_old, days_old, older_than_hour)| {
                                        tree.entry(height).or_default().iterate(value, input_type);
                                        if let Some(vec) = vecs.get_mut(input_type) {
                                            vec.push((typeindex, outputindex));
                                        }
                                        if let Some(vec) = vecs2.get_mut(input_type) {
                                            vec.push((typeindex, (value, addressdata_opt, prev_price, blocks_old, days_old, older_than_hour)));
                                        }
                                        (tree, vecs, vecs2)
                                    },
                                )
                                .reduce( || {
                                    (
                                        BTreeMap::<Height, Transacted>::default(),
                                        AddressTypeToTypeIndexVec::<OutputIndex>::default(),
                                        AddressTypeToTypeIndexVec::<(Sats, Option<WithAddressDataSource<AddressData>>, Option<Dollars>, usize, f64, bool)>::default(),
                                    )
                                }, |(first_tree, mut source_vecs,mut source_vecs2), (second_tree, other_vecs, other_vecs2)| {
                                    let (mut tree_source, tree_to_consume) = if first_tree.len() > second_tree.len() {
                                        (first_tree, second_tree)
                                    } else {
                                        (second_tree, first_tree)
                                    };
                                    tree_to_consume.into_iter().for_each(|(k, v)| {
                                        *tree_source.entry(k).or_default() += v;
                                    });
                                    source_vecs.merge(other_vecs);
                                    source_vecs2.merge(other_vecs2);
                                    (tree_source, source_vecs, source_vecs2)
                                })
                        });

                        let received_handle = s.spawn(|| {
                            (first_outputindex..first_outputindex + *output_count)
                                .into_par_iter()
                                .map(OutputIndex::from)
                                .map(|outputindex| {
                                    let value = outputindex_to_value
                                        .get_or_read(outputindex, &outputindex_to_value_mmap)
                                        .unwrap()
                                        .unwrap()
                                        .into_owned();

                                    let output_type = outputindex_to_outputtype
                                        .get_or_read(outputindex, &outputindex_to_outputtype_mmap)
                                        .unwrap()
                                        .unwrap()
                                        .into_owned();

                                    let typeindex = outputindex_to_typeindex
                                        .get_or_read(outputindex, &outputindex_to_typeindex_mmap)
                                        .unwrap()
                                        .unwrap()
                                        .into_owned();

                                    let addressdata_opt = if output_type.is_address() && !addresstype_to_typeindex_to_addressdata.get(output_type).unwrap().contains_key(&typeindex) && !addresstype_to_typeindex_to_emptyaddressdata.get(output_type).unwrap().contains_key(&typeindex) {
                                        Some(if let Some(addressdata) = stores.get_addressdata(output_type, typeindex).unwrap() {
                                            WithAddressDataSource::FromAddressDataStore(addressdata)
                                        } else if let Some(emptyaddressdata) = stores.get_emptyaddressdata(output_type, typeindex).unwrap() {
                                            WithAddressDataSource::FromEmptyAddressDataStore(emptyaddressdata.into())
                                        } else {
                                            WithAddressDataSource::New(AddressData::default())
                                        })
                                    } else {
                                        None
                                    };

                                    (value, output_type, typeindex, outputindex, addressdata_opt)
                                })
                                .fold(
                                    || (Transacted::default(), AddressTypeToTypeIndexVec::<OutputIndex>::default(),
                                    AddressTypeToTypeIndexVec::<(Sats, Option<WithAddressDataSource<AddressData>>)>::default(),
                                    ),
                                    |(mut transacted, mut vecs, mut vecs2), (value, output_type, typeindex, outputindex, addressdata_opt)| {
                                        transacted.iterate(value, output_type);
                                        if let Some(vec) = vecs.get_mut(output_type) {
                                            vec.push((typeindex, outputindex));
                                        }
                                        if let Some(vec) = vecs2.get_mut(output_type) {
                                            vec.push((typeindex, (value, addressdata_opt)));
                                        }
                                        (transacted, vecs, vecs2)
                                    },
                                )
                                .reduce(|| (Transacted::default(), AddressTypeToTypeIndexVec::<OutputIndex>::default(),
                                AddressTypeToTypeIndexVec::<(Sats, Option<WithAddressDataSource<AddressData>>)>::default(),
                                ), |(transacted, mut vecs, mut vecs2), (other_transacted, other_vecs, other_vecs2)| {
                                    vecs.merge(other_vecs);
                                    vecs2.merge(other_vecs2);
                                    (transacted + other_transacted, vecs, vecs2)
                                })
                        });

                        (sent_handle.join().unwrap(), received_handle.join().unwrap())
                    });

                    addresstype_to_typeindex_to_sent_outputindex.merge(new_addresstype_to_typedindex_to_sent_outputindex);
                    addresstype_to_typeindex_to_received_outputindex.merge(new_addresstype_to_typedindex_to_received_outputindex);

                    addresstype_to_typedindex_to_received_data.process_received(
                        &mut self.address_vecs,
                        &mut addresstype_to_typeindex_to_addressdata,
                        &mut addresstype_to_typeindex_to_emptyaddressdata,
                        price,
                        &mut addresstype_to_address_count,
                        &mut addresstype_to_empty_address_count
                    );

                    addresstype_to_typedindex_to_sent_data.process_sent(
                        &mut self.address_vecs,
                        &mut addresstype_to_typeindex_to_addressdata,
                        &mut addresstype_to_typeindex_to_emptyaddressdata,
                        price,
                        &mut addresstype_to_address_count,
                        &mut addresstype_to_empty_address_count
                    )?;

                    unspendable_supply += received
                        .by_type
                        .unspendable
                        .as_vec()
                        .into_iter()
                        .map(|state| state.value)
                        .sum::<Sats>()
                        + height_to_unclaimed_rewards_iter.unwrap_get_inner(height);

                    opreturn_supply += received.by_type.unspendable.opreturn.value;

                    if height == Height::new(0) {
                        received = Transacted::default();
                        unspendable_supply += Sats::FIFTY_BTC;
                    } else if height == Height::new(91_842) || height == Height::new(91_880) {
                        // Need to destroy invalid coinbases due to duplicate txids
                        if height == Height::new(91_842) {
                            height_to_sent.entry(Height::new(91_812)).or_default()
                        } else {
                            height_to_sent.entry(Height::new(91_722)).or_default()
                        }
                        .iterate(Sats::FIFTY_BTC, OutputType::P2PK65);
                    };

                    if chain_state_starting_height <= height {
                        // Push current block state before processing sends and receives
                        chain_state.push(BlockState {
                            supply: received.spendable_supply.clone(),
                            price,
                            timestamp,
                        });

                        self.utxo_vecs.receive(received, height, price);

                        let unsafe_chain_state = UnsafeSlice::new(&mut chain_state);

                        height_to_sent.par_iter().for_each(|(height, sent)| unsafe {
                            (*unsafe_chain_state.get(height.unwrap_to_usize())).supply -=
                                &sent.spendable_supply;
                        });

                        self.utxo_vecs.send(height_to_sent, chain_state.as_slice());
                    } else {
                        dbg!(chain_state_starting_height, height);
                        panic!("temp, just making sure")
                    }

                    let mut separate_utxo_vecs = self.utxo_vecs.as_mut_separate_vecs();

                    separate_utxo_vecs
                        .iter_mut()
                        .try_for_each(|(_, v)| v.forced_pushed_at(height, exit))?;

                    let mut separate_address_vecs = self.address_vecs.as_mut_separate_vecs();

                    separate_address_vecs
                        .iter_mut()
                        .try_for_each(|(_, v)| v.forced_pushed_at(height, exit))?;

                    self.height_to_unspendable_supply.forced_push_at(
                        height,
                        unspendable_supply,
                        exit,
                    )?;

                    self.height_to_opreturn_supply
                        .forced_push_at(height, opreturn_supply, exit)?;

                    self.addresstype_to_height_to_address_count.forced_push_at(height, &addresstype_to_address_count, exit)?;

                    self.addresstype_to_height_to_empty_address_count.forced_push_at(height, &addresstype_to_empty_address_count, exit)?;

                    let date = height_to_date_fixed_iter.unwrap_get_inner(height);
                    let dateindex = DateIndex::try_from(date).unwrap();
                    let date_first_height =
                        dateindex_to_first_height_iter.unwrap_get_inner(dateindex);
                    let date_height_count =
                        dateindex_to_height_count_iter.unwrap_get_inner(dateindex);
                    let is_date_last_height = date_first_height
                        + Height::from(date_height_count).decremented().unwrap()
                        == height;
                    let date_price = dateindex_to_close_iter
                        .as_mut()
                        .map(|v| is_date_last_height.then(|| *v.unwrap_get_inner(dateindex)));

                    // thread::scope(|scope| {
                    //     scope.spawn(|| {
                    separate_utxo_vecs.par_iter_mut().try_for_each(|(_, v)| {
                        v.compute_then_force_push_unrealized_states(
                            height,
                            price,
                            is_date_last_height.then_some(dateindex),
                            date_price,
                            exit,
                        )
                    })?;
                    // });
                    // scope.spawn(|| {
                    separate_address_vecs.par_iter_mut().try_for_each(|(_, v)| {
                        v.compute_then_force_push_unrealized_states(
                            height,
                            price,
                            is_date_last_height.then_some(dateindex),
                            date_price,
                            exit,
                        )
                    })?;
                    //     });
                    // });


                    if height != Height::ZERO && height.unwrap_to_usize() % 10_000 == 0 {
                        info!("Flushing...");
                        exit.block();
                        self.flush_states(
                            height,
                            &chain_state,
                            exit,
                        )?;
                        stores.commit(
                            height,
                            mem::take(&mut addresstype_to_typeindex_to_sent_outputindex),
                            mem::take(&mut addresstype_to_typeindex_to_received_outputindex),
                            mem::take(&mut addresstype_to_typeindex_to_addressdata), mem::take(&mut addresstype_to_typeindex_to_emptyaddressdata)
                        )?;
                        exit.release();
                    }

                    Ok(())
                })?;

            exit.block();

            info!("Flushing...");

            self.flush_states(height, &chain_state, exit)?;
            stores.commit(
                height,
                mem::take(&mut addresstype_to_typeindex_to_sent_outputindex),
                mem::take(&mut addresstype_to_typeindex_to_received_outputindex),
                mem::take(&mut addresstype_to_typeindex_to_addressdata),
                mem::take(&mut addresstype_to_typeindex_to_emptyaddressdata),
            )?;
        } else {
            exit.block();
        }

        info!("Computing overlapping...");

        // thread::scope(|scope| {
        //     scope.spawn(|| {
        self.utxo_vecs
            .compute_overlapping_vecs(starting_indexes, exit)?;
        // });
        // scope.spawn(|| {
        self.address_vecs
            .compute_overlapping_vecs(starting_indexes, exit)?;
        //     });
        // });

        info!("Computing rest part 1...");

        // thread::scope(|scope| {
        //     scope.spawn(|| {
        self.utxo_vecs
            .as_mut_vecs()
            .par_iter_mut()
            .try_for_each(|(_, v)| {
                v.compute_rest_part1(indexer, indexes, fetched, starting_indexes, exit)
            })
            .unwrap();
        // });
        // scope.spawn(|| {
        self.address_vecs
            .as_mut_vecs()
            .par_iter_mut()
            .try_for_each(|(_, v)| {
                v.compute_rest_part1(indexer, indexes, fetched, starting_indexes, exit)
            })
            .unwrap();
        //     });
        // });

        info!("Computing rest part 2...");

        let height_to_supply = self.utxo_vecs.all.1.height_to_supply_value.bitcoin.clone();
        let dateindex_to_supply = self
            .utxo_vecs
            .all
            .1
            .indexes_to_supply
            .bitcoin
            .dateindex
            .clone();
        let height_to_realized_cap = self.utxo_vecs.all.1.height_to_realized_cap.clone();
        let dateindex_to_realized_cap = self
            .utxo_vecs
            .all
            .1
            .indexes_to_realized_cap
            .as_ref()
            .map(|v| v.dateindex.unwrap_last().clone());

        // thread::scope(|scope| {
        //     scope.spawn(|| {
        self.utxo_vecs
            .as_mut_vecs()
            .par_iter_mut()
            .try_for_each(|(_, v)| {
                v.compute_rest_part2(
                    indexer,
                    indexes,
                    fetched,
                    starting_indexes,
                    market,
                    &height_to_supply,
                    dateindex_to_supply.as_ref().unwrap(),
                    height_to_realized_cap.as_ref(),
                    dateindex_to_realized_cap.as_ref(),
                    exit,
                )
            })?;
        // });
        // scope.spawn(|| {
        self.address_vecs
            .as_mut_vecs()
            .par_iter_mut()
            .try_for_each(|(_, v)| {
                v.compute_rest_part2(
                    indexer,
                    indexes,
                    fetched,
                    starting_indexes,
                    market,
                    &height_to_supply,
                    dateindex_to_supply.as_ref().unwrap(),
                    height_to_realized_cap.as_ref(),
                    dateindex_to_realized_cap.as_ref(),
                    exit,
                )
            })?;
        //     });
        // });
        self.indexes_to_unspendable_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            Some(&self.height_to_unspendable_supply),
        )?;
        self.indexes_to_opreturn_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            Some(&self.height_to_opreturn_supply),
        )?;

        stores.rotate_memtables();

        exit.release();

        Ok(())
    }

    fn flush_states(
        &mut self,
        height: Height,
        chain_state: &[BlockState],
        exit: &Exit,
    ) -> Result<()> {
        self.utxo_vecs
            .as_mut_separate_vecs()
            .par_iter_mut()
            .try_for_each(|(_, v)| v.safe_flush_stateful_vecs(height, exit))?;
        self.address_vecs
            .as_mut_separate_vecs()
            .par_iter_mut()
            .try_for_each(|(_, v)| v.safe_flush_stateful_vecs(height, exit))?;
        self.height_to_unspendable_supply.safe_flush(exit)?;
        self.height_to_opreturn_supply.safe_flush(exit)?;
        self.addresstype_to_height_to_address_count
            .as_mut_vec()
            .into_iter()
            .try_for_each(|v| v.safe_flush(exit))?;
        self.addresstype_to_height_to_empty_address_count
            .as_mut_vec()
            .into_iter()
            .try_for_each(|v| v.safe_flush(exit))?;

        self.chain_state.truncate_if_needed(Height::ZERO)?;
        chain_state.iter().for_each(|block_state| {
            self.chain_state.push(block_state.supply.clone());
        });
        self.chain_state.flush()?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.utxo_vecs
                .vecs()
                .into_iter()
                .flat_map(|v| v.vecs())
                .collect::<Vec<_>>(),
            self.address_vecs
                .vecs()
                .into_iter()
                .flat_map(|v| v.vecs())
                .collect::<Vec<_>>(),
            self.indexes_to_unspendable_supply.vecs(),
            self.indexes_to_opreturn_supply.vecs(),
            self.addresstype_to_height_to_address_count
                .as_typed_vec()
                .into_iter()
                .map(|(_, v)| v as &dyn AnyCollectableVec)
                .collect::<Vec<_>>(),
            self.addresstype_to_height_to_empty_address_count
                .as_typed_vec()
                .into_iter()
                .map(|(_, v)| v as &dyn AnyCollectableVec)
                .collect::<Vec<_>>(),
            vec![
                &self.height_to_unspendable_supply,
                &self.height_to_opreturn_supply,
                // &self.height_to_address_count,
                // &self.height_to_empty_address_count,
            ],
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}

impl AddressTypeToTypeIndexVec<(Sats, Option<WithAddressDataSource<AddressData>>)> {
    fn process_received(
        mut self,
        vecs: &mut address_cohorts::Vecs,
        addresstype_to_typeindex_to_addressdata: &mut AddressTypeToTypeIndexTree<
            WithAddressDataSource<AddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &mut AddressTypeToTypeIndexTree<
            WithAddressDataSource<EmptyAddressData>,
        >,
        price: Option<Dollars>,
        addresstype_to_address_count: &mut GroupedByAddressType<usize>,
        addresstype_to_empty_address_count: &mut GroupedByAddressType<usize>,
    ) {
        self.into_typed_vec().into_iter().for_each(|(_type, vec)| {
            vec.into_iter()
                .for_each(|(type_index, (value, addressdata_opt))| {
                    let mut is_new = false;
                    let mut from_any_empty = false;

                    let addressdata_withsource = addresstype_to_typeindex_to_addressdata
                        .get_mut(_type)
                        .unwrap()
                        .entry(type_index)
                        .or_insert_with(|| {
                            addresstype_to_typeindex_to_emptyaddressdata
                                .get_mut(_type)
                                .unwrap()
                                .remove(&type_index)
                                .map(|ad| {
                                    from_any_empty = true;
                                    ad.into()
                                })
                                .unwrap_or_else(|| {
                                    let addressdata = addressdata_opt.unwrap();
                                    is_new = addressdata.is_new();
                                    from_any_empty = addressdata.is_from_emptyaddressdata();
                                    addressdata
                                })
                        });

                    if is_new || from_any_empty {
                        (*addresstype_to_address_count.get_mut(_type).unwrap()) += 1;
                        if from_any_empty {
                            (*addresstype_to_empty_address_count.get_mut(_type).unwrap()) -= 1;
                        }
                    }

                    let addressdata = addressdata_withsource.deref_mut();

                    let prev_amount = addressdata.amount();

                    let amount = prev_amount + value;

                    if is_new
                        || vecs.by_size_range.get_mut(amount).0.clone()
                            != vecs.by_size_range.get_mut(prev_amount).0.clone()
                    {
                        if !is_new {
                            vecs.by_size_range
                                .get_mut(prev_amount)
                                .1
                                .state
                                .subtract(addressdata);
                        }

                        addressdata.receive(value, price);

                        vecs.by_size_range.get_mut(amount).1.state.add(addressdata);
                    } else {
                        addressdata.receive(value, price);

                        vecs.by_size_range
                            .get_mut(amount)
                            .1
                            .state
                            .receive(value, price);
                    }
                });
        });
    }
}

impl
    AddressTypeToTypeIndexVec<(
        Sats,
        Option<WithAddressDataSource<AddressData>>,
        Option<Dollars>,
        usize,
        f64,
        bool,
    )>
{
    fn process_sent(
        mut self,
        vecs: &mut address_cohorts::Vecs,
        addresstype_to_typeindex_to_addressdata: &mut AddressTypeToTypeIndexTree<
            WithAddressDataSource<AddressData>,
        >,
        addresstype_to_typeindex_to_emptyaddressdata: &mut AddressTypeToTypeIndexTree<
            WithAddressDataSource<EmptyAddressData>,
        >,
        price: Option<Dollars>,
        addresstype_to_address_count: &mut GroupedByAddressType<usize>,
        addresstype_to_empty_address_count: &mut GroupedByAddressType<usize>,
    ) -> Result<()> {
        self.into_typed_vec()
            .into_iter()
            .try_for_each(|(_type, vec)| {
                vec.into_iter().try_for_each(
                    |(
                        type_index,
                        (value, addressdata_opt, prev_price, blocks_old, days_old, older_than_hour),
                    )| {
                        let typeindex_to_addressdata = addresstype_to_typeindex_to_addressdata
                            .get_mut(_type)
                            .unwrap();

                        let addressdata_withsource = typeindex_to_addressdata
                            .entry(type_index)
                            .or_insert(addressdata_opt.unwrap());

                        let addressdata = addressdata_withsource.deref_mut();

                        let prev_amount = addressdata.amount();

                        let amount = prev_amount.checked_sub(value).unwrap();

                        let will_be_empty = addressdata.outputs_len - 1 == 0;

                        if will_be_empty
                            || vecs.by_size_range.get_mut(amount).0.clone()
                                != vecs.by_size_range.get_mut(prev_amount).0.clone()
                        {
                            vecs.by_size_range
                                .get_mut(prev_amount)
                                .1
                                .state
                                .subtract(addressdata);

                            addressdata.send(value, prev_price)?;

                            if will_be_empty {
                                (*addresstype_to_address_count.get_mut(_type).unwrap()) -= 1;
                                (*addresstype_to_empty_address_count.get_mut(_type).unwrap()) += 1;

                                let addressdata =
                                    typeindex_to_addressdata.remove(&type_index).unwrap();

                                addresstype_to_typeindex_to_emptyaddressdata
                                    .get_mut(_type)
                                    .unwrap()
                                    .insert(type_index, addressdata.into());
                            } else {
                                vecs.by_size_range.get_mut(amount).1.state.add(addressdata);
                            }
                        } else {
                            addressdata.send(value, prev_price)?;

                            vecs.by_size_range.get_mut(amount).1.state.send(
                                value,
                                price,
                                prev_price,
                                blocks_old,
                                days_old,
                                older_than_hour,
                            );
                        }

                        Ok(())
                    },
                )
            })
    }
}
