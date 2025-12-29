use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Date, DateIndex, Dollars, StoredF32, Version};
use vecdb::{
    AnyStoredVec, AnyVec, CollectableVec, Database, EagerVec, Exit, GenericStoredVec, IterableVec,
    PcoVec, VecIndex,
};

use crate::{
    Indexes,
    grouped::{
        ComputedStandardDeviationVecsFromDateIndex, LazyVecsFrom2FromDateIndex, PriceTimesRatio,
        StandardDeviationVecsOptions, source::Source,
    },
    indexes, price,
    utils::{OptionExt, get_percentile},
};

use super::{ComputedVecsFromDateIndex, VecBuilderOptions};

#[derive(Clone, Traversable)]
pub struct ComputedRatioVecsFromDateIndex {
    pub price: Option<ComputedVecsFromDateIndex<Dollars>>,

    pub ratio: ComputedVecsFromDateIndex<StoredF32>,
    pub ratio_1w_sma: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_1m_sma: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_pct99: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_pct98: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_pct95: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_pct5: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_pct2: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_pct1: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub ratio_pct99_usd: Option<LazyVecsFrom2FromDateIndex<Dollars, Dollars, StoredF32>>,
    pub ratio_pct98_usd: Option<LazyVecsFrom2FromDateIndex<Dollars, Dollars, StoredF32>>,
    pub ratio_pct95_usd: Option<LazyVecsFrom2FromDateIndex<Dollars, Dollars, StoredF32>>,
    pub ratio_pct5_usd: Option<LazyVecsFrom2FromDateIndex<Dollars, Dollars, StoredF32>>,
    pub ratio_pct2_usd: Option<LazyVecsFrom2FromDateIndex<Dollars, Dollars, StoredF32>>,
    pub ratio_pct1_usd: Option<LazyVecsFrom2FromDateIndex<Dollars, Dollars, StoredF32>>,

    pub ratio_sd: Option<ComputedStandardDeviationVecsFromDateIndex>,
    pub ratio_4y_sd: Option<ComputedStandardDeviationVecsFromDateIndex>,
    pub ratio_2y_sd: Option<ComputedStandardDeviationVecsFromDateIndex>,
    pub ratio_1y_sd: Option<ComputedStandardDeviationVecsFromDateIndex>,
}

const VERSION: Version = Version::TWO;

