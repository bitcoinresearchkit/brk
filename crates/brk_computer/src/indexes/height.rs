use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, Year10, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour12, Hour4,
    Minute1, Minute10, Minute30, Minute5, Month1, Month3, Month6, StoredU64, Version, Week1,
    Year1,
};
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, Rw, StorageMode};

use brk_error::Result;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub minute1: M::Stored<EagerVec<PcoVec<Height, Minute1>>>,
    pub minute5: M::Stored<EagerVec<PcoVec<Height, Minute5>>>,
    pub minute10: M::Stored<EagerVec<PcoVec<Height, Minute10>>>,
    pub minute30: M::Stored<EagerVec<PcoVec<Height, Minute30>>>,
    pub hour1: M::Stored<EagerVec<PcoVec<Height, Hour1>>>,
    pub hour4: M::Stored<EagerVec<PcoVec<Height, Hour4>>>,
    pub hour12: M::Stored<EagerVec<PcoVec<Height, Hour12>>>,
    pub day1: M::Stored<EagerVec<PcoVec<Height, Day1>>>,
    pub day3: M::Stored<EagerVec<PcoVec<Height, Day3>>>,
    pub difficultyepoch: M::Stored<EagerVec<PcoVec<Height, DifficultyEpoch>>>,
    pub halvingepoch: M::Stored<EagerVec<PcoVec<Height, HalvingEpoch>>>,
    pub week1: M::Stored<EagerVec<PcoVec<Height, Week1>>>,
    pub month1: M::Stored<EagerVec<PcoVec<Height, Month1>>>,
    pub month3: M::Stored<EagerVec<PcoVec<Height, Month3>>>,
    pub month6: M::Stored<EagerVec<PcoVec<Height, Month6>>>,
    pub year1: M::Stored<EagerVec<PcoVec<Height, Year1>>>,
    pub year10: M::Stored<EagerVec<PcoVec<Height, Year10>>>,
    pub txindex_count: M::Stored<EagerVec<PcoVec<Height, StoredU64>>>,
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, "height", version)?,
            minute1: EagerVec::forced_import(db, "minute1", version)?,
            minute5: EagerVec::forced_import(db, "minute5", version)?,
            minute10: EagerVec::forced_import(db, "minute10", version)?,
            minute30: EagerVec::forced_import(db, "minute30", version)?,
            hour1: EagerVec::forced_import(db, "hour1", version)?,
            hour4: EagerVec::forced_import(db, "hour4", version)?,
            hour12: EagerVec::forced_import(db, "hour12", version)?,
            day1: EagerVec::forced_import(db, "day1", version)?,
            day3: EagerVec::forced_import(db, "day3", version)?,
            difficultyepoch: EagerVec::forced_import(db, "difficultyepoch", version)?,
            halvingepoch: EagerVec::forced_import(db, "halvingepoch", version)?,
            week1: EagerVec::forced_import(db, "week1", version)?,
            month1: EagerVec::forced_import(db, "month1", version)?,
            month3: EagerVec::forced_import(db, "month3", version)?,
            month6: EagerVec::forced_import(db, "month6", version)?,
            year1: EagerVec::forced_import(db, "year1", version)?,
            year10: EagerVec::forced_import(db, "year10", version)?,
            txindex_count: EagerVec::forced_import(db, "txindex_count", version)?,
        })
    }
}
