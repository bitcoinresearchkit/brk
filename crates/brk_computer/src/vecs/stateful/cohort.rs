use std::{fs, path::Path};

use brk_core::{
    CheckedSub, DateIndex, Dollars, Height, Result, Sats, StoredF32, StoredUsize, Version,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_state::{CohortState, RealizedState};
use brk_vec::{AnyCollectableVec, AnyVec, Compressed, Computation, EagerVec, VecIterator};

use crate::vecs::{
    Indexes, fetched,
    grouped::{
        ComputedRatioVecsFromDateIndex, ComputedValueVecsFromHeight, ComputedVecsFromHeight,
        StorableVecGeneatorOptions,
    },
    indexes,
};

const VERSION: Version = Version::ZERO;

pub struct Vecs {
    starting_height: Height,
    pub state: CohortState,

    // Cumulative
    pub height_to_realized_cap: Option<EagerVec<Height, Dollars>>,
    pub height_to_supply: EagerVec<Height, Sats>,
    pub height_to_utxo_count: EagerVec<Height, StoredUsize>,
    // Single
    pub height_to_realized_profit: Option<EagerVec<Height, Dollars>>,
    pub height_to_realized_loss: Option<EagerVec<Height, Dollars>>,
    pub height_to_value_created: Option<EagerVec<Height, Dollars>>,
    pub height_to_adjusted_value_created: Option<EagerVec<Height, Dollars>>,
    pub height_to_value_destroyed: Option<EagerVec<Height, Dollars>>,
    pub height_to_adjusted_value_destroyed: Option<EagerVec<Height, Dollars>>,

    pub dateindex_to_adjusted_spent_output_profit_ratio: Option<EagerVec<DateIndex, StoredF32>>,
    pub dateindex_to_realized_cap_30d_change: Option<EagerVec<DateIndex, Dollars>>,
    pub dateindex_to_sell_side_risk_ratio: Option<EagerVec<DateIndex, StoredF32>>,
    pub dateindex_to_spent_output_profit_ratio: Option<EagerVec<DateIndex, StoredF32>>,
    pub indexes_to_adjusted_value_created: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_adjusted_value_destroyed: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_negative_realized_loss: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_net_realized_profit_and_loss: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_cap: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_loss: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_price: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_price_extra: Option<ComputedRatioVecsFromDateIndex>,
    pub indexes_to_realized_profit: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_value: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_supply: ComputedValueVecsFromHeight,
    pub indexes_to_utxo_count: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_value_created: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_value_destroyed: Option<ComputedVecsFromHeight<Dollars>>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        cohort_name: Option<&str>,
        _computation: Computation,
        compressed: Compressed,
        version: Version,
        fetched: Option<&fetched::Vecs>,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = fetched.is_some();

        fs::create_dir_all(path)?;

        // let prefix = |s: &str| cohort_name.map_or(s.to_string(), |name| format!("{s}_{name}"));

        let suffix = |s: &str| cohort_name.map_or(s.to_string(), |name| format!("{name}_{s}"));

        let mut state = CohortState::default();
        if compute_dollars {
            state.realized = Some(RealizedState::NAN);
        }

        Ok(Self {
            starting_height: Height::ZERO,
            state,

            height_to_realized_cap: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("realized_cap"),
                    version + VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            indexes_to_realized_cap: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_cap"),
                    false,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_supply: EagerVec::forced_import(
                path,
                &suffix("supply"),
                version + VERSION + Version::ZERO,
                compressed,
            )?,
            indexes_to_supply: ComputedValueVecsFromHeight::forced_import(
                path,
                &suffix("supply"),
                false,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
                compute_dollars,
            )?,
            height_to_utxo_count: EagerVec::forced_import(
                path,
                &suffix("utxo_count"),
                version + VERSION + Version::ZERO,
                compressed,
            )?,
            indexes_to_utxo_count: ComputedVecsFromHeight::forced_import(
                path,
                &suffix("utxo_count"),
                false,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_realized_price: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_price"),
                    true,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            indexes_to_realized_price_extra: compute_dollars.then(|| {
                ComputedRatioVecsFromDateIndex::forced_import(
                    path,
                    &suffix("realized_price"),
                    false,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_last(),
                )
                .unwrap()
            }),
            height_to_realized_profit: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("realized_profit"),
                    version + VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            indexes_to_realized_profit: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_profit"),
                    false,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_realized_loss: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("realized_loss"),
                    version + VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            indexes_to_realized_loss: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_loss"),
                    false,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            indexes_to_negative_realized_loss: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("negative_realized_loss"),
                    true,
                    version + VERSION + Version::ONE,
                    compressed,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_value_created: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("value_created"),
                    version + VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            indexes_to_value_created: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("value_created"),
                    false,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            indexes_to_realized_value: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("realized_value"),
                    true,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_adjusted_value_created: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("adjusted_value_created"),
                    version + VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            indexes_to_adjusted_value_created: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("adjusted_value_created"),
                    false,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_value_destroyed: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("value_destroyed"),
                    version + VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            indexes_to_value_destroyed: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("value_destroyed"),
                    false,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            height_to_adjusted_value_destroyed: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("adjusted_value_destroyed"),
                    version + VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            indexes_to_adjusted_value_destroyed: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("adjusted_value_destroyed"),
                    false,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            dateindex_to_realized_cap_30d_change: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("realized_cap_30d_change"),
                    version + VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            indexes_to_net_realized_profit_and_loss: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    path,
                    &suffix("net_realized_profit_and_loss"),
                    true,
                    version + VERSION + Version::ZERO,
                    compressed,
                    StorableVecGeneatorOptions::default().add_sum(),
                )
                .unwrap()
            }),
            dateindex_to_sell_side_risk_ratio: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("sell_side_risk_ratio"),
                    version + VERSION + Version::ONE,
                    compressed,
                )
                .unwrap()
            }),
            dateindex_to_spent_output_profit_ratio: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("spent_output_profit_ratio"),
                    version + VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            dateindex_to_adjusted_spent_output_profit_ratio: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &suffix("adjusted_spent_output_profit_ratio"),
                    version + VERSION + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
        })
    }

    pub fn starting_height(&self) -> Height {
        [
            self.height_to_supply.len(),
            self.height_to_utxo_count.len(),
            self.height_to_realized_cap
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_realized_profit
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_realized_loss
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_value_created
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_adjusted_value_created
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_value_destroyed
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
            self.height_to_adjusted_value_destroyed
                .as_ref()
                .map_or(usize::MAX, |v| v.len()),
        ]
        .into_iter()
        .map(Height::from)
        .min()
        .unwrap()
    }

    pub fn init(&mut self, starting_height: Height) {
        if starting_height > self.starting_height() {
            unreachable!()
        }

        self.starting_height = starting_height;

        if let Some(prev_height) = starting_height.checked_sub(Height::new(1)) {
            self.state.supply.value = self
                .height_to_supply
                .into_iter()
                .unwrap_get_inner(prev_height);
            self.state.supply.utxos = *self
                .height_to_utxo_count
                .into_iter()
                .unwrap_get_inner(prev_height);

            if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
                self.state.realized.as_mut().unwrap().cap = height_to_realized_cap
                    .into_iter()
                    .unwrap_get_inner(prev_height);
            }
        }
    }

    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.height_to_supply
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_supply.inner_version(),
            )?;

        self.height_to_utxo_count
            .validate_computed_version_or_reset_file(
                base_version + self.height_to_utxo_count.inner_version(),
            )?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut().as_mut() {
            height_to_realized_cap.validate_computed_version_or_reset_file(
                base_version + height_to_realized_cap.inner_version(),
            )?;

            let height_to_realized_profit_inner_version = self
                .height_to_realized_profit
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_realized_profit
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + height_to_realized_profit_inner_version,
                )?;
            let height_to_realized_loss_inner_version = self
                .height_to_realized_loss
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_realized_loss
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + height_to_realized_loss_inner_version,
                )?;
            let height_to_value_created_inner_version = self
                .height_to_value_created
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_value_created
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + height_to_value_created_inner_version,
                )?;
            let height_to_adjusted_value_created_inner_version = self
                .height_to_adjusted_value_created
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_adjusted_value_created
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + height_to_adjusted_value_created_inner_version,
                )?;
            let height_to_value_destroyed_inner_version = self
                .height_to_value_destroyed
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_value_destroyed
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + height_to_value_destroyed_inner_version,
                )?;
            let height_to_adjusted_value_destroyed_inner_version = self
                .height_to_adjusted_value_destroyed
                .as_ref()
                .unwrap()
                .inner_version();
            self.height_to_adjusted_value_destroyed
                .as_mut()
                .unwrap()
                .validate_computed_version_or_reset_file(
                    base_version + height_to_adjusted_value_destroyed_inner_version,
                )?;
        }

        Ok(())
    }

    pub fn forced_pushed_at(&mut self, height: Height, exit: &Exit) -> Result<()> {
        if self.starting_height > height {
            return Ok(());
        }

        self.height_to_supply
            .forced_push_at(height, self.state.supply.value, exit)?;

        self.height_to_utxo_count.forced_push_at(
            height,
            StoredUsize::from(self.state.supply.utxos),
            exit,
        )?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
            let realized = self.state.realized.as_ref().unwrap_or_else(|| {
                dbg!((&self.state.realized, &self.state.supply));
                panic!();
            });

            height_to_realized_cap.forced_push_at(height, realized.cap, exit)?;

            self.height_to_realized_profit
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.profit, exit)?;
            self.height_to_realized_loss
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.loss, exit)?;
            self.height_to_value_created
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.value_created, exit)?;
            self.height_to_adjusted_value_created
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.adj_value_created, exit)?;
            self.height_to_value_destroyed
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.value_destroyed, exit)?;
            self.height_to_adjusted_value_destroyed
                .as_mut()
                .unwrap()
                .forced_push_at(height, realized.adj_value_destroyed, exit)?;
        }
        Ok(())
    }

    pub fn safe_flush_height_vecs(&mut self, exit: &Exit) -> Result<()> {
        self.height_to_supply.safe_flush(exit)?;

        self.height_to_utxo_count.safe_flush(exit)?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
            height_to_realized_cap.safe_flush(exit)?;
            self.height_to_realized_profit
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_realized_loss
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_value_created
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_adjusted_value_created
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_value_destroyed
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
            self.height_to_adjusted_value_destroyed
                .as_mut()
                .unwrap()
                .safe_flush(exit)?;
        }

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.indexes_to_supply.compute_rest(
            indexer,
            indexes,
            fetched,
            starting_indexes,
            exit,
            Some(&self.height_to_supply),
        )?;

        self.indexes_to_utxo_count.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_utxo_count),
        )?;

        if let Some(indexes_to_realized_cap) = self.indexes_to_realized_cap.as_mut() {
            indexes_to_realized_cap.compute_rest(
                indexes,
                starting_indexes,
                exit,
                Some(self.height_to_realized_cap.as_ref().unwrap()),
            )?;

            self.indexes_to_realized_price
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_divide(
                            starting_indexes.height,
                            self.height_to_realized_cap.as_ref().unwrap(),
                            &**self.indexes_to_supply.bitcoin.height.as_ref().unwrap(),
                            exit,
                        )
                    },
                )?;

            self.indexes_to_realized_price_extra
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexer,
                    indexes,
                    fetched.as_ref().unwrap(),
                    starting_indexes,
                    exit,
                    Some(
                        self.indexes_to_realized_price
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_last(),
                    ),
                )?;

            self.indexes_to_realized_profit
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_realized_profit.as_ref().unwrap()),
                )?;

            self.indexes_to_realized_loss
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_realized_loss.as_ref().unwrap()),
                )?;

            self.indexes_to_negative_realized_loss
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_transform(
                            starting_indexes.height,
                            self.height_to_realized_loss.as_ref().unwrap(),
                            |(i, v, ..)| (i, v * -1_i64),
                            exit,
                        )
                    },
                )?;

            self.indexes_to_value_created
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_value_created.as_ref().unwrap()),
                )?;

            self.indexes_to_adjusted_value_created
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_adjusted_value_created.as_ref().unwrap()),
                )?;

            self.indexes_to_value_destroyed
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_value_destroyed.as_ref().unwrap()),
                )?;

            self.indexes_to_adjusted_value_destroyed
                .as_mut()
                .unwrap()
                .compute_rest(
                    indexes,
                    starting_indexes,
                    exit,
                    Some(self.height_to_adjusted_value_destroyed.as_ref().unwrap()),
                )?;

            self.dateindex_to_realized_cap_30d_change
                .as_mut()
                .unwrap()
                .compute_change(
                    starting_indexes.dateindex,
                    self.indexes_to_realized_cap
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .unwrap_last(),
                    30,
                    exit,
                )?;

            self.indexes_to_net_realized_profit_and_loss
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_subtract(
                            starting_indexes.height,
                            self.height_to_realized_profit.as_ref().unwrap(),
                            self.height_to_realized_loss.as_ref().unwrap(),
                            exit,
                        )
                    },
                )?;

            self.indexes_to_realized_value
                .as_mut()
                .unwrap()
                .compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
                        vec.compute_add(
                            starting_indexes.height,
                            self.height_to_realized_profit.as_ref().unwrap(),
                            self.height_to_realized_loss.as_ref().unwrap(),
                            exit,
                        )
                    },
                )?;

            self.dateindex_to_spent_output_profit_ratio
                .as_mut()
                .unwrap()
                .compute_divide(
                    starting_indexes.dateindex,
                    self.indexes_to_value_created
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .unwrap_sum(),
                    self.indexes_to_value_destroyed
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .unwrap_sum(),
                    exit,
                )?;

            self.dateindex_to_adjusted_spent_output_profit_ratio
                .as_mut()
                .unwrap()
                .compute_divide(
                    starting_indexes.dateindex,
                    self.indexes_to_adjusted_value_created
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .unwrap_sum(),
                    self.indexes_to_adjusted_value_destroyed
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .unwrap_sum(),
                    exit,
                )?;

            self.dateindex_to_sell_side_risk_ratio
                .as_mut()
                .unwrap()
                .compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_realized_value
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .unwrap_sum(),
                    self.indexes_to_realized_cap
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .unwrap_last(),
                    exit,
                )?;
        }

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            vec![
                &self.height_to_supply as &dyn AnyCollectableVec,
                &self.height_to_utxo_count,
            ],
            self.height_to_realized_cap
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_supply.vecs(),
            self.indexes_to_utxo_count.vecs(),
            self.indexes_to_realized_cap
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_realized_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_realized_value
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_realized_price_extra
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_realized_profit
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_realized_profit
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_realized_loss
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_realized_loss
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.indexes_to_negative_realized_loss
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_value_created
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_value_created
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_adjusted_value_created
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_adjusted_value_created
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_value_destroyed
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.dateindex_to_spent_output_profit_ratio
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.dateindex_to_adjusted_spent_output_profit_ratio
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_value_destroyed
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.height_to_adjusted_value_destroyed
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.indexes_to_adjusted_value_destroyed
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.dateindex_to_realized_cap_30d_change
                .as_ref()
                .map_or(vec![], |v| vec![v]),
            self.indexes_to_net_realized_profit_and_loss
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.dateindex_to_sell_side_risk_ratio
                .as_ref()
                .map_or(vec![], |v| vec![v]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}

