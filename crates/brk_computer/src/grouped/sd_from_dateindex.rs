use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{CheckedSub, Date, DateIndex, Dollars, StoredF32, Version};
use vecdb::{
    AnyCollectableVec, AnyIterableVec, AnyStoredVec, AnyVec, BoxedVecIterator, CollectableVec,
    Database, EagerVec, Exit, GenericStoredVec, StoredIndex,
};

use crate::{Indexes, grouped::source::Source, indexes};

use super::{ComputedVecsFromDateIndex, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedStandardDeviationVecsFromDateIndex {
    days: usize,

    pub sma: Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub sd: ComputedVecsFromDateIndex<StoredF32>,
    pub _0sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub p0_5sd: ComputedVecsFromDateIndex<StoredF32>,
    pub p1sd: ComputedVecsFromDateIndex<StoredF32>,
    pub p1_5sd: ComputedVecsFromDateIndex<StoredF32>,
    pub p2sd: ComputedVecsFromDateIndex<StoredF32>,
    pub p2_5sd: ComputedVecsFromDateIndex<StoredF32>,
    pub p3sd: ComputedVecsFromDateIndex<StoredF32>,
    pub m0_5sd: ComputedVecsFromDateIndex<StoredF32>,
    pub m1sd: ComputedVecsFromDateIndex<StoredF32>,
    pub m1_5sd: ComputedVecsFromDateIndex<StoredF32>,
    pub m2sd: ComputedVecsFromDateIndex<StoredF32>,
    pub m2_5sd: ComputedVecsFromDateIndex<StoredF32>,
    pub m3sd: ComputedVecsFromDateIndex<StoredF32>,
    pub p0_5sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub p1sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub p1_5sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub p2sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub p2_5sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub p3sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub m0_5sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub m1sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub m1_5sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub m2sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub m2_5sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub m3sd_as_price: ComputedVecsFromDateIndex<Dollars>,
    pub zscore: ComputedVecsFromDateIndex<StoredF32>,
}

const VERSION: Version = Version::ONE;

impl ComputedStandardDeviationVecsFromDateIndex {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        days: usize,
        source: Source<DateIndex, StoredF32>,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let options = VecBuilderOptions::default().add_last();

        Ok(Self {
            days,
            sma: source.is_compute().then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &format!("{name}_sma"),
                    Source::Compute,
                    version + VERSION + Version::ZERO,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p0_5sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p0_5sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p1sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p1sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p1_5sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p1_5sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p2sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p2sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p2_5sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p2_5sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p3sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p3sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m0_5sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m0_5sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m1sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m1sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m1_5sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m1_5sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m2sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m2sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m2_5sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m2_5sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m3sd: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m3sd"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            _0sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_0sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p0_5sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p0_5sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p1sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p1sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p1_5sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p1_5sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p2sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p2sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p2_5sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p2_5sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            p3sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_p3sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m0_5sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m0_5sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m1sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m1sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m1_5sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m1_5sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m2sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m2sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m2_5sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m2_5sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            m3sd_as_price: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_m3sd_as_price"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
            zscore: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_zscore"),
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                options,
            )?,
        })
    }

    pub fn compute_all(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        source: &impl CollectableVec<DateIndex, StoredF32>,
        source_as_price: Option<&impl AnyIterableVec<DateIndex, Dollars>>,
    ) -> Result<()> {
        let min_date = DateIndex::try_from(Date::MIN_RATIO).unwrap();

        self.sma.as_mut().unwrap().compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma_(
                    starting_indexes.dateindex,
                    source,
                    self.days,
                    exit,
                    Some(min_date),
                )?;
                Ok(())
            },
        )?;

        let sma_opt: Option<&EagerVec<DateIndex, StoredF32>> = None;
        self.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            sma_opt,
            source,
            source_as_price,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        sma_opt: Option<&impl AnyIterableVec<DateIndex, StoredF32>>,
        source: &impl CollectableVec<DateIndex, StoredF32>,
        source_as_price: Option<&impl AnyIterableVec<DateIndex, Dollars>>,
    ) -> Result<()> {
        let sma = sma_opt.unwrap_or_else(|| unsafe {
            std::mem::transmute(&self.sma.as_ref().unwrap().dateindex)
        });

        let min_date = DateIndex::try_from(Date::MIN_RATIO).unwrap();

        let source_version = source.version();

        self.mut_vecs().iter_mut().try_for_each(|v| -> Result<()> {
            v.validate_computed_version_or_reset(
                Version::ZERO + v.inner_version() + source_version,
            )?;
            Ok(())
        })?;

        let starting_dateindex = self
            .mut_vecs()
            .iter()
            .map(|v| DateIndex::from(v.len()))
            .min()
            .unwrap()
            .min(starting_indexes.dateindex);

        let mut sorted = source.collect_range(
            Some(min_date.unwrap_to_usize()),
            Some(starting_dateindex.unwrap_to_usize()),
        )?;

        sorted.sort_unstable();

        let mut sma_iter = sma.iter();

        source
            .iter_at(starting_dateindex)
            .try_for_each(|(index, ratio)| -> Result<()> {
                if index < min_date {
                    self.sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;

                    self.p0_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.p1sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.p1_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.p2sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.p2_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.p3sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.m0_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.m1sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.m1_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.m2sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.m2_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                    self.m3sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        StoredF32::NAN,
                        exit,
                    )?;
                } else {
                    let ratio = ratio.into_owned();
                    let pos = sorted.binary_search(&ratio).unwrap_or_else(|pos| pos);
                    sorted.insert(pos, ratio);

                    let avg = sma_iter.unwrap_get_inner(index);

                    let population =
                        index.checked_sub(min_date).unwrap().unwrap_to_usize() as f32 + 1.0;

                    let sd = StoredF32::from(
                        (sorted.iter().map(|v| (**v - *avg).powi(2)).sum::<f32>() / population)
                            .sqrt(),
                    );

                    self.sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, sd, exit)?;
                    self.p0_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg + StoredF32::from(0.5 * *sd),
                        exit,
                    )?;
                    self.p1sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, avg + sd, exit)?;
                    self.p1_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg + StoredF32::from(1.5 * *sd),
                        exit,
                    )?;
                    self.p2sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg + 2 * sd,
                        exit,
                    )?;
                    self.p2_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg + StoredF32::from(2.5 * *sd),
                        exit,
                    )?;
                    self.p3sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg + 3 * sd,
                        exit,
                    )?;
                    self.m0_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg - StoredF32::from(0.5 * *sd),
                        exit,
                    )?;
                    self.m1sd
                        .dateindex
                        .as_mut()
                        .unwrap()
                        .forced_push_at(index, avg - sd, exit)?;
                    self.m1_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg - StoredF32::from(1.5 * *sd),
                        exit,
                    )?;
                    self.m2sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg - 2 * sd,
                        exit,
                    )?;
                    self.m2_5sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg - StoredF32::from(2.5 * *sd),
                        exit,
                    )?;
                    self.m3sd.dateindex.as_mut().unwrap().forced_push_at(
                        index,
                        avg - 3 * sd,
                        exit,
                    )?;
                }

                Ok(())
            })?;

        drop(sma_iter);

        self.mut_vecs()
            .into_iter()
            .try_for_each(|v| v.safe_flush(exit))?;

        [
            &mut self.sd,
            &mut self.p0_5sd,
            &mut self.p1sd,
            &mut self.p1_5sd,
            &mut self.p2sd,
            &mut self.p2_5sd,
            &mut self.p3sd,
            &mut self.m0_5sd,
            &mut self.m1sd,
            &mut self.m1_5sd,
            &mut self.m2sd,
            &mut self.m2_5sd,
            &mut self.m3sd,
        ]
        .into_iter()
        .try_for_each(|v| {
            v.compute_rest(starting_indexes, exit, None as Option<&EagerVec<_, _>>)
        })?;

        self.zscore.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, _, starting_indexes, exit| {
                vec.compute_zscore(
                    starting_indexes.dateindex,
                    source,
                    sma,
                    self.sd.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            },
        )?;

        let Some(price) = source_as_price else {
            return Ok(());
        };

        let compute_as_price =
            |as_price: &mut ComputedVecsFromDateIndex<Dollars>,
             mut iter: BoxedVecIterator<DateIndex, StoredF32>| {
                as_price.compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |vec, _, _, starting_indexes, exit| {
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
                    },
                )
            };

        compute_as_price(&mut self._0sd_as_price, sma.iter())?;
        compute_as_price(
            &mut self.p0_5sd_as_price,
            self.p0_5sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.p1sd_as_price,
            self.p1sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.p1_5sd_as_price,
            self.p1_5sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.p2sd_as_price,
            self.p2sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.p2_5sd_as_price,
            self.p2_5sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.p3sd_as_price,
            self.p3sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.m0_5sd_as_price,
            self.m0_5sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.m1sd_as_price,
            self.m1sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.m1_5sd_as_price,
            self.m1_5sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.m2sd_as_price,
            self.m2sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.m2_5sd_as_price,
            self.m2_5sd.dateindex.as_ref().unwrap().iter(),
        )?;
        compute_as_price(
            &mut self.m3sd_as_price,
            self.m3sd.dateindex.as_ref().unwrap().iter(),
        )?;

        Ok(())
    }

    fn mut_vecs(&mut self) -> [&mut EagerVec<DateIndex, StoredF32>; 13] {
        [
            self.sd.dateindex.as_mut().unwrap(),
            self.p0_5sd.dateindex.as_mut().unwrap(),
            self.p1sd.dateindex.as_mut().unwrap(),
            self.p1_5sd.dateindex.as_mut().unwrap(),
            self.p2sd.dateindex.as_mut().unwrap(),
            self.p2_5sd.dateindex.as_mut().unwrap(),
            self.p3sd.dateindex.as_mut().unwrap(),
            self.m0_5sd.dateindex.as_mut().unwrap(),
            self.m1sd.dateindex.as_mut().unwrap(),
            self.m1_5sd.dateindex.as_mut().unwrap(),
            self.m2sd.dateindex.as_mut().unwrap(),
            self.m2_5sd.dateindex.as_mut().unwrap(),
            self.m3sd.dateindex.as_mut().unwrap(),
        ]
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.sma.as_ref().map_or(vec![], |v| v.vecs()),
            self.sd.vecs(),
            self.p0_5sd.vecs(),
            self.p1sd.vecs(),
            self.p1_5sd.vecs(),
            self.p2sd.vecs(),
            self.p2_5sd.vecs(),
            self.p3sd.vecs(),
            self.m0_5sd.vecs(),
            self.m1sd.vecs(),
            self.m1_5sd.vecs(),
            self.m2sd.vecs(),
            self.m2_5sd.vecs(),
            self.m3sd.vecs(),
            self._0sd_as_price.vecs(),
            self.p0_5sd_as_price.vecs(),
            self.p1sd_as_price.vecs(),
            self.p1_5sd_as_price.vecs(),
            self.p2sd_as_price.vecs(),
            self.p2_5sd_as_price.vecs(),
            self.p3sd_as_price.vecs(),
            self.m0_5sd_as_price.vecs(),
            self.m1sd_as_price.vecs(),
            self.m1_5sd_as_price.vecs(),
            self.m2sd_as_price.vecs(),
            self.m2_5sd_as_price.vecs(),
            self.m3sd_as_price.vecs(),
            self.zscore.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
