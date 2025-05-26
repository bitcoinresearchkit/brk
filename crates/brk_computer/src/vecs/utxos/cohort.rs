use std::{fs, path::Path};

use brk_core::{CheckedSub, Dollars, Height, Sats, StoredUsize};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyVec, Compressed, Computation, EagerVec, Result, VecIterator, Version,
};

use crate::{
    states::{CohortState, RealizedState},
    vecs::{
        Indexes, fetched,
        grouped::{
            ComputedRatioVecsFromDateIndex, ComputedValueVecsFromHeight, ComputedVecsFromHeight,
            StorableVecGeneatorOptions,
        },
        indexes,
    },
};

const VERSION: Version = Version::ZERO;

pub struct Vecs {
    starting_height: Height,
    pub state: CohortState,

    pub height_to_realized_cap: Option<EagerVec<Height, Dollars>>,
    pub indexes_to_realized_cap: Option<ComputedVecsFromHeight<Dollars>>,
    pub height_to_supply: EagerVec<Height, Sats>,
    pub indexes_to_supply: ComputedValueVecsFromHeight,
    pub height_to_utxo_count: EagerVec<Height, StoredUsize>,
    pub indexes_to_utxo_count: ComputedVecsFromHeight<StoredUsize>,

    pub indexes_to_realized_price: Option<ComputedVecsFromHeight<Dollars>>,
    pub indexes_to_realized_price_extra: Option<ComputedRatioVecsFromDateIndex>,
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
        })
    }

    pub fn starting_height(&self) -> Height {
        [
            self.height_to_supply.len(),
            self.height_to_utxo_count.len(),
            self.height_to_realized_cap
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
                self.state.realized.as_mut().unwrap().realized_cap = height_to_realized_cap
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
            height_to_realized_cap.forced_push_at(
                height,
                self.state
                    .realized
                    .as_ref()
                    .unwrap_or_else(|| {
                        dbg!(&self.state);
                        panic!();
                    })
                    .realized_cap,
                exit,
            )?;
        }
        Ok(())
    }

    pub fn safe_flush_height_vecs(&mut self, exit: &Exit) -> Result<()> {
        self.height_to_supply.safe_flush(exit)?;

        self.height_to_utxo_count.safe_flush(exit)?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
            height_to_realized_cap.safe_flush(exit)?;
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
        }

        if let Some(indexes_to_realized_price) = self.indexes_to_realized_price.as_mut() {
            indexes_to_realized_price.compute_all(
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
        }

        if let Some(indexes_to_realized_price_extra) = self.indexes_to_realized_price_extra.as_mut()
        {
            indexes_to_realized_price_extra.compute_rest(
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
            self.indexes_to_realized_price_extra
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
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
            indexes_to_realized_cap: self.indexes_to_realized_cap.clone(),
            height_to_supply: self.height_to_supply.clone(),
            indexes_to_supply: self.indexes_to_supply.clone(),
            height_to_utxo_count: self.height_to_utxo_count.clone(),
            indexes_to_utxo_count: self.indexes_to_utxo_count.clone(),

            indexes_to_realized_price: self.indexes_to_realized_price.clone(),
            indexes_to_realized_price_extra: self.indexes_to_realized_price_extra.clone(),
        }
    }
}