impl Clone for Vecs {
    fn clone(&self) -> Self {
        Self {
            starting_height: self.starting_height,
            state: CohortState::default(),

            height_to_realized_cap: self.height_to_realized_cap.clone(),
            height_to_supply: self.height_to_supply.clone(),
            height_to_utxo_count: self.height_to_utxo_count.clone(),
            height_to_realized_profit: self.height_to_realized_profit.clone(),
            height_to_realized_loss: self.height_to_realized_loss.clone(),
            height_to_value_created: self.height_to_value_created.clone(),
            height_to_adjusted_value_created: self.height_to_adjusted_value_created.clone(),
            height_to_value_destroyed: self.height_to_value_destroyed.clone(),
            height_to_adjusted_value_destroyed: self.height_to_adjusted_value_destroyed.clone(),

            indexes_to_supply: self.indexes_to_supply.clone(),
            indexes_to_utxo_count: self.indexes_to_utxo_count.clone(),
            indexes_to_realized_cap: self.indexes_to_realized_cap.clone(),
            indexes_to_realized_profit: self.indexes_to_realized_profit.clone(),
            indexes_to_realized_loss: self.indexes_to_realized_loss.clone(),
            indexes_to_negative_realized_loss: self.indexes_to_negative_realized_loss.clone(),
            indexes_to_value_created: self.indexes_to_value_created.clone(),
            indexes_to_adjusted_value_created: self.indexes_to_adjusted_value_created.clone(),
            indexes_to_value_destroyed: self.indexes_to_value_destroyed.clone(),
            indexes_to_adjusted_value_destroyed: self.indexes_to_adjusted_value_destroyed.clone(),
            dateindex_to_realized_cap_30d_change: self.dateindex_to_realized_cap_30d_change.clone(),
            indexes_to_realized_value: self.indexes_to_realized_value.clone(),
            indexes_to_net_realized_profit_and_loss: self
                .indexes_to_net_realized_profit_and_loss
                .clone(),
            indexes_to_realized_price: self.indexes_to_realized_price.clone(),
            dateindex_to_sell_side_risk_ratio: self.dateindex_to_sell_side_risk_ratio.clone(),
            indexes_to_realized_price_extra: self.indexes_to_realized_price_extra.clone(),
            dateindex_to_spent_output_profit_ratio: self
                .dateindex_to_spent_output_profit_ratio
                .clone(),
            dateindex_to_adjusted_spent_output_profit_ratio: self
                .dateindex_to_adjusted_spent_output_profit_ratio
                .clone(),
        }
    }
}
