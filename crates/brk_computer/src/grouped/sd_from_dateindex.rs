use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Date, DateIndex, Dollars, StoredF32, Version};
use vecdb::{PcoVec, 
    AnyStoredVec, AnyVec, BoxedVecIterator, CollectableVec, Database, EagerVec, Exit,
    GenericStoredVec, IterableVec, VecIndex,
};

use crate::{Indexes, grouped::source::Source, indexes, utils::OptionExt};

use super::{ComputedVecsFromDateIndex, VecBuilderOptions};

#[derive(Clone, Traversable)]
pub struct ComputedStandardDeviationVecsFromDateIndex {
    days: usize,

    pub sma: Option<ComputedVecsFromDateIndex<StoredF32>>,

    pub sd: ComputedVecsFromDateIndex<StoredF32>,

    pub zscore: Option<ComputedVecsFromDateIndex<StoredF32>>,

    pub p0_5sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub p1sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub p1_5sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub p2sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub p2_5sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub p3sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub m0_5sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub m1sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub m1_5sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub m2sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub m2_5sd: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub m3sd: Option<ComputedVecsFromDateIndex<StoredF32>>,

    pub _0sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub p0_5sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub p1sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub p1_5sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub p2sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub p2_5sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub p3sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub m0_5sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub m1sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub m1_5sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub m2sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub m2_5sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
    pub m3sd_usd: Option<ComputedVecsFromDateIndex<Dollars>>,
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

impl ComputedStandardDeviationVecsFromDateIndex {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        days: usize,
        sma: Source<DateIndex, StoredF32>,
        parent_version: Version,
        indexes: &indexes::Vecs,
        options: StandardDeviationVecsOptions,
    ) -> Result<Self> {
        let opts = VecBuilderOptions::default().add_last();
        let version = parent_version + Version::ONE;

        macro_rules! import {
            ($suffix:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    db, &format!("{name}_{}", $suffix), Source::Compute, version, indexes, opts,
                ).unwrap()
            };
        }

