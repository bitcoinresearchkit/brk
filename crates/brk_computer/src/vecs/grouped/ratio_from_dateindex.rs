use std::{f32, path::Path};

use brk_core::{Date, DateIndex, Dollars, Result, StoredF32, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyIterableVec, AnyVec, CollectableVec, EagerVec, Format, StoredIndex,
    VecIterator,
};

use crate::{
    utils::get_percentile,
    vecs::{Indexes, fetched, indexes},
};

use super::{ComputedVecsFromDateIndex, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedRatioVecsFromDateIndex {
    pub price: Option<ComputedVecsFromDateIndex<Dollars>>,

    pub ratio: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1w_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1m_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1y_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_4y_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1y_sma_momentum_oscillator: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_sd: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_4y_sd: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1y_sd: ComputedVecsFromDateIndex<StoredF32>,
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
    pub ratio_4y_zscore: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1y_zscore: ComputedVecsFromDateIndex<StoredF32>,
}

const VERSION: Version = Version::ZERO;

impl ComputedRatioVecsFromDateIndex {
    pub fn forced_import(
        path: &Path,
        name: &str,
        compute_source: bool,
        version: Version,
        format: Format,
    ) -> color_eyre::Result<Self> {
        let options = StorableVecGeneatorOptions::default().add_last();

        Ok(Self {
            price: compute_source.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    path,
                    name,
                    true,
                    version + VERSION,
                    format,
                    options,
                )
                .unwrap()
            }),
            ratio: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_sma: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_sma"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_1w_sma: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_1w_sma"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_1m_sma: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_1m_sma"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_1y_sma: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_1y_sma"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_4y_sma: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_4y_sma"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_1y_sma_momentum_oscillator: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_1y_sma_momentum_oscillator"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_sd"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_4y_sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_4y_sd"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_1y_sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_1y_sd"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p99_9: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99_9"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p99_5: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99_5"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p99: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p1: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p1"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p0_5: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p0_5"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p0_1: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p0_1"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p1sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p1sd"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p2sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p2sd"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p3sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p3sd"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_m1sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m1sd"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_m2sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m2sd"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_m3sd: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m3sd"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p99_9_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99_9_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p99_5_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99_5_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p99_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p99_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p1_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p1_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p0_5_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p0_5_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p0_1_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p0_1_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p1sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p1sd_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p2sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p2sd_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_p3sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_p3sd_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_m1sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m1sd_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_m2sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m2sd_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_m3sd_as_price: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_m3sd_as_price"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_zscore: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_zscore"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_4y_zscore: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_4y_zscore"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            ratio_1y_zscore: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_ratio_1y_zscore"),
                true,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
        })
    }

    pub fn compute_all<F>(
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
            fetched,
            starting_indexes,
            exit,
            date_to_price_opt,
        )
    }

    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: &fetched::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        date_to_price_opt: Option<&impl AnyIterableVec<DateIndex, Dollars>>,
    ) -> color_eyre::Result<()> {
        let date_to_price = date_to_price_opt.unwrap_or_else(|| unsafe {
            std::mem::transmute(&self.price.as_ref().unwrap().dateindex)
        });

        let closes = fetched.timeindexes_to_close.dateindex.as_ref().unwrap();

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
                )
            },
        )?;

        let min_ratio_date = DateIndex::try_from(Date::MIN_RATIO).unwrap();

        self.ratio_sma.compute_all(
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
                )
            },
        )?;

        self.ratio_1w_sma.compute_all(
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
                )
            },
        )?;

        self.ratio_1m_sma.compute_all(
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
                )
            },
        )?;

        self.ratio_1y_sma.compute_all(
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
                )
            },
        )?;

        self.ratio_4y_sma.compute_all(
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
                )
            },
        )?;

        self.ratio_1y_sma_momentum_oscillator.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut ratio_1y_sma_iter =
                    self.ratio_1y_sma.dateindex.as_ref().unwrap().into_iter();
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
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

        let ratio_version = self.ratio.dateindex.as_ref().unwrap().version();
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

        let mut sorted = self.ratio.dateindex.as_ref().unwrap().collect_range(
            Some(min_ratio_date.unwrap_to_usize()),
            Some(starting_dateindex.unwrap_to_usize()),
        )?;

        sorted.sort_unstable();

        // if sorted.len() != starting_dateindex.unwrap_to_usize() - min_ratio_date.unwrap_to_usize() {
        //     unreachable!();
        // }

        let mut sma_iter = self.ratio_sma.dateindex.as_ref().unwrap().into_iter();
        let mut _4y_sma_iter = self.ratio_4y_sma.dateindex.as_ref().unwrap().into_iter();
        let mut _1y_sma_iter = self.ratio_1y_sma.dateindex.as_ref().unwrap().into_iter();

        let nan = StoredF32::from(f32::NAN);
        self.ratio
            .dateindex
            .as_ref()
            .unwrap()
            .iter_at(starting_dateindex)
            .try_for_each(|(index, ratio)| -> Result<()> {
                if index < min_ratio_date {
                    self.ratio_p0_1
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p0_5
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p1
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p99
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p99_5
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p99_9
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_4y_sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_1y_sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;

                    self.ratio_p1sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p2sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_p3sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_m1sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_m2sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                    self.ratio_m3sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, nan, exit)?;
                } else {
                    let ratio = ratio.into_owned();
                    let pos = sorted.binary_search(&ratio).unwrap_or_else(|pos| pos);
                    sorted.insert(pos, ratio);
                    self.ratio_p0_1.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        get_percentile(&sorted, 0.001),
                        exit,
                    )?;
                    self.ratio_p0_5.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        get_percentile(&sorted, 0.005),
                        exit,
                    )?;
                    self.ratio_p1.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        get_percentile(&sorted, 0.01),
                        exit,
                    )?;
                    self.ratio_p99.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        get_percentile(&sorted, 0.99),
                        exit,
                    )?;
                    self.ratio_p99_5
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, get_percentile(&sorted, 0.995), exit)?;
                    self.ratio_p99_9
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
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, _1y_sd, exit)?;

                    self.ratio_p1sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg + sd,
                        exit,
                    )?;
                    self.ratio_p2sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg + 2 * sd,
                        exit,
                    )?;
                    self.ratio_p3sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg + 3 * sd,
                        exit,
                    )?;
                    self.ratio_m1sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg - sd,
                        exit,
                    )?;
                    self.ratio_m2sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg - 2 * sd,
                        exit,
                    )?;
                    self.ratio_m3sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg - 3 * sd,
                        exit,
                    )?;
                }

                Ok(())
            })?;

        self.mut_ratio_vecs()
            .into_iter()
            .try_for_each(|v| v.safe_flush(exit))?;

        self.ratio_p99_9.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p99_5.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p99.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p1.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p0_5.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p0_1.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_sd.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_4y_sd.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_1y_sd.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p1sd.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p2sd.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_p3sd.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_m1sd.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_m2sd.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;
        self.ratio_m3sd.compute_rest(
            indexes,
            starting_indexes,
            exit,
            None as Option<&EagerVec<_, _>>,
        )?;

        let date_to_price = date_to_price_opt.unwrap_or_else(|| unsafe {
            std::mem::transmute(&self.price.as_ref().unwrap().dateindex)
        });

        self.ratio_p99_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p99.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p99_5_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p99_5.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p99_9_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p99_9.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p1_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p1.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p0_5_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p0_5.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p0_1_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p0_1.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p1sd_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p1sd.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p2sd_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p2sd.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_p3sd_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_p3sd.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_m1sd_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_m1sd.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_m2sd_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_m2sd.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_m3sd_as_price.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                let mut iter = self.ratio_m3sd.dateindex.as_ref().unwrap().into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    date_to_price,
                    |(i, price, ..)| {
                        let multiplier = iter.unwrap_get_inner(i);
                        (i, price * multiplier)
                    },
                    exit,
                )
            },
        )?;

        self.ratio_zscore.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_zscore(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    self.ratio_sma.dateindex.as_ref().unwrap(),
                    self.ratio_sd.dateindex.as_ref().unwrap(),
                    exit,
                )
            },
        )?;

        self.ratio_4y_zscore.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_zscore(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    self.ratio_4y_sma.dateindex.as_ref().unwrap(),
                    self.ratio_4y_sd.dateindex.as_ref().unwrap(),
                    exit,
                )
            },
        )?;

        self.ratio_1y_zscore.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_zscore(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.as_ref().unwrap(),
                    self.ratio_1y_sma.dateindex.as_ref().unwrap(),
                    self.ratio_1y_sd.dateindex.as_ref().unwrap(),
                    exit,
                )
            },
        )?;

        Ok(())
    }

    fn mut_ratio_vecs(&mut self) -> Vec<&mut EagerVec<DateIndex, StoredF32>> {
        vec![
            self.ratio_sd.dateindex.as_mut().unwrap(),
            self.ratio_4y_sd.dateindex.as_mut().unwrap(),
            self.ratio_1y_sd.dateindex.as_mut().unwrap(),
            self.ratio_p99_9.dateindex.as_mut().unwrap(),
            self.ratio_p99_5.dateindex.as_mut().unwrap(),
            self.ratio_p99.dateindex.as_mut().unwrap(),
            self.ratio_p1.dateindex.as_mut().unwrap(),
            self.ratio_p0_5.dateindex.as_mut().unwrap(),
            self.ratio_p0_1.dateindex.as_mut().unwrap(),
            self.ratio_p1sd.dateindex.as_mut().unwrap(),
            self.ratio_p2sd.dateindex.as_mut().unwrap(),
            self.ratio_p3sd.dateindex.as_mut().unwrap(),
            self.ratio_m1sd.dateindex.as_mut().unwrap(),
            self.ratio_m2sd.dateindex.as_mut().unwrap(),
            self.ratio_m3sd.dateindex.as_mut().unwrap(),
        ]
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.price.as_ref().map_or(vec![], |v| v.vecs()),
            self.ratio.vecs(),
            self.ratio_sma.vecs(),
            self.ratio_1w_sma.vecs(),
            self.ratio_1m_sma.vecs(),
            self.ratio_1y_sma.vecs(),
            self.ratio_4y_sma.vecs(),
            self.ratio_1y_sma_momentum_oscillator.vecs(),
            self.ratio_sd.vecs(),
            self.ratio_1y_sd.vecs(),
            self.ratio_4y_sd.vecs(),
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
            self.ratio_1y_zscore.vecs(),
            self.ratio_4y_zscore.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
