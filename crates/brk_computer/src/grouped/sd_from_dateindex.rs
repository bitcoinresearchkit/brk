use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{CheckedSub, Date, DateIndex, Dollars, StoredF32, Version};
use vecdb::{
    AnyIterableVec, AnyStoredVec, AnyVec, BoxedVecIterator, CollectableVec, Database, EagerVec,
    Exit, GenericStoredVec, StoredIndex,
};

use crate::{Indexes, grouped::source::Source, indexes};

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
        let builder_options = VecBuilderOptions::default().add_last();

        let version = parent_version + Version::ONE;

        Ok(Self {
            days,
            sma: sma.is_compute().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_sma"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_sd"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                builder_options,
            )?,
            p0_5sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p0_5sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p1sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p1sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p1_5sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p1_5sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p2sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p2sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p2_5sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p2_5sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p3sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p3sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m0_5sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m0_5sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m1sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m1sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m1_5sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m1_5sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m2sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m2sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m2_5sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m2_5sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m3sd: options.bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m3sd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            _0sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_0sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p0_5sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p0_5sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p1sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p1sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p1_5sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p1_5sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p2sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p2sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p2_5sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p2_5sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            p3sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_p3sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m0_5sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m0_5sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m1sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m1sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m1_5sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m1_5sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m2sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m2sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m2_5sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m2_5sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            m3sd_usd: options.price_bands().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_m3sd_usd"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
            zscore: options.zscore().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_zscore"),
                    Source::Compute,
                    version + Version::ZERO,
                    indexes,
                    builder_options,
                )
                .unwrap()
            }),
        })
    }

    pub fn compute_all(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        source: &impl CollectableVec<DateIndex, StoredF32>,
        price_opt: Option<&impl AnyIterableVec<DateIndex, Dollars>>,
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

        let sma_opt: Option<&EagerVec<DateIndex, StoredF32>> = None;
        self.compute_rest(starting_indexes, exit, sma_opt, source, price_opt)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        sma_opt: Option<&impl AnyIterableVec<DateIndex, StoredF32>>,
        source: &impl CollectableVec<DateIndex, StoredF32>,
        price_opt: Option<&impl AnyIterableVec<DateIndex, Dollars>>,
    ) -> Result<()> {
        let sma = sma_opt.unwrap_or_else(|| unsafe {
            std::mem::transmute(&self.sma.as_ref().unwrap().dateindex)
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

        let mut sma_iter = sma.iter();

        let mut p0_5sd = self.p0_5sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut p1sd = self.p1sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut p1_5sd = self.p1_5sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut p2sd = self.p2sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut p2_5sd = self.p2_5sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut p3sd = self.p3sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut m0_5sd = self.m0_5sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut m1sd = self.m1sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut m1_5sd = self.m1_5sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut m2sd = self.m2sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut m2_5sd = self.m2_5sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());
        let mut m3sd = self.m3sd.as_mut().map(|c| c.dateindex.as_mut().unwrap());

        source
            .iter_at(starting_dateindex)
            .try_for_each(|(index, ratio)| -> Result<()> {
                if index < min_date {
                    self.sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;

                    if let Some(v) = p0_5sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = p1sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = p1_5sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = p2sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = p2_5sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = p3sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = m0_5sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = m1sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = m1_5sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = m2sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = m2_5sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                    if let Some(v) = m3sd.as_mut() {
                        v.forced_push_at(index, StoredF32::NAN, exit)?
                    }
                } else {
                    let pos = sorted.binary_search(&ratio).unwrap_or_else(|pos| pos);
                    sorted.insert(pos, ratio);

                    let avg = sma_iter.unwrap_get_inner(index);

                    let population = index.checked_sub(min_date).unwrap().to_usize() as f32 + 1.0;

                    let sd = StoredF32::from(
                        (sorted.iter().map(|v| (**v - *avg).powi(2)).sum::<f32>() / population)
                            .sqrt(),
                    );

                    self.sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, sd, exit)?;
                    if let Some(v) = p0_5sd.as_mut() {
                        v.forced_push_at(index, avg + StoredF32::from(0.5 * *sd), exit)?
                    }
                    if let Some(v) = p1sd.as_mut() {
                        v.forced_push_at(index, avg + sd, exit)?
                    }
                    if let Some(v) = p1_5sd.as_mut() {
                        v.forced_push_at(index, avg + StoredF32::from(1.5 * *sd), exit)?
                    }
                    if let Some(v) = p2sd.as_mut() {
                        v.forced_push_at(index, avg + 2 * sd, exit)?
                    }
                    if let Some(v) = p2_5sd.as_mut() {
                        v.forced_push_at(index, avg + StoredF32::from(2.5 * *sd), exit)?
                    }
                    if let Some(v) = p3sd.as_mut() {
                        v.forced_push_at(index, avg + 3 * sd, exit)?
                    }
                    if let Some(v) = m0_5sd.as_mut() {
                        v.forced_push_at(index, avg - StoredF32::from(0.5 * *sd), exit)?
                    }
                    if let Some(v) = m1sd.as_mut() {
                        v.forced_push_at(index, avg - sd, exit)?
                    }
                    if let Some(v) = m1_5sd.as_mut() {
                        v.forced_push_at(index, avg - StoredF32::from(1.5 * *sd), exit)?
                    }
                    if let Some(v) = m2sd.as_mut() {
                        v.forced_push_at(index, avg - 2 * sd, exit)?
                    }
                    if let Some(v) = m2_5sd.as_mut() {
                        v.forced_push_at(index, avg - StoredF32::from(2.5 * *sd), exit)?
                    }
                    if let Some(v) = m3sd.as_mut() {
                        v.forced_push_at(index, avg - 3 * sd, exit)?
                    }
                }

                Ok(())
            })?;

        drop(sma_iter);

        self.mut_stateful_date_vecs()
            .try_for_each(|v| v.safe_flush(exit))?;

        self.mut_stateful_computed().try_for_each(|v| {
            v.compute_rest(starting_indexes, exit, None as Option<&EagerVec<_, _>>)
        })?;

        if let Some(zscore) = self.zscore.as_mut() {
            zscore.compute_all(starting_indexes, exit, |vec| {
                vec.compute_zscore(
                    starting_indexes.dateindex,
                    source,
                    sma,
                    self.sd.dateindex.as_ref().unwrap(),
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
                            let multiplier = iter.unwrap_get_inner(i);
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

        compute_usd(self._0sd_usd.as_mut().unwrap(), sma.iter())?;
        compute_usd(
            self.p0_5sd_usd.as_mut().unwrap(),
            self.p0_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.p1sd_usd.as_mut().unwrap(),
            self.p1sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.p1_5sd_usd.as_mut().unwrap(),
            self.p1_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.p2sd_usd.as_mut().unwrap(),
            self.p2sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.p2_5sd_usd.as_mut().unwrap(),
            self.p2_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.p3sd_usd.as_mut().unwrap(),
            self.p3sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m0_5sd_usd.as_mut().unwrap(),
            self.m0_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m1sd_usd.as_mut().unwrap(),
            self.m1sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m1_5sd_usd.as_mut().unwrap(),
            self.m1_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m2sd_usd.as_mut().unwrap(),
            self.m2sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m2_5sd_usd.as_mut().unwrap(),
            self.m2_5sd
                .as_ref()
                .unwrap()
                .dateindex
                .as_ref()
                .unwrap()
                .iter(),
        )?;
        compute_usd(
            self.m3sd_usd.as_mut().unwrap(),
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
    ) -> impl Iterator<Item = &mut EagerVec<DateIndex, StoredF32>> {
        self.mut_stateful_computed()
            .map(|c| c.dateindex.as_mut().unwrap())
    }
}
