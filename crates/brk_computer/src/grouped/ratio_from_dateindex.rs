use std::{f32, sync::Arc};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{Date, DateIndex, Dollars, StoredF32, Version};
use brk_vecs::{
    AnyCollectableVec, AnyIterableVec, AnyStoredVec, AnyVec, CollectableVec, Computation, EagerVec,
    Exit, File, Format, GenericStoredVec, StoredIndex, VecIterator,
};

use crate::{Indexes, grouped::source::Source, indexes, price, utils::get_percentile};

use super::{ComputedVecsFromDateIndex, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedRatioVecsFromDateIndex {
    pub price: Option<ComputedVecsFromDateIndex<Dollars>>,

    pub ratio: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_sma: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_1w_sma: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_1m_sma: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_1y_sma: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_4y_sma: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_1y_sma_momentum_oscillator: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_4y_sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_1y_sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_p99_9: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_p99_5: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_p99: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_p1: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_p0_5: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_p0_1: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_p1sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_p2sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_p3sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_m1sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_m2sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_m3sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_p99_9_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_p99_5_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_p99_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_p1_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_p0_5_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_p0_1_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_p1sd_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_p2sd_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_p3sd_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_m1sd_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_m2sd_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_m3sd_as_price: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_zscore: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_4y_zscore: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_1y_zscore: Option<ComputedVecsFromDateIndex<StoredF32>>,
}

const VERSION: Version = Version::ZERO;

impl ComputedRatioVecsFromDateIndex {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        file: &Arc<File>,
        name: &str,
        source: Source<DateIndex, Dollars>,
        version: Version,
        format: Format,
        computation: Computation,
        indexes: &indexes::Vecs,
        extended: bool,
    ) -> Result<Self> {
        let options = VecBuilderOptions::default().add_last();

        Ok(Self {
            price: source.is_compute().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    name,
                    Source::Compute,
                    version + VERSION,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio: ComputedVecsFromDateIndex::forced_import(
                file,
                &format!("{name}_ratio"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                options,
            )?,
            ratio_sma: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_sma"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_1w_sma: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_1w_sma"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_1m_sma: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_1m_sma"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_1y_sma: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_1y_sma"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_4y_sma: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_4y_sma"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_1y_sma_momentum_oscillator: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_1y_sma_momentum_oscillator"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_sd: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_sd"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_4y_sd: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_4y_sd"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_1y_sd: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_1y_sd"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p99_9: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p99_9"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p99_5: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p99_5"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p99: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p99"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p1: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p1"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p0_5: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p0_5"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p0_1: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p0_1"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p1sd: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p1sd"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p2sd: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p2sd"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p3sd: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p3sd"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_m1sd: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_m1sd"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_m2sd: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_m2sd"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_m3sd: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_m3sd"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p99_9_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p99_9_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p99_5_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p99_5_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p99_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p99_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p1_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p1_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p0_5_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p0_5_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p0_1_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p0_1_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p1sd_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p1sd_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p2sd_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p2sd_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_p3sd_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_p3sd_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_m1sd_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_m1sd_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_m2sd_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_m2sd_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_m3sd_as_price: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_m3sd_as_price"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_zscore: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_zscore"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_4y_zscore: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_4y_zscore"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            ratio_1y_zscore: extended.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    file,
                    &format!("{name}_ratio_1y_zscore"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    format,
                    computation,
                    indexes,
                    options,
                )
                .unwrap()
            }),
        })
    }

    pub fn compute_all<F>(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        compute: F,
    ) -> Result<()>
    where
        F: FnMut(
            &mut EagerVec<DateIndex, Dollars>,
            &Indexer,
            &indexes::Vecs,
            &Indexes,
            &Exit,
        ) -> Result<()>,
    {
        self.price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            compute,
        )?;

        let date_to_price_opt: Option<&EagerVec<DateIndex, Dollars>> = None;
        self.compute_rest(
            indexer,
            indexes,
            price,
            starting_indexes,
            exit,
            date_to_price_opt,
        )
    }

    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        date_to_price_opt: Option<&impl AnyIterableVec<DateIndex, Dollars>>,
    ) -> Result<()> {
        let date_to_price = date_to_price_opt.unwrap_or_else(|| unsafe {
            std::mem::transmute(&self.price.as_ref().unwrap().dateindex)
        });

        let closes = price.timeindexes_to_close.dateindex.as_ref().unwrap();

        self.ratio.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut price = date_to_price.iter();
                v.compute_transform(
                    starting_indexes.dateindex,
                    closes,
                    |(i, close, ..)| {
                        let price = price.unwrap_get_inner(i);
                        if price == Dollars::ZERO {
                            (i, StoredF32::from(1.0))
                        } else {
                            (i, StoredF32::from(*close / price))
                        }
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        if self.ratio_sma.is_none() {
            return Ok(());
        }

        let min_ratio_date = DateIndex::try_from(Date::MIN_RATIO).unwrap();

        self.ratio_sma.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    usize::MAX,
                    exit,
                    Some(min_ratio_date),
                )?;
                Ok(())
            },
        )?;

        self.ratio_1w_sma.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    7,
                    exit,
                    Some(min_ratio_date),
                )?;
                Ok(())
            },
        )?;

        self.ratio_1m_sma.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    30,
                    exit,
                    Some(min_ratio_date),
                )?;
                Ok(())
            },
        )?;

        self.ratio_1y_sma.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    365,
                    exit,
                    Some(min_ratio_date),
                )?;
                Ok(())
            },
        )?;

        self.ratio_4y_sma.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    4 * 365,
                    exit,
                    Some(min_ratio_date),
                )?;
                Ok(())
            },
        )?;

        self.ratio_1y_sma_momentum_oscillator
            .as_mut()
            .unwrap()
            .compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    let mut ratio_1y_sma_iter = self
                        .ratio_1y_sma
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .as_ref()
                        .unwrap()
                        .into_iter();
                    v.compute_transform(
                        starting_indexes.dateindex,
                        self.ratio.dateindex.as_ref().unwrap(),
                        |(i, ratio, ..)| {
                            (
                                i,
                                StoredF32::from(
                                    *ratio / *ratio_1y_sma_iter.unwrap_get_inner(i) - 1.0,
                                ),
                            )
                        },
                        exit,
                    )?;
                    Ok(())
                },
            )?;

        let ratio_version = self.ratio.dateindex.as_ref().unwrap().version();
        self.mut_ratio_vecs()
            .iter_mut()
            .try_for_each(|v| -> Result<()> {
                v.validate_computed_version_or_reset_file(
                    Version::ZERO + v.inner_version() + ratio_version,
                )?;
                Ok(())
            })?;

        let starting_dateindex = self
            .mut_ratio_vecs()
            .iter()
            .map(|v| DateIndex::from(v.len()))
            .min()
            .unwrap()
            .min(starting_indexes.dateindex);

        let mut sorted = self.ratio.dateindex.as_ref().unwrap().collect_range(
            Some(min_ratio_date.unwrap_to_usize()),
            Some(starting_dateindex.unwrap_to_usize()),
        )?;

        sorted.sort_unstable();

        // if sorted.len() != starting_dateindex.unwrap_to_usize() - min_ratio_date.unwrap_to_usize() {
        //     unreachable!();
        // }

        let mut sma_iter = self
            .ratio_sma
            .as_ref()
            .unwrap()
            .dateindex
            .as_ref()
            .unwrap()
            .into_iter();
        let mut _4y_sma_iter = self
            .ratio_4y_sma
            .as_ref()
            .unwrap()
            .dateindex
            .as_ref()
            .unwrap()
            .into_iter();
        let mut _1y_sma_iter = self
            .ratio_1y_sma
            .as_ref()
            .unwrap()
            .dateindex
            .as_ref()
            .unwrap()
            .into_iter();

        let nan = StoredF32::from(f32::NAN);
        self.ratio
            .dateindex
            .as_ref()
            .unwrap()
            .iter_at(starting_dateindex)
            .try_for_each(|(index, ratio)| -> Result<()> {
                if index < min_ratio_date {
                    self.ratio_p0_1
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p0_5
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p1
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p99
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p99_5
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p99_9
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_4y_sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_1y_sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;

                    self.ratio_p1sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p2sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p3sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_m1sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_m2sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_m3sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                } else {
                    let ratio = ratio.into_owned();
                    let pos = sorted.binary_search(&ratio).unwrap_or_else(|pos| pos);
                    sorted.insert(pos, ratio);
                    self.ratio_p0_1
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, get_percentile(&sorted, 0.001), exit)?;
                    self.ratio_p0_5
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, get_percentile(&sorted, 0.005), exit)?;
                    self.ratio_p1
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, get_percentile(&sorted, 0.01), exit)?;
                    self.ratio_p99
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, get_percentile(&sorted, 0.99), exit)?;
                    self.ratio_p99_5
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, get_percentile(&sorted, 0.995), exit)?;
                    self.ratio_p99_9
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, get_percentile(&sorted, 0.999), exit)?;

                    let avg = sma_iter.unwrap_get_inner(index);

                    let sd = StoredF32::from(
                        (sorted.iter().map(|v| (**v - *avg).powi(2)).sum::<f32>()
                            / (index.unwrap_to_usize() + 1) as f32)
                            .sqrt(),
                    );

                    self.ratio_sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, sd, exit)?;

                    let _4y_avg = _4y_sma_iter.unwrap_get_inner(index);

                    let _4y_sd = StoredF32::from(
                        (sorted.iter().map(|v| (**v - *_4y_avg).powi(2)).sum::<f32>()
                            / (index.unwrap_to_usize() + 1) as f32)
                            .sqrt(),
                    );

                    self.ratio_4y_sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, _4y_sd, exit)?;

                    let _1y_avg = _1y_sma_iter.unwrap_get_inner(index);

                    let _1y_sd = StoredF32::from(
                        (sorted.iter().map(|v| (**v - *_1y_avg).powi(2)).sum::<f32>()
                            / (index.unwrap_to_usize() + 1) as f32)
                            .sqrt(),
                    );

                    self.ratio_1y_sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, _1y_sd, exit)?;

                    self.ratio_p1sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, avg + sd, exit)?;
                    self.ratio_p2sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, avg + 2 * sd, exit)?;
                    self.ratio_p3sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, avg + 3 * sd, exit)?;
                    self.ratio_m1sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, avg - sd, exit)?;
                    self.ratio_m2sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, avg - 2 * sd, exit)?;
                    self.ratio_m3sd
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, avg - 3 * sd, exit)?;
                }

                Ok(())
            })?;

        drop(sma_iter);
        drop(_4y_sma_iter);
        drop(_1y_sma_iter);

        self.mut_ratio_vecs()
            .into_iter()
            .try_for_each(|v| v.safe_flush(exit))?;

        self.ratio_p99_9.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p99_5.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p99.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p1.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p0_5.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p0_1.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_sd.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_4y_sd.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_1y_sd.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p1sd.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p2sd.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p3sd.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_m1sd.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_m2sd.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_m3sd.as_mut().unwrap().compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;

        let date_to_price = date_to_price_opt.unwrap_or_else(|| unsafe {
            std::mem::transmute(&self.price.as_ref().unwrap().dateindex)
        });

        self.ratio_p99_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_p99
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_p99_5_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_p99_5
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_p99_9_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_p99_9
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_p1_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_p1
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_p0_5_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_p0_5
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_p0_1_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_p0_1
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_p1sd_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_p1sd
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_p2sd_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_p2sd
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_p3sd_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_p3sd
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_m1sd_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_m1sd
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_m2sd_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_m2sd
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_m3sd_as_price.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self
                    .ratio_m3sd
                    .as_ref()
                    .unwrap()
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_zscore.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_zscore(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    self.ratio_sma.as_ref().unwrap().dateindex.as_ref().unwrap(),
                    self.ratio_sd.as_ref().unwrap().dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_4y_zscore.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_zscore(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    self.ratio_4y_sma
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    self.ratio_4y_sd
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.ratio_1y_zscore.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_zscore(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    self.ratio_1y_sma
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    self.ratio_1y_sd
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    exit,
                )?;
                Ok(())
            },
        )?;

        Ok(())
    }

    fn mut_ratio_vecs(&mut self) -> Vec<&mut EagerVec<DateIndex, StoredF32>> {
        [
            self.ratio_sd
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_4y_sd
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_1y_sd
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_p99_9
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_p99_5
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_p99
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_p1
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_p0_5
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_p0_1
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_p1sd
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_p2sd
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_p3sd
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_m1sd
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_m2sd
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
            self.ratio_m3sd
                .as_mut()
                .map_or(vec![], |v| vec![v.dateindex.as_mut().unwrap()]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.price.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio.vecs(),
            self.ratio_sma.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_1w_sma.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_1m_sma.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_1y_sma.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_4y_sma.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_1y_sma_momentum_oscillator
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_sd.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_1y_sd.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_4y_sd.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p99_9.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p99_5.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p99.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p1.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p0_5.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p0_1.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p1sd.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p2sd.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p3sd.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_m1sd.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_m2sd.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_m3sd.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p99_9_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_p99_5_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_p99_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_p1_as_price.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_p0_5_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_p0_1_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_p1sd_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_p2sd_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_p3sd_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_m1sd_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_m2sd_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_m3sd_as_price
                .as_ref()
                .map_or(vec![], |v| v.vecs()),
            self.ratio_zscore.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_1y_zscore.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio_4y_zscore.as_ref().map_or(vec![], |v| v.vecs()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
