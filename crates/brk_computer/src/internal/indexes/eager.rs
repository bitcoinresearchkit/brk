use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, Epoch, Halving, Height, Hour1, Hour4, Hour12, Indexes, Minute10, Minute30, Month1,
    Month3, Month6, Version, Week1, Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    AnyVec, Database, EagerVec, Exit, ImportableVec, PcoVec, ReadableVec, Rw, StorageMode,
    VecIndex, WritableVec,
};

use crate::{
    indexes,
    internal::{ComputedVecValue, NumericValue, PerResolution},
};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct EagerIndexes<T, M: StorageMode = Rw>(
    #[allow(clippy::type_complexity)]
    pub  PerResolution<
        <M as StorageMode>::Stored<EagerVec<PcoVec<Minute10, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Minute30, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Hour1, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Hour4, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Hour12, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Day1, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Day3, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Week1, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Month1, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Month3, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Month6, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Year1, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Year10, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Halving, T>>>,
        <M as StorageMode>::Stored<EagerVec<PcoVec<Epoch, T>>>,
    >,
)
where
    T: ComputedVecValue + PartialOrd + JsonSchema;

impl<T> EagerIndexes<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        macro_rules! per_period {
            () => {
                ImportableVec::forced_import(db, name, version)?
            };
        }

        Ok(Self(PerResolution {
            minute10: per_period!(),
            minute30: per_period!(),
            hour1: per_period!(),
            hour4: per_period!(),
            hour12: per_period!(),
            day1: per_period!(),
            day3: per_period!(),
            week1: per_period!(),
            month1: per_period!(),
            month3: per_period!(),
            month6: per_period!(),
            year1: per_period!(),
            year10: per_period!(),
            halving: per_period!(),
            epoch: per_period!(),
        }))
    }

    pub(crate) fn compute_first(
        &mut self,
        starting_indexes: &Indexes,
        height_source: &impl ReadableVec<Height, T>,
        indexes: &indexes::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let prev_height = starting_indexes.height.decremented().unwrap_or_default();

        macro_rules! period {
            ($field:ident) => {
                self.0.$field.compute_indirect_sequential(
                    indexes
                        .height
                        .$field
                        .collect_one(prev_height)
                        .unwrap_or_default(),
                    &indexes.$field.first_height,
                    height_source,
                    exit,
                )?;
            };
        }

        period!(minute10);
        period!(minute30);
        period!(hour1);
        period!(hour4);
        period!(hour12);
        period!(day1);
        period!(day3);
        period!(week1);
        period!(month1);
        period!(month3);
        period!(month6);
        period!(year1);
        period!(year10);
        period!(halving);
        period!(epoch);

        Ok(())
    }

    pub(crate) fn compute_max(
        &mut self,
        starting_indexes: &Indexes,
        height_source: &impl ReadableVec<Height, T>,
        indexes: &indexes::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let src_len = height_source.len();
        let prev_height = starting_indexes.height.decremented().unwrap_or_default();

        macro_rules! period {
            ($field:ident) => {
                compute_period_extremum(
                    &mut self.0.$field,
                    indexes
                        .height
                        .$field
                        .collect_one(prev_height)
                        .unwrap_or_default(),
                    &indexes.$field.first_height,
                    height_source,
                    src_len,
                    T::max,
                    exit,
                )?;
            };
        }

        period!(minute10);
        period!(minute30);
        period!(hour1);
        period!(hour4);
        period!(hour12);
        period!(day1);
        period!(day3);
        period!(week1);
        period!(month1);
        period!(month3);
        period!(month6);
        period!(year1);
        period!(year10);
        period!(halving);
        period!(epoch);

        Ok(())
    }

    pub(crate) fn compute_min(
        &mut self,
        starting_indexes: &Indexes,
        height_source: &impl ReadableVec<Height, T>,
        indexes: &indexes::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let src_len = height_source.len();
        let prev_height = starting_indexes.height.decremented().unwrap_or_default();

        macro_rules! period {
            ($field:ident) => {
                compute_period_extremum(
                    &mut self.0.$field,
                    indexes
                        .height
                        .$field
                        .collect_one(prev_height)
                        .unwrap_or_default(),
                    &indexes.$field.first_height,
                    height_source,
                    src_len,
                    T::min,
                    exit,
                )?;
            };
        }

        period!(minute10);
        period!(minute30);
        period!(hour1);
        period!(hour4);
        period!(hour12);
        period!(day1);
        period!(day3);
        period!(week1);
        period!(month1);
        period!(month3);
        period!(month6);
        period!(year1);
        period!(year10);
        period!(halving);
        period!(epoch);

        Ok(())
    }
}

fn compute_period_extremum<I: VecIndex, T: ComputedVecValue + JsonSchema>(
    out: &mut EagerVec<PcoVec<I, T>>,
    starting_index: I,
    fh: &impl ReadableVec<I, Height>,
    height_source: &impl ReadableVec<Height, T>,
    src_len: usize,
    better: fn(T, T) -> T,
    exit: &Exit,
) -> Result<()> {
    out.validate_and_truncate(fh.version() + height_source.version(), starting_index)?;
    let mut cursor = height_source.cursor();
    Ok(out.repeat_until_complete(exit, |this| {
        let skip = this.len();
        let end = fh.len();
        if skip >= end {
            return Ok(());
        }

        let fh_batch: Vec<Height> = fh.collect_range_at(skip, (end + 1).min(fh.len()));

        if cursor.position() < fh_batch[0].to_usize() {
            cursor.advance(fh_batch[0].to_usize() - cursor.position());
        }

        for j in 0..(end - skip) {
            let first_h = fh_batch[j].to_usize();
            let end_h = fh_batch.get(j + 1).map_or(src_len, |h| h.to_usize());

            if cursor.position() < first_h {
                cursor.advance(first_h - cursor.position());
            }

            let range_len = end_h.saturating_sub(first_h);
            let v = if range_len > 0 {
                cursor
                    .fold(range_len, None, |acc, b| {
                        Some(match acc {
                            Some(a) => better(a, b),
                            None => b,
                        })
                    })
                    .unwrap_or_else(|| T::from(0_usize))
            } else {
                T::from(0_usize)
            };

            this.checked_push_at(skip + j, v)?;
        }

        Ok(())
    })?)
}