impl ComputedRatioVecsFromDateIndex {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        source: Source<DateIndex, Dollars>,
        version: Version,
        indexes: &indexes::Vecs,
        extended: bool,
        price_vecs: Option<&price::Vecs>,
    ) -> Result<Self> {
        let opts = VecBuilderOptions::default().add_last();
        let v = version + VERSION;

        macro_rules! import {
            ($suffix:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    Source::Compute,
                    v,
                    indexes,
                    opts,
                )
                .unwrap()
            };
        }
        // Create sources first so lazy vecs can reference them
        let price = source.is_compute().then(|| {
            ComputedVecsFromDateIndex::forced_import(db, name, Source::Compute, v, indexes, opts)
                .unwrap()
        });

        macro_rules! import_sd {
            ($suffix:expr, $days:expr) => {
                ComputedStandardDeviationVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    $days,
                    Source::Compute,
                    v,
                    indexes,
                    StandardDeviationVecsOptions::default().add_all(),
                    price_vecs,
                )
                .unwrap()
            };
        }

        let ratio_pct99 = extended.then(|| import!("ratio_pct99"));
        let ratio_pct98 = extended.then(|| import!("ratio_pct98"));
        let ratio_pct95 = extended.then(|| import!("ratio_pct95"));
        let ratio_pct5 = extended.then(|| import!("ratio_pct5"));
        let ratio_pct2 = extended.then(|| import!("ratio_pct2"));
        let ratio_pct1 = extended.then(|| import!("ratio_pct1"));

        // Create lazy usd vecs from price and ratio sources
        macro_rules! lazy_usd {
            ($ratio:expr, $suffix:expr) => {
                price.as_ref().zip($ratio.as_ref()).map(|(p, r)| {
                    LazyVecsFrom2FromDateIndex::from_computed::<PriceTimesRatio>(
                        &format!("{name}_{}", $suffix),
                        v,
                        p,
                        r,
                    )
                })
            };
        }

        Ok(Self {
            ratio: import!("ratio"),
            ratio_1w_sma: extended.then(|| import!("ratio_1w_sma")),
            ratio_1m_sma: extended.then(|| import!("ratio_1m_sma")),
            ratio_sd: extended.then(|| import_sd!("ratio", usize::MAX)),
            ratio_1y_sd: extended.then(|| import_sd!("ratio_1y", 365)),
            ratio_2y_sd: extended.then(|| import_sd!("ratio_2y", 2 * 365)),
            ratio_4y_sd: extended.then(|| import_sd!("ratio_4y", 4 * 365)),
            ratio_pct99_usd: lazy_usd!(&ratio_pct99, "ratio_pct99_usd"),
            ratio_pct98_usd: lazy_usd!(&ratio_pct98, "ratio_pct98_usd"),
            ratio_pct95_usd: lazy_usd!(&ratio_pct95, "ratio_pct95_usd"),
            ratio_pct5_usd: lazy_usd!(&ratio_pct5, "ratio_pct5_usd"),
            ratio_pct2_usd: lazy_usd!(&ratio_pct2, "ratio_pct2_usd"),
            ratio_pct1_usd: lazy_usd!(&ratio_pct1, "ratio_pct1_usd"),
            price,
            ratio_pct99,
            ratio_pct98,
            ratio_pct95,
            ratio_pct5,
            ratio_pct2,
            ratio_pct1,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<DateIndex, Dollars>>) -> Result<()>,
    {
        self.price
            .as_mut()
            .unwrap()
            .compute_all(starting_indexes, exit, compute)?;

        let date_to_price_opt: Option<&EagerVec<PcoVec<DateIndex, Dollars>>> = None;
        self.compute_rest(price, starting_indexes, exit, date_to_price_opt)
    }

    pub fn compute_rest(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        price_opt: Option<&impl IterableVec<DateIndex, Dollars>>,
    ) -> Result<()> {
        let closes = price.timeindexes_to_price_close.dateindex.u();

        let price =
            price_opt.unwrap_or_else(|| unsafe { std::mem::transmute(&self.price.u().dateindex) });

        self.ratio.compute_all(starting_indexes, exit, |v| {
            v.compute_transform2(
                starting_indexes.dateindex,
                closes,
                price,
                |(i, close, price, ..)| {
                    if price == Dollars::ZERO {
                        (i, StoredF32::from(1.0))
                    } else {
                        (i, StoredF32::from(*close / price))
                    }
                },
                exit,
            )?;
            Ok(())
        })?;

        if self.ratio_1w_sma.is_none() {
            return Ok(());
        }

        let min_ratio_date = DateIndex::try_from(Date::MIN_RATIO).unwrap();

        self.ratio_1w_sma
            .as_mut()
            .unwrap()
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.u(),
                    7,
                    exit,
                    Some(min_ratio_date),
                )?;
                Ok(())
            })?;

        self.ratio_1m_sma
            .as_mut()
            .unwrap()
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    self.ratio.dateindex.u(),
                    30,
                    exit,
                    Some(min_ratio_date),
                )?;
                Ok(())
            })?;

        let ratio_version = self.ratio.dateindex.u().version();
        self.mut_ratio_vecs()
            .iter_mut()
            .try_for_each(|v| -> Result<()> {
                v.validate_computed_version_or_reset(
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

        let min_ratio_date_usize = min_ratio_date.to_usize();

        let mut sorted = self.ratio.dateindex.u().collect_range(
            Some(min_ratio_date_usize),
            Some(starting_dateindex.to_usize()),
        );

        sorted.sort_unstable();

        // Cache mutable refs before the loop to avoid repeated unwrap chains
        let pct1_vec = self.ratio_pct1.um().dateindex.um();
        let pct2_vec = self.ratio_pct2.um().dateindex.um();
        let pct5_vec = self.ratio_pct5.um().dateindex.um();
        let pct95_vec = self.ratio_pct95.um().dateindex.um();
        let pct98_vec = self.ratio_pct98.um().dateindex.um();
        let pct99_vec = self.ratio_pct99.um().dateindex.um();

        self.ratio
            .dateindex
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .skip(starting_dateindex.to_usize())
            .try_for_each(|(index, ratio)| -> Result<()> {
                if index < min_ratio_date_usize {
                    pct1_vec.truncate_push_at(index, StoredF32::NAN)?;
                    pct2_vec.truncate_push_at(index, StoredF32::NAN)?;
                    pct5_vec.truncate_push_at(index, StoredF32::NAN)?;
                    pct95_vec.truncate_push_at(index, StoredF32::NAN)?;
                    pct98_vec.truncate_push_at(index, StoredF32::NAN)?;
                    pct99_vec.truncate_push_at(index, StoredF32::NAN)?;
                } else {
                    let pos = sorted.binary_search(&ratio).unwrap_or_else(|pos| pos);
                    sorted.insert(pos, ratio);

                    pct1_vec.truncate_push_at(index, get_percentile(&sorted, 0.01))?;
                    pct2_vec.truncate_push_at(index, get_percentile(&sorted, 0.02))?;
                    pct5_vec.truncate_push_at(index, get_percentile(&sorted, 0.05))?;
                    pct95_vec.truncate_push_at(index, get_percentile(&sorted, 0.95))?;
                    pct98_vec.truncate_push_at(index, get_percentile(&sorted, 0.98))?;
                    pct99_vec.truncate_push_at(index, get_percentile(&sorted, 0.99))?;
                }

                Ok(())
            })?;

        self.mut_ratio_vecs()
            .into_iter()
            .try_for_each(|v| v.safe_flush(exit))?;

        self.ratio_pct1.um().compute_rest(
            starting_indexes,
            exit,
            None as Option<&EagerVec<PcoVec<_, _>>>,
        )?;
        self.ratio_pct2.um().compute_rest(
            starting_indexes,
            exit,
            None as Option<&EagerVec<PcoVec<_, _>>>,
        )?;
        self.ratio_pct5.um().compute_rest(
            starting_indexes,
            exit,
            None as Option<&EagerVec<PcoVec<_, _>>>,
        )?;
        self.ratio_pct95.um().compute_rest(
            starting_indexes,
            exit,
            None as Option<&EagerVec<PcoVec<_, _>>>,
        )?;
        self.ratio_pct98.um().compute_rest(
            starting_indexes,
            exit,
            None as Option<&EagerVec<PcoVec<_, _>>>,
        )?;
        self.ratio_pct99.um().compute_rest(
            starting_indexes,
            exit,
            None as Option<&EagerVec<PcoVec<_, _>>>,
        )?;

        self.ratio_sd
            .um()
            .compute_all(starting_indexes, exit, self.ratio.dateindex.u())?;
        self.ratio_4y_sd
            .um()
            .compute_all(starting_indexes, exit, self.ratio.dateindex.u())?;
        self.ratio_2y_sd
            .um()
            .compute_all(starting_indexes, exit, self.ratio.dateindex.u())?;
        self.ratio_1y_sd
            .um()
            .compute_all(starting_indexes, exit, self.ratio.dateindex.u())?;

        Ok(())
    }

    fn mut_ratio_vecs(&mut self) -> Vec<&mut EagerVec<PcoVec<DateIndex, StoredF32>>> {
        let mut vecs = Vec::with_capacity(6);
        if let Some(v) = self.ratio_pct1.as_mut() {
            vecs.push(v.dateindex.um());
        }
        if let Some(v) = self.ratio_pct2.as_mut() {
            vecs.push(v.dateindex.um());
        }
        if let Some(v) = self.ratio_pct5.as_mut() {
            vecs.push(v.dateindex.um());
        }
        if let Some(v) = self.ratio_pct95.as_mut() {
            vecs.push(v.dateindex.um());
        }
        if let Some(v) = self.ratio_pct98.as_mut() {
            vecs.push(v.dateindex.um());
        }
        if let Some(v) = self.ratio_pct99.as_mut() {
            vecs.push(v.dateindex.um());
        }
        vecs
    }
}