        Ok(Self {
            days,
            sma: sma.is_compute().then(|| import!("sma")),
            sd: import!("sd"),
            p0_5sd: options.bands().then(|| import!("p0_5sd")),
            p1sd: options.bands().then(|| import!("p1sd")),
            p1_5sd: options.bands().then(|| import!("p1_5sd")),
            p2sd: options.bands().then(|| import!("p2sd")),
            p2_5sd: options.bands().then(|| import!("p2_5sd")),
            p3sd: options.bands().then(|| import!("p3sd")),
            m0_5sd: options.bands().then(|| import!("m0_5sd")),
            m1sd: options.bands().then(|| import!("m1sd")),
            m1_5sd: options.bands().then(|| import!("m1_5sd")),
            m2sd: options.bands().then(|| import!("m2sd")),
            m2_5sd: options.bands().then(|| import!("m2_5sd")),
            m3sd: options.bands().then(|| import!("m3sd")),
            _0sd_usd: options.price_bands().then(|| import!("0sd_usd")),
            p0_5sd_usd: options.price_bands().then(|| import!("p0_5sd_usd")),
            p1sd_usd: options.price_bands().then(|| import!("p1sd_usd")),
            p1_5sd_usd: options.price_bands().then(|| import!("p1_5sd_usd")),
            p2sd_usd: options.price_bands().then(|| import!("p2sd_usd")),
            p2_5sd_usd: options.price_bands().then(|| import!("p2_5sd_usd")),
            p3sd_usd: options.price_bands().then(|| import!("p3sd_usd")),
            m0_5sd_usd: options.price_bands().then(|| import!("m0_5sd_usd")),
            m1sd_usd: options.price_bands().then(|| import!("m1sd_usd")),
            m1_5sd_usd: options.price_bands().then(|| import!("m1_5sd_usd")),
            m2sd_usd: options.price_bands().then(|| import!("m2sd_usd")),
            m2_5sd_usd: options.price_bands().then(|| import!("m2_5sd_usd")),
            m3sd_usd: options.price_bands().then(|| import!("m3sd_usd")),
            zscore: options.zscore().then(|| import!("zscore")),
        })
    }

    pub fn compute_all(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        source: &impl CollectableVec<DateIndex, StoredF32>,
        price_opt: Option<&impl IterableVec<DateIndex, Dollars>>,
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
        self.compute_rest(starting_indexes, exit, sma_opt, source, price_opt)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        sma_opt: Option<&impl IterableVec<DateIndex, StoredF32>>,
        source: &impl CollectableVec<DateIndex, StoredF32>,
        price_opt: Option<&impl IterableVec<DateIndex, Dollars>>,
    ) -> Result<()> {
        let sma = sma_opt.unwrap_or_else(|| unsafe {
            std::mem::transmute(&self.sma.u().dateindex)
        });

        let min_date = DateIndex::try_from(Date::MIN_RATIO).unwrap();

        let source_version = source.version();

        self.mut_stateful_date_vecs()
            .try_for_each(|v| -> Result<()> {
                v.validate_computed_version_or_reset(
                    Version::ZERO + v.inner_version() + source_version,
                )?;
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

        let mut p0_5sd = self.p0_5sd.as_mut().map(|c| c.dateindex.um());
        let mut p1sd = self.p1sd.as_mut().map(|c| c.dateindex.um());
        let mut p1_5sd = self.p1_5sd.as_mut().map(|c| c.dateindex.um());
        let mut p2sd = self.p2sd.as_mut().map(|c| c.dateindex.um());
        let mut p2_5sd = self.p2_5sd.as_mut().map(|c| c.dateindex.um());
        let mut p3sd = self.p3sd.as_mut().map(|c| c.dateindex.um());
        let mut m0_5sd = self.m0_5sd.as_mut().map(|c| c.dateindex.um());
        let mut m1sd = self.m1sd.as_mut().map(|c| c.dateindex.um());
        let mut m1_5sd = self.m1_5sd.as_mut().map(|c| c.dateindex.um());
        let mut m2sd = self.m2sd.as_mut().map(|c| c.dateindex.um());
        let mut m2_5sd = self.m2_5sd.as_mut().map(|c| c.dateindex.um());
        let mut m3sd = self.m3sd.as_mut().map(|c| c.dateindex.um());

        let min_date_usize = min_date.to_usize();
        let mut sma_iter = sma.iter().skip(starting_dateindex.to_usize());

        source
            .iter()
            .enumerate()
            .skip(starting_dateindex.to_usize())
            .try_for_each(|(index, ratio)| -> Result<()> {
                if index < min_date_usize {
                    self.sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, StoredF32::NAN)?;

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

                    let avg = sma_iter.next().unwrap();

                    let population =
                        index.checked_sub(min_date_usize).unwrap().to_usize() as f32 + 1.0;

                    let sd = StoredF32::from(
                        (sorted.iter().map(|v| (**v - *avg).powi(2)).sum::<f32>() / population)
                            .sqrt(),
                    );

                    self.sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .truncate_push_at(index, sd)?;
                    if let Some(v) = p0_5sd.as_mut() {
                        v.truncate_push_at(index, avg + StoredF32::from(0.5 * *sd))?
                    }
                    if let Some(v) = p1sd.as_mut() {
                        v.truncate_push_at(index, avg + sd)?
                    }
                    if let Some(v) = p1_5sd.as_mut() {
                        v.truncate_push_at(index, avg + StoredF32::from(1.5 * *sd))?
                    }
                    if let Some(v) = p2sd.as_mut() {
                        v.truncate_push_at(index, avg + 2 * sd)?
                    }
                    if let Some(v) = p2_5sd.as_mut() {
                        v.truncate_push_at(index, avg + StoredF32::from(2.5 * *sd))?
                    }
                    if let Some(v) = p3sd.as_mut() {
                        v.truncate_push_at(index, avg + 3 * sd)?
                    }
                    if let Some(v) = m0_5sd.as_mut() {
                        v.truncate_push_at(index, avg - StoredF32::from(0.5 * *sd))?
                    }
                    if let Some(v) = m1sd.as_mut() {
                        v.truncate_push_at(index, avg - sd)?
                    }
                    if let Some(v) = m1_5sd.as_mut() {
                        v.truncate_push_at(index, avg - StoredF32::from(1.5 * *sd))?
                    }
                    if let Some(v) = m2sd.as_mut() {
                        v.truncate_push_at(index, avg - 2 * sd)?
                    }
                    if let Some(v) = m2_5sd.as_mut() {
                        v.truncate_push_at(index, avg - StoredF32::from(2.5 * *sd))?
                    }
                    if let Some(v) = m3sd.as_mut() {
                        v.truncate_push_at(index, avg - 3 * sd)?
                    }
                }

                Ok(())
            })?;

        drop(sma_iter);

        self.mut_stateful_date_vecs()
            .try_for_each(|v| v.safe_flush(exit))?;

        self.mut_stateful_computed().try_for_each(|v| {
            v.compute_rest(starting_indexes, exit, None as Option<&EagerVec<PcoVec<_, _>>>)
        })?;

        if let Some(zscore) = self.zscore.as_mut() {
            zscore.compute_all(starting_indexes, exit, |vec| {
                vec.compute_zscore(
                    starting_indexes.dateindex,
                    source,
                    sma,
                    self.sd.dateindex.u(),
                    exit,
                )?;
                Ok(())
            })?;
        }

        let Some(price) = price_opt else {
            return Ok(());
        };

        let compute_usd =
            |usd: &mut ComputedVecsFromDateIndex<Dollars>,
             mut iter: BoxedVecIterator<DateIndex, StoredF32>| {
                usd.compute_all(starting_indexes, exit, |vec| {
                    vec.compute_transform(
                        starting_indexes.dateindex,
                        price,
                        |(i, price, ..)| {
                            let multiplier = iter.get_unwrap(i);
                            (i, price * multiplier)
                        },
                        exit,
                    )?;
                    Ok(())
                })
            };

        if self._0sd_usd.is_none() {
            return Ok(());
        }

        compute_usd(self._0sd_usd.um(), sma.iter())?;
        compute_usd(
            self.p0_5sd_usd.um(),
            self.p0_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.p1sd_usd.um(),
            self.p1sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.p1_5sd_usd.um(),
            self.p1_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.p2sd_usd.um(),
            self.p2sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.p2_5sd_usd.um(),
            self.p2_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.p3sd_usd.um(),
            self.p3sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m0_5sd_usd.um(),
            self.m0_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m1sd_usd.um(),
            self.m1sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m1_5sd_usd.um(),
            self.m1_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m2sd_usd.um(),
            self.m2sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m2_5sd_usd.um(),
            self.m2_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m3sd_usd.um(),
            self.m3sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;

        Ok(())
    }

    fn mut_stateful_computed(
        &mut self,
    ) -> impl Iterator<Item = &mut ComputedVecsFromDateIndex<StoredF32>> {
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
        self.mut_stateful_computed()
            .map(|c| c.dateindex.um())
    }
}
