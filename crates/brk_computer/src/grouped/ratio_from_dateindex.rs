use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Date, DateIndex, Dollars, StoredF32, Version};
use vecdb::{PcoVec, 
    AnyStoredVec, AnyVec, CollectableVec, Database, EagerVec, Exit, GenericStoredVec, IterableVec,
    TypedVecIterator, VecIndex,
};

use crate::{
    Indexes,
    grouped::{
        ComputedStandardDeviationVecsFromDateIndex, StandardDeviationVecsOptions, source::Source,
    },
    indexes, price,
    utils::{get_percentile, OptionExt},
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
    pub ratio_pct99_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_pct98_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_pct95_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_pct5_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_pct2_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub ratio_pct1_usd: Option<ComputedVecsFromDateIndex<Dollars>>,

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
    ) -> Result<Self> {
        let opts = VecBuilderOptions::default().add_last();
        let v = version + VERSION;

        macro_rules! import {
            ($suffix:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    db, &format!("{name}_{}", $suffix), Source::Compute, v, indexes, opts,
                ).unwrap()
            };
        }
        macro_rules! import_sd {
            ($suffix:expr, $days:expr) => {
                ComputedStandardDeviationVecsFromDateIndex::forced_import(
                    db, &format!("{name}_{}", $suffix), $days, Source::Compute, v, indexes,
                    StandardDeviationVecsOptions::default().add_all(),
                ).unwrap()
            };
        }

        Ok(Self {
            price: source.is_compute().then(|| {
                ComputedVecsFromDateIndex::forced_import(db, name, Source::Compute, v, indexes, opts).unwrap()
            }),
            ratio: import!("ratio"),
            ratio_1w_sma: extended.then(|| import!("ratio_1w_sma")),
            ratio_1m_sma: extended.then(|| import!("ratio_1m_sma")),
            ratio_sd: extended.then(|| import_sd!("ratio", usize::MAX)),
            ratio_1y_sd: extended.then(|| import_sd!("ratio_1y", 365)),
            ratio_2y_sd: extended.then(|| import_sd!("ratio_2y", 2 * 365)),
            ratio_4y_sd: extended.then(|| import_sd!("ratio_4y", 4 * 365)),
            ratio_pct99: extended.then(|| import!("ratio_pct99")),
            ratio_pct98: extended.then(|| import!("ratio_pct98")),
            ratio_pct95: extended.then(|| import!("ratio_pct95")),
            ratio_pct5: extended.then(|| import!("ratio_pct5")),
            ratio_pct2: extended.then(|| import!("ratio_pct2")),
            ratio_pct1: extended.then(|| import!("ratio_pct1")),
            ratio_pct99_usd: extended.then(|| import!("ratio_pct99_usd")),
            ratio_pct98_usd: extended.then(|| import!("ratio_pct98_usd")),
            ratio_pct95_usd: extended.then(|| import!("ratio_pct95_usd")),
            ratio_pct5_usd: extended.then(|| import!("ratio_pct5_usd")),
            ratio_pct2_usd: extended.then(|| import!("ratio_pct2_usd")),
            ratio_pct1_usd: extended.then(|| import!("ratio_pct1_usd")),
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

        let price = price_opt.unwrap_or_else(|| unsafe {
            std::mem::transmute(&self.price.u().dateindex)
        });

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

        self.ratio
            .dateindex
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .skip(starting_dateindex.to_usize())
            .try_for_each(|(index, ratio)| -> Result<()> {
                if index < min_ratio_date_usize {
                    self.ratio_pct5
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, StoredF32::NAN)?;
                    self.ratio_pct2
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, StoredF32::NAN)?;
                    self.ratio_pct1
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, StoredF32::NAN)?;
                    self.ratio_pct95
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, StoredF32::NAN)?;
                    self.ratio_pct98
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, StoredF32::NAN)?;
                    self.ratio_pct99
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, StoredF32::NAN)?;
                } else {
                    let pos = sorted.binary_search(&ratio).unwrap_or_else(|pos| pos);
                    sorted.insert(pos, ratio);

                    self.ratio_pct1
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, get_percentile(&sorted, 0.01))?;
                    self.ratio_pct2
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, get_percentile(&sorted, 0.02))?;
                    self.ratio_pct5
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, get_percentile(&sorted, 0.05))?;
                    self.ratio_pct95
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, get_percentile(&sorted, 0.95))?;
                    self.ratio_pct98
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, get_percentile(&sorted, 0.98))?;
                    self.ratio_pct99
                        .as_mut()
                        .unwrap()
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, get_percentile(&sorted, 0.99))?;
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

        let date_to_price = price_opt.unwrap_or_else(|| unsafe {
            std::mem::transmute(&self.price.u().dateindex)
        });

        self.ratio_pct99_usd
            .as_mut()
            .unwrap()
            .compute_all(starting_indexes, exit, |vec| {
                let mut iter = self
                    .ratio_pct99
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
                        let multiplier = iter.get_unwrap(i);
                        (i, price * multiplier)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        let compute_usd =
            |usd: Option<&mut ComputedVecsFromDateIndex<Dollars>>,
             source: Option<&ComputedVecsFromDateIndex<StoredF32>>| {
                usd.unwrap().compute_all(starting_indexes, exit, |vec| {
                    let mut iter = source.unwrap().dateindex.u().into_iter();
                    vec.compute_transform(
                        starting_indexes.dateindex,
                        date_to_price,
                        |(i, price, ..)| {
                            let multiplier = iter.get_unwrap(i);
                            (i, price * multiplier)
                        },
                        exit,
                    )?;
                    Ok(())
                })
            };

        compute_usd(self.ratio_pct1_usd.as_mut(), self.ratio_pct1.as_ref())?;
        compute_usd(self.ratio_pct2_usd.as_mut(), self.ratio_pct2.as_ref())?;
        compute_usd(self.ratio_pct5_usd.as_mut(), self.ratio_pct5.as_ref())?;
        compute_usd(self.ratio_pct95_usd.as_mut(), self.ratio_pct95.as_ref())?;
        compute_usd(self.ratio_pct98_usd.as_mut(), self.ratio_pct98.as_ref())?;
        compute_usd(self.ratio_pct99_usd.as_mut(), self.ratio_pct99.as_ref())?;

        self.ratio_sd.um().compute_all(
            starting_indexes,
            exit,
            self.ratio.dateindex.u(),
            Some(date_to_price),
        )?;
        self.ratio_4y_sd.um().compute_all(
            starting_indexes,
            exit,
            self.ratio.dateindex.u(),
            Some(date_to_price),
        )?;
        self.ratio_2y_sd.um().compute_all(
            starting_indexes,
            exit,
            self.ratio.dateindex.u(),
            Some(date_to_price),
        )?;
        self.ratio_1y_sd.um().compute_all(
            starting_indexes,
            exit,
            self.ratio.dateindex.u(),
            Some(date_to_price),
        )?;

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
