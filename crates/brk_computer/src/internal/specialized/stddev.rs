use std::mem;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Close, Date, DateIndex, Dollars, StoredF32, Version};
use vecdb::{
    AnyStoredVec, AnyVec, CollectableVec, Database, EagerVec, Exit, GenericStoredVec, IterableVec,
    PcoVec, VecIndex,
};

use crate::{ComputeIndexes, indexes, price};

use super::super::{BinaryDateLast, ClosePriceTimesRatio, ComputedDateLast};

#[derive(Clone, Traversable)]
pub struct ComputedStandardDeviationVecsDate {
    days: usize,

    pub sma: Option<ComputedDateLast<StoredF32>>,

    pub sd: ComputedDateLast<StoredF32>,

    pub zscore: Option<ComputedDateLast<StoredF32>>,

    pub p0_5sd: Option<ComputedDateLast<StoredF32>>,
    pub p1sd: Option<ComputedDateLast<StoredF32>>,
    pub p1_5sd: Option<ComputedDateLast<StoredF32>>,
    pub p2sd: Option<ComputedDateLast<StoredF32>>,
    pub p2_5sd: Option<ComputedDateLast<StoredF32>>,
    pub p3sd: Option<ComputedDateLast<StoredF32>>,
    pub m0_5sd: Option<ComputedDateLast<StoredF32>>,
    pub m1sd: Option<ComputedDateLast<StoredF32>>,
    pub m1_5sd: Option<ComputedDateLast<StoredF32>>,
    pub m2sd: Option<ComputedDateLast<StoredF32>>,
    pub m2_5sd: Option<ComputedDateLast<StoredF32>>,
    pub m3sd: Option<ComputedDateLast<StoredF32>>,

    pub _0sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub p0_5sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub p1sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub p1_5sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub p2sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub p2_5sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub p3sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub m0_5sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub m1sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub m1_5sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub m2sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub m2_5sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
    pub m3sd_usd: Option<BinaryDateLast<Dollars, Close<Dollars>, StoredF32>>,
}

#[derive(Debug, Default)]
pub struct StandardDeviationVecsOptions {
    zscore: bool,
    bands: bool,
    price_bands: bool,
}

impl StandardDeviationVecsOptions {
    pub fn add_all(mut self) -> Self {
        self.zscore = true;
        self.bands = true;
        self.price_bands = true;
        self
    }

    pub fn add_zscore(mut self) -> Self {
        self.zscore = true;
        self
    }

    pub fn add_bands(mut self) -> Self {
        self.bands = true;
        self
    }

    pub fn add_price_bands(mut self) -> Self {
        self.bands = true;
        self.price_bands = true;
        self
    }

    pub fn zscore(&self) -> bool {
        self.zscore
    }

    pub fn bands(&self) -> bool {
        self.bands
    }

    pub fn price_bands(&self) -> bool {
        self.price_bands
    }
}

