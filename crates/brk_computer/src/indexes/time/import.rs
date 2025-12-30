use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;

impl Vecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            dateindex_to_date: EagerVec::forced_import(db, "date", version)?,
            dateindex_to_dateindex: EagerVec::forced_import(db, "dateindex", version)?,
            dateindex_to_first_height: EagerVec::forced_import(db, "first_height", version)?,
            dateindex_to_height_count: EagerVec::forced_import(db, "height_count", version)?,
            dateindex_to_monthindex: EagerVec::forced_import(db, "monthindex", version)?,
            dateindex_to_weekindex: EagerVec::forced_import(db, "weekindex", version)?,
            weekindex_to_dateindex_count: EagerVec::forced_import(db, "dateindex_count", version)?,
            weekindex_to_first_dateindex: EagerVec::forced_import(db, "first_dateindex", version)?,
            weekindex_to_weekindex: EagerVec::forced_import(db, "weekindex", version)?,
            monthindex_to_dateindex_count: EagerVec::forced_import(db, "dateindex_count", version)?,
            monthindex_to_first_dateindex: EagerVec::forced_import(db, "first_dateindex", version)?,
            monthindex_to_monthindex: EagerVec::forced_import(db, "monthindex", version)?,
            monthindex_to_quarterindex: EagerVec::forced_import(db, "quarterindex", version)?,
            monthindex_to_semesterindex: EagerVec::forced_import(db, "semesterindex", version)?,
            monthindex_to_yearindex: EagerVec::forced_import(db, "yearindex", version)?,
            quarterindex_to_first_monthindex: EagerVec::forced_import(db, "first_monthindex", version)?,
            quarterindex_to_monthindex_count: EagerVec::forced_import(db, "monthindex_count", version)?,
            quarterindex_to_quarterindex: EagerVec::forced_import(db, "quarterindex", version)?,
            semesterindex_to_first_monthindex: EagerVec::forced_import(db, "first_monthindex", version)?,
            semesterindex_to_monthindex_count: EagerVec::forced_import(db, "monthindex_count", version)?,
            semesterindex_to_semesterindex: EagerVec::forced_import(db, "semesterindex", version)?,
            yearindex_to_decadeindex: EagerVec::forced_import(db, "decadeindex", version)?,
            yearindex_to_first_monthindex: EagerVec::forced_import(db, "first_monthindex", version)?,
            yearindex_to_monthindex_count: EagerVec::forced_import(db, "monthindex_count", version)?,
            yearindex_to_yearindex: EagerVec::forced_import(db, "yearindex", version)?,
            decadeindex_to_decadeindex: EagerVec::forced_import(db, "decadeindex", version)?,
            decadeindex_to_first_yearindex: EagerVec::forced_import(db, "first_yearindex", version)?,
            decadeindex_to_yearindex_count: EagerVec::forced_import(db, "yearindex_count", version)?,
        })
    }
}
