use std::{f32, path::Path};

use brk_core::{Date, DateIndex, Dollars, StoredF32};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyIterableVec, AnyVec, CollectableVec, Compressed, EagerVec, Result,
    StoredIndex, VecIterator, Version,
};
// use rayon::prelude::*;

use crate::{
    utils::get_percentile,
    vecs::{Indexes, fetched, indexes},
};

use super::{ComputedVecsFromDateIndex, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedRatioVecsFromDateIndex {
    pub price: ComputedVecsFromDateIndex<Dollars>,

    pub ratio: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1w_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1m_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1y_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1y_sma_momentum_oscillator: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_standard_deviation: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_p99_9: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_p99_5: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_p99: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_p1: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_p0_5: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_p0_1: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_p1sd: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_p2sd: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_p3sd: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_m1sd: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_m2sd: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_m3sd: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_p99_9_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_p99_5_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_p99_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_p1_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_p0_5_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_p0_1_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_p1sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_p2sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_p3sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_m1sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_m2sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_m3sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub ratio_zscore: ComputedVecsFromDateIndex<StoredF32>,
}

const VERSION: Version = Version::ZERO;

impl ComputedRatioVecsFromDateIndex {
    pub fn forced_import(
        path: &Path,
        name: &str,
        // _compute_source: bool,
        version: Version,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        Ok(Self {
            price: ComputedVecsFromDateIndex::forced_import(
                path,
                name,
                VERSION + version,
                compressed,
                options,
            )?,
            ratio: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_sma: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_sma"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_1w_sma: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_1w_sma"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_1m_sma: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_1m_sma"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_1y_sma: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_1y_sma"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_1y_sma_momentum_oscillator: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_1y_sma_momentum_oscillator"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_standard_deviation: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_standard_deviation"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p99_9: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99_9"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p99_5: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99_5"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p99: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p1: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p1"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p0_5: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p0_5"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p0_1: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p0_1"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p1sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p1sd"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p2sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p2sd"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p3sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p3sd"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_m1sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m1sd"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_m2sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m2sd"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_m3sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m3sd"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p99_9_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99_9_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p99_5_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99_5_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p99_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p1_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p1_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p0_5_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p0_5_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p0_1_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p0_1_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p1sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p1sd_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p2sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p2sd_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_p3sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p3sd_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_m1sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m1sd_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_m2sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m2sd_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_m3sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m3sd_as_price"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
            ratio_zscore: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_zscore"),
                VERSION + version + Version::ZERO,
                compressed,
                options,
            )?,
        })
    }

    pub fn compute<F>(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: &fetched::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        compute: F,
    ) -> color_eyre::Result<()>
    where
        F: FnMut(
            &mut EagerVec<DateIndex, Dollars>,
            &Indexer,
            &indexes::Vecs,
            &Indexes,
            &Exit,
        ) -> Result<()>,
    {
        self.price
            .compute(indexer, indexes, starting_indexes, exit, compute)?;

        let closes = &fetched.timeindexes_to_close.dateindex;

        self.ratio.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut price = self.price.dateindex.into_iter();
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
                )
            },
        )?;

        let min_ratio_date = DateIndex::try_from(Date::MIN_RATIO).unwrap();

        self.ratio_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    &self.ratio.dateindex,
                    usize::MAX,
                    exit,
                    Some(min_ratio_date),
                )
            },
        )?;

        self.ratio_1w_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    &self.ratio.dateindex,
                    7,
                    exit,
                    Some(min_ratio_date),
                )
            },
        )?;

        self.ratio_1m_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    &self.ratio.dateindex,
                    30,
                    exit,
                    Some(min_ratio_date),
                )
            },
        )?;

        self.ratio_1y_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    &self.ratio.dateindex,
                    365,
                    exit,
                    Some(min_ratio_date),
                )
            },
        )?;

        self.ratio_1y_sma_momentum_oscillator.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut ratio_1y_sma_iter = self.ratio_1y_sma.dateindex.into_iter();
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.ratio.dateindex,
                    |(i, ratio, ..)| {
                        (
                            i,
                            StoredF32::from(*ratio / *ratio_1y_sma_iter.unwrap_get_inner(i) - 1.0),
                        )
                    },
                    exit,
                )
            },
        )?;

        let ratio_version = self.ratio.dateindex.version();
        self.mut_ratio_vecs()
            .iter_mut()
            .try_for_each(|v| -> Result<()> {
                v.validate_computed_version_or_reset_file(
                    Version::ZERO + v.inner_version() + ratio_version,
                )
            })?;

        let starting_dateindex = self
            .mut_ratio_vecs()
            .iter()
            .map(|v| DateIndex::from(v.len()))
            .min()
            .unwrap()
            .min(starting_indexes.dateindex);

        let mut sorted = self.ratio.dateindex.collect_range(
            Some(min_ratio_date.unwrap_to_usize()),
            Some(starting_dateindex.unwrap_to_usize()),
        )?;

        sorted.sort_unstable();

        // if sorted.len() != starting_dateindex.unwrap_to_usize() - min_ratio_date.unwrap_to_usize() {
        //     unreachable!();
        // }

        let mut sma_iter = self.ratio_sma.dateindex.into_iter();

        let nan = StoredF32::from(f32::NAN);
        self.ratio
            .dateindex
            .iter_at(starting_dateindex)
            .try_for_each(|(index, ratio)| -> Result<()> {
                if index < min_ratio_date {
                    self.ratio_p0_1.dateindex.forced_push_at(index, nan, exit)?;
                    self.ratio_p0_5.dateindex.forced_push_at(index, nan, exit)?;
                    self.ratio_p1.dateindex.forced_push_at(index, nan, exit)?;
                    self.ratio_p99.dateindex.forced_push_at(index, nan, exit)?;
                    self.ratio_p99_5
                        .dateindex
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p99_9
                        .dateindex
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_standard_deviation
                        .dateindex
                        .forced_push_at(index, nan, exit)?;

                    self.ratio_p1sd.dateindex.forced_push_at(index, nan, exit)?;
                    self.ratio_p2sd.dateindex.forced_push_at(index, nan, exit)?;
                    self.ratio_p3sd.dateindex.forced_push_at(index, nan, exit)?;
                    self.ratio_m1sd.dateindex.forced_push_at(index, nan, exit)?;
                    self.ratio_m2sd.dateindex.forced_push_at(index, nan, exit)?;
                    self.ratio_m3sd.dateindex.forced_push_at(index, nan, exit)?;
                } else {
                    let ratio = ratio.into_inner();
                    let pos = sorted.binary_search(&ratio).unwrap_or_else(|pos| pos);
                    sorted.insert(pos, ratio);
                    self.ratio_p0_1.dateindex.forced_push_at(
                        index,
                        get_percentile(&sorted, 0.001),
                        exit,
                    )?;
                    self.ratio_p0_5.dateindex.forced_push_at(
                        index,
                        get_percentile(&sorted, 0.005),
                        exit,
                    )?;
                    self.ratio_p1.dateindex.forced_push_at(
                        index,
                        get_percentile(&sorted, 0.01),
                        exit,
                    )?;
                    self.ratio_p99.dateindex.forced_push_at(
                        index,
                        get_percentile(&sorted, 0.99),
                        exit,
                    )?;
                    self.ratio_p99_5.dateindex.forced_push_at(
                        index,
                        get_percentile(&sorted, 0.995),
                        exit,
                    )?;
                    self.ratio_p99_9.dateindex.forced_push_at(
                        index,
                        get_percentile(&sorted, 0.999),
                        exit,
                    )?;

                    let avg = sma_iter.unwrap_get_inner(index);

                    let sd = StoredF32::from(
                        (sorted.iter().map(|v| (**v - *avg).powi(2)).sum::<f32>()
                            / (index.unwrap_to_usize() + 1) as f32)
                            .sqrt(),
                    );

                    self.ratio_standard_deviation
                        .dateindex
                        .forced_push_at(index, sd, exit)?;

                    self.ratio_p1sd
                        .dateindex
                        .forced_push_at(index, avg + sd, exit)?;
                    self.ratio_p2sd
                        .dateindex
                        .forced_push_at(index, avg + 2 * sd, exit)?;
                    self.ratio_p3sd
                        .dateindex
                        .forced_push_at(index, avg + 3 * sd, exit)?;
                    self.ratio_m1sd
                        .dateindex
                        .forced_push_at(index, avg - sd, exit)?;
                    self.ratio_m2sd
                        .dateindex
                        .forced_push_at(index, avg - 2 * sd, exit)?;
                    self.ratio_m3sd
                        .dateindex
                        .forced_push_at(index, avg - 3 * sd, exit)?;
                }

                Ok(())
            })?;

        self.mut_ratio_vecs()
            .into_iter()
            .try_for_each(|v| v.safe_flush(exit))?;

        self.ratio_p99_9
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_p99_5
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_p99
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_p1
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_p0_5
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_p0_1
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_standard_deviation
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_p1sd
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_p2sd
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_p3sd
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_m1sd
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_m2sd
            .compute_rest(indexes, starting_indexes, exit)?;
        self.ratio_m3sd
            .compute_rest(indexes, starting_indexes, exit)?;

        self.ratio_p99_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p99.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p99_5_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p99_5.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p99_9_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p99_9.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p1_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p1.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p0_5_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p0_5.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p0_1_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p0_1.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p1sd_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p1sd.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p2sd_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p2sd.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p3sd_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p3sd.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_m1sd_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_m1sd.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_m2sd_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_m2sd.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_m3sd_as_price.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_m3sd.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.price.dateindex,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_zscore.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut sma_iter = self.ratio_sma.dateindex.into_iter();
                let mut sd_iter = self.ratio_standard_deviation.dateindex.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &self.ratio.dateindex,
                    |(i, ratio, ..)| {
                        let sma = sma_iter.unwrap_get_inner(i);
                        let sd = sd_iter.unwrap_get_inner(i);
                        (i, (ratio - sma) / sd)
                    },
                    exit,
                )
            },
        )?;

        Ok(())
    }

    fn mut_ratio_vecs(&mut self) -> Vec<&mut EagerVec<DateIndex, StoredF32>> {
        vec![
            &mut self.ratio_standard_deviation.dateindex,
            &mut self.ratio_p99_9.dateindex,
            &mut self.ratio_p99_5.dateindex,
            &mut self.ratio_p99.dateindex,
            &mut self.ratio_p1.dateindex,
            &mut self.ratio_p0_5.dateindex,
            &mut self.ratio_p0_1.dateindex,
            &mut self.ratio_p1sd.dateindex,
            &mut self.ratio_p2sd.dateindex,
            &mut self.ratio_p3sd.dateindex,
            &mut self.ratio_m1sd.dateindex,
            &mut self.ratio_m2sd.dateindex,
            &mut self.ratio_m3sd.dateindex,
        ]
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.price.vecs(),
            self.ratio.vecs(),
            self.ratio_sma.vecs(),
            self.ratio_1w_sma.vecs(),
            self.ratio_1m_sma.vecs(),
            self.ratio_1y_sma.vecs(),
            self.ratio_1y_sma_momentum_oscillator.vecs(),
            self.ratio_standard_deviation.vecs(),
            self.ratio_p99_9.vecs(),
            self.ratio_p99_5.vecs(),
            self.ratio_p99.vecs(),
            self.ratio_p1.vecs(),
            self.ratio_p0_5.vecs(),
            self.ratio_p0_1.vecs(),
            self.ratio_p1sd.vecs(),
            self.ratio_p2sd.vecs(),
            self.ratio_p3sd.vecs(),
            self.ratio_m1sd.vecs(),
            self.ratio_m2sd.vecs(),
            self.ratio_m3sd.vecs(),
            self.ratio_p99_9_as_price.vecs(),
            self.ratio_p99_5_as_price.vecs(),
            self.ratio_p99_as_price.vecs(),
            self.ratio_p1_as_price.vecs(),
            self.ratio_p0_5_as_price.vecs(),
            self.ratio_p0_1_as_price.vecs(),
            self.ratio_p1sd_as_price.vecs(),
            self.ratio_p2sd_as_price.vecs(),
            self.ratio_p3sd_as_price.vecs(),
            self.ratio_m1sd_as_price.vecs(),
            self.ratio_m2sd_as_price.vecs(),
            self.ratio_m3sd_as_price.vecs(),
            self.ratio_zscore.vecs(),
        ]
        .concat()
    }
}
