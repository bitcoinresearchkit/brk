use brk_traversable::Traversable;
use brk_types::{
    Date, DateIndex, DecadeIndex, Height, MonthIndex, QuarterIndex, SemesterIndex, StoredU64,
    WeekIndex, YearIndex,
};
use vecdb::{EagerVec, PcoVec};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub dateindex_to_date: EagerVec<PcoVec<DateIndex, Date>>,
    pub dateindex_to_dateindex: EagerVec<PcoVec<DateIndex, DateIndex>>,
    pub dateindex_to_first_height: EagerVec<PcoVec<DateIndex, Height>>,
    pub dateindex_to_height_count: EagerVec<PcoVec<DateIndex, StoredU64>>,
    pub dateindex_to_monthindex: EagerVec<PcoVec<DateIndex, MonthIndex>>,
    pub dateindex_to_weekindex: EagerVec<PcoVec<DateIndex, WeekIndex>>,
    pub weekindex_to_dateindex_count: EagerVec<PcoVec<WeekIndex, StoredU64>>,
    pub weekindex_to_first_dateindex: EagerVec<PcoVec<WeekIndex, DateIndex>>,
    pub weekindex_to_weekindex: EagerVec<PcoVec<WeekIndex, WeekIndex>>,
    pub monthindex_to_dateindex_count: EagerVec<PcoVec<MonthIndex, StoredU64>>,
    pub monthindex_to_first_dateindex: EagerVec<PcoVec<MonthIndex, DateIndex>>,
    pub monthindex_to_monthindex: EagerVec<PcoVec<MonthIndex, MonthIndex>>,
    pub monthindex_to_quarterindex: EagerVec<PcoVec<MonthIndex, QuarterIndex>>,
    pub monthindex_to_semesterindex: EagerVec<PcoVec<MonthIndex, SemesterIndex>>,
    pub monthindex_to_yearindex: EagerVec<PcoVec<MonthIndex, YearIndex>>,
    pub quarterindex_to_first_monthindex: EagerVec<PcoVec<QuarterIndex, MonthIndex>>,
    pub quarterindex_to_monthindex_count: EagerVec<PcoVec<QuarterIndex, StoredU64>>,
    pub quarterindex_to_quarterindex: EagerVec<PcoVec<QuarterIndex, QuarterIndex>>,
    pub semesterindex_to_first_monthindex: EagerVec<PcoVec<SemesterIndex, MonthIndex>>,
    pub semesterindex_to_monthindex_count: EagerVec<PcoVec<SemesterIndex, StoredU64>>,
    pub semesterindex_to_semesterindex: EagerVec<PcoVec<SemesterIndex, SemesterIndex>>,
    pub yearindex_to_decadeindex: EagerVec<PcoVec<YearIndex, DecadeIndex>>,
    pub yearindex_to_first_monthindex: EagerVec<PcoVec<YearIndex, MonthIndex>>,
    pub yearindex_to_monthindex_count: EagerVec<PcoVec<YearIndex, StoredU64>>,
    pub yearindex_to_yearindex: EagerVec<PcoVec<YearIndex, YearIndex>>,
    pub decadeindex_to_decadeindex: EagerVec<PcoVec<DecadeIndex, DecadeIndex>>,
    pub decadeindex_to_first_yearindex: EagerVec<PcoVec<DecadeIndex, YearIndex>>,
    pub decadeindex_to_yearindex_count: EagerVec<PcoVec<DecadeIndex, StoredU64>>,
}

pub struct StartingTimeIndexes {
    pub dateindex: DateIndex,
    pub weekindex: WeekIndex,
    pub monthindex: MonthIndex,
    pub quarterindex: QuarterIndex,
    pub semesterindex: SemesterIndex,
    pub yearindex: YearIndex,
    pub decadeindex: DecadeIndex,
}