impl ComputedStandardDeviationVecsDate {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        days: usize,
        parent_version: Version,
        indexes: &indexes::Vecs,
        options: StandardDeviationVecsOptions,
        price_vecs: Option<&price::Vecs>,
    ) -> Result<Self> {
        let version = parent_version + Version::ONE;

        macro_rules! import {
            ($suffix:expr) => {
                ComputedDateLast::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    version,
                    indexes,
                )
                .unwrap()
            };
        }

        let sma_vec = Some(import!("sma"));
        let p0_5sd = options.bands().then(|| import!("p0_5sd"));
        let p1sd = options.bands().then(|| import!("p1sd"));
        let p1_5sd = options.bands().then(|| import!("p1_5sd"));
        let p2sd = options.bands().then(|| import!("p2sd"));
        let p2_5sd = options.bands().then(|| import!("p2_5sd"));
        let p3sd = options.bands().then(|| import!("p3sd"));
        let m0_5sd = options.bands().then(|| import!("m0_5sd"));
        let m1sd = options.bands().then(|| import!("m1sd"));
        let m1_5sd = options.bands().then(|| import!("m1_5sd"));
        let m2sd = options.bands().then(|| import!("m2sd"));
        let m2_5sd = options.bands().then(|| import!("m2_5sd"));
        let m3sd = options.bands().then(|| import!("m3sd"));

        macro_rules! lazy_usd {
            ($band:expr, $suffix:expr) => {
                price_vecs
                    .map(|p| &p.usd.timeindexes_to_price_close)
                    .zip($band.as_ref())
                    .filter(|_| options.price_bands())
                    .map(|(p, b)| {
                        BinaryDateLast::from_computed_both_last::<ClosePriceTimesRatio>(
                            &format!("{name}_{}", $suffix),
                            version,
                            p,
                            b,
                        )
                    })
            };
        }

        Ok(Self {
            days,
            sd: import!("sd"),
            zscore: options.zscore().then(|| import!("zscore")),
            // Lazy USD vecs
            _0sd_usd: lazy_usd!(&sma_vec, "0sd_usd"),
            p0_5sd_usd: lazy_usd!(&p0_5sd, "p0_5sd_usd"),
            p1sd_usd: lazy_usd!(&p1sd, "p1sd_usd"),
            p1_5sd_usd: lazy_usd!(&p1_5sd, "p1_5sd_usd"),
            p2sd_usd: lazy_usd!(&p2sd, "p2sd_usd"),
            p2_5sd_usd: lazy_usd!(&p2_5sd, "p2_5sd_usd"),
            p3sd_usd: lazy_usd!(&p3sd, "p3sd_usd"),
            m0_5sd_usd: lazy_usd!(&m0_5sd, "m0_5sd_usd"),
            m1sd_usd: lazy_usd!(&m1sd, "m1sd_usd"),
            m1_5sd_usd: lazy_usd!(&m1_5sd, "m1_5sd_usd"),
            m2sd_usd: lazy_usd!(&m2sd, "m2sd_usd"),
            m2_5sd_usd: lazy_usd!(&m2_5sd, "m2_5sd_usd"),
            m3sd_usd: lazy_usd!(&m3sd, "m3sd_usd"),
            // Stored band sources
            sma: sma_vec,
            p0_5sd,
            p1sd,
            p1_5sd,
            p2sd,
            p2_5sd,
            p3sd,
            m0_5sd,
            m1sd,
            m1_5sd,
            m2sd,
            m2_5sd,
            m3sd,
        })
    }

    pub fn compute_all(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        source: &impl CollectableVec<DateIndex, StoredF32>,
    ) -> Result<()> {
        let min_date = DateIndex::try_from(Date::MIN_RATIO).unwrap();

        self.sma
            .as_mut()
            .unwrap()
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    source,
                    self.days,
                    exit,
                    Some(min_date),
                )?;
                Ok(())
            })?;

        let sma_opt: Option<&EagerVec<PcoVec<DateIndex, StoredF32>>> = None;
        self.compute_rest(starting_indexes, exit, sma_opt, source)
    }

    pub fn compute_rest(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        sma_opt: Option<&impl IterableVec<DateIndex, StoredF32>>,
        source: &impl CollectableVec<DateIndex, StoredF32>,
    ) -> Result<()> {
        let sma = sma_opt
            .unwrap_or_else(|| unsafe { mem::transmute(&self.sma.as_ref().unwrap().dateindex) });

        let min_date = DateIndex::try_from(Date::MIN_RATIO).unwrap();

        let source_version = source.version();

        self.mut_stateful_date_vecs()
            .try_for_each(|v| -> Result<()> {
                v.validate_computed_version_or_reset(source_version)?;
                Ok(())
            })?;

        let starting_dateindex = self
            .mut_stateful_date_vecs()
            .map(|v| DateIndex::from(v.len()))
            .min()
            .unwrap()
            .min(starting_indexes.dateindex);

        let mut sorted = source.collect_range(
            Some(min_date.to_usize()),
            Some(starting_dateindex.to_usize()),
        );

        sorted.sort_unstable();

        let mut p0_5sd = self.p0_5sd.as_mut().map(|c| &mut c.dateindex);
        let mut p1sd = self.p1sd.as_mut().map(|c| &mut c.dateindex);
        let mut p1_5sd = self.p1_5sd.as_mut().map(|c| &mut c.dateindex);
        let mut p2sd = self.p2sd.as_mut().map(|c| &mut c.dateindex);
        let mut p2_5sd = self.p2_5sd.as_mut().map(|c| &mut c.dateindex);
        let mut p3sd = self.p3sd.as_mut().map(|c| &mut c.dateindex);
        let mut m0_5sd = self.m0_5sd.as_mut().map(|c| &mut c.dateindex);
        let mut m1sd = self.m1sd.as_mut().map(|c| &mut c.dateindex);
        let mut m1_5sd = self.m1_5sd.as_mut().map(|c| &mut c.dateindex);
        let mut m2sd = self.m2sd.as_mut().map(|c| &mut c.dateindex);
        let mut m2_5sd = self.m2_5sd.as_mut().map(|c| &mut c.dateindex);
        let mut m3sd = self.m3sd.as_mut().map(|c| &mut c.dateindex);

        let min_date_usize = min_date.to_usize();
        let mut sma_iter = sma.iter().skip(starting_dateindex.to_usize());

        source
            .iter()
            .enumerate()
            .skip(starting_dateindex.to_usize())
            .try_for_each(|(index, ratio)| -> Result<()> {
                if index < min_date_usize {
                    self.sd.dateindex.truncate_push_at(index, StoredF32::NAN)?;

                    if let Some(v) = p0_5sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = p1sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = p1_5sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = p2sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = p2_5sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = p3sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = m0_5sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = m1sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = m1_5sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = m2sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = m2_5sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    if let Some(v) = m3sd.as_mut() {
                        v.truncate_push_at(index, StoredF32::NAN)?
                    }
                    // Advance iterator to stay in sync
                    sma_iter.next();
                } else {
                    let pos = sorted.binary_search(&ratio).unwrap_or_else(|pos| pos);
                    sorted.insert(pos, ratio);

                    let average = sma_iter.next().unwrap();

                    let population =
                        index.checked_sub(min_date_usize).unwrap().to_usize() as f32 + 1.0;

                    let sd = StoredF32::from(
                        (sorted.iter().map(|v| (**v - *average).powi(2)).sum::<f32>() / population)
                            .sqrt(),
                    );

                    self.sd.dateindex.truncate_push_at(index, sd)?;
                    if let Some(v) = p0_5sd.as_mut() {
                        v.truncate_push_at(index, average + StoredF32::from(0.5 * *sd))?
                    }
                    if let Some(v) = p1sd.as_mut() {
                        v.truncate_push_at(index, average + sd)?
                    }
                    if let Some(v) = p1_5sd.as_mut() {
                        v.truncate_push_at(index, average + StoredF32::from(1.5 * *sd))?
                    }
                    if let Some(v) = p2sd.as_mut() {
                        v.truncate_push_at(index, average + 2 * sd)?
                    }
                    if let Some(v) = p2_5sd.as_mut() {
                        v.truncate_push_at(index, average + StoredF32::from(2.5 * *sd))?
                    }
                    if let Some(v) = p3sd.as_mut() {
                        v.truncate_push_at(index, average + 3 * sd)?
                    }
                    if let Some(v) = m0_5sd.as_mut() {
                        v.truncate_push_at(index, average - StoredF32::from(0.5 * *sd))?
                    }
                    if let Some(v) = m1sd.as_mut() {
                        v.truncate_push_at(index, average - sd)?
                    }
                    if let Some(v) = m1_5sd.as_mut() {
                        v.truncate_push_at(index, average - StoredF32::from(1.5 * *sd))?
                    }
                    if let Some(v) = m2sd.as_mut() {
                        v.truncate_push_at(index, average - 2 * sd)?
                    }
                    if let Some(v) = m2_5sd.as_mut() {
                        v.truncate_push_at(index, average - StoredF32::from(2.5 * *sd))?
                    }
                    if let Some(v) = m3sd.as_mut() {
                        v.truncate_push_at(index, average - 3 * sd)?
                    }
                }

                Ok(())
            })?;

        drop(sma_iter);

        {
            let _lock = exit.lock();
            self.mut_stateful_date_vecs().try_for_each(|v| v.flush())?;
        }

        self.mut_stateful_computed().try_for_each(|v| {
            v.compute_rest(
                starting_indexes,
                exit,
                None as Option<&EagerVec<PcoVec<_, _>>>,
            )
        })?;

        if let Some(zscore) = self.zscore.as_mut() {
            zscore.compute_all(starting_indexes, exit, |vec| {
                vec.compute_zscore(
                    starting_indexes.dateindex,
                    source,
                    sma,
                    &self.sd.dateindex,
                    exit,
                )?;
                Ok(())
            })?;
        }

        Ok(())
    }

    fn mut_stateful_computed(&mut self) -> impl Iterator<Item = &mut ComputedDateLast<StoredF32>> {
        [
            Some(&mut self.sd),
            self.p0_5sd.as_mut(),
            self.p1sd.as_mut(),
            self.p1_5sd.as_mut(),
            self.p2sd.as_mut(),
            self.p2_5sd.as_mut(),
            self.p3sd.as_mut(),
            self.m0_5sd.as_mut(),
            self.m1sd.as_mut(),
            self.m1_5sd.as_mut(),
            self.m2sd.as_mut(),
            self.m2_5sd.as_mut(),
            self.m3sd.as_mut(),
        ]
        .into_iter()
        .flatten()
    }

    fn mut_stateful_date_vecs(
        &mut self,
    ) -> impl Iterator<Item = &mut EagerVec<PcoVec<DateIndex, StoredF32>>> {
        self.mut_stateful_computed().map(|c| &mut c.dateindex)
    }
}
