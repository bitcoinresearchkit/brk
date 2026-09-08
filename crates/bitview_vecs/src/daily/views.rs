use bitview_traversable::Traversable;
use brk_types::{
    Day1, Day3, Epoch, Halving, Height, Hour1, Hour4, Hour12, Minute10, Minute30, Month1, Month3,
    Month6, Version, Week1, Year1, Year10,
};
use vecdb::ReadableBoxedVec;

use crate::DailyView;
use crate::{DailyMappings, DailyValue, LastDay, RepeatDay};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct DailyViews<T>
where
    T: DailyValue,
{
    pub height: DailyView<Height, T, RepeatDay>,
    pub minute10: DailyView<Minute10, T, RepeatDay>,
    pub minute30: DailyView<Minute30, T, RepeatDay>,
    pub hour1: DailyView<Hour1, T, RepeatDay>,
    pub hour4: DailyView<Hour4, T, RepeatDay>,
    pub hour12: DailyView<Hour12, T, RepeatDay>,
    pub day3: DailyView<Day3, T, LastDay>,
    pub week1: DailyView<Week1, T, LastDay>,
    pub month1: DailyView<Month1, T, LastDay>,
    pub month3: DailyView<Month3, T, LastDay>,
    pub month6: DailyView<Month6, T, LastDay>,
    pub year1: DailyView<Year1, T, LastDay>,
    pub year10: DailyView<Year10, T, LastDay>,
    pub halving: DailyView<Halving, T, LastDay>,
    pub epoch: DailyView<Epoch, T, LastDay>,
}

impl<T> DailyViews<T>
where
    T: DailyValue,
{
    pub fn new(
        name: &str,
        source: ReadableBoxedVec<Day1, T>,
        version: Version,
        mappings: &DailyMappings,
    ) -> Self {
        Self {
            height: DailyView::new(name, version, source.clone(), &mappings.height),
            minute10: DailyView::new(name, version, source.clone(), &mappings.minute10),
            minute30: DailyView::new(name, version, source.clone(), &mappings.minute30),
            hour1: DailyView::new(name, version, source.clone(), &mappings.hour1),
            hour4: DailyView::new(name, version, source.clone(), &mappings.hour4),
            hour12: DailyView::new(name, version, source.clone(), &mappings.hour12),
            day3: DailyView::new(name, version, source.clone(), &mappings.day3),
            week1: DailyView::new(name, version, source.clone(), &mappings.week1),
            month1: DailyView::new(name, version, source.clone(), &mappings.month1),
            month3: DailyView::new(name, version, source.clone(), &mappings.month3),
            month6: DailyView::new(name, version, source.clone(), &mappings.month6),
            year1: DailyView::new(name, version, source.clone(), &mappings.year1),
            year10: DailyView::new(name, version, source.clone(), &mappings.year10),
            halving: DailyView::new(name, version, source.clone(), &mappings.halving),
            epoch: DailyView::new(name, version, source, &mappings.epoch),
        }
    }
}
