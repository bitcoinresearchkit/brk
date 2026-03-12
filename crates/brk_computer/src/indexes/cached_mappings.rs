use brk_types::{
    Day1, Day3, Epoch, Halving, Height, Hour1, Hour4, Hour12, Minute10, Minute30, Month1, Month3,
    Month6, Week1, Year1, Year10,
};
use vecdb::CachedVec;

use super::Vecs;

#[derive(Clone)]
pub struct CachedMappings {
    pub minute10_first_height: CachedVec<Minute10, Height>,
    pub minute30_first_height: CachedVec<Minute30, Height>,
    pub hour1_first_height: CachedVec<Hour1, Height>,
    pub hour4_first_height: CachedVec<Hour4, Height>,
    pub hour12_first_height: CachedVec<Hour12, Height>,
    pub day1_first_height: CachedVec<Day1, Height>,
    pub day3_first_height: CachedVec<Day3, Height>,
    pub week1_first_height: CachedVec<Week1, Height>,
    pub month1_first_height: CachedVec<Month1, Height>,
    pub month3_first_height: CachedVec<Month3, Height>,
    pub month6_first_height: CachedVec<Month6, Height>,
    pub year1_first_height: CachedVec<Year1, Height>,
    pub year10_first_height: CachedVec<Year10, Height>,
    pub halving_identity: CachedVec<Halving, Halving>,
    pub epoch_identity: CachedVec<Epoch, Epoch>,
}

impl CachedMappings {
    pub fn new(vecs: &Vecs) -> Self {
        Self {
            minute10_first_height: CachedVec::new(&vecs.minute10.first_height),
            minute30_first_height: CachedVec::new(&vecs.minute30.first_height),
            hour1_first_height: CachedVec::new(&vecs.hour1.first_height),
            hour4_first_height: CachedVec::new(&vecs.hour4.first_height),
            hour12_first_height: CachedVec::new(&vecs.hour12.first_height),
            day1_first_height: CachedVec::new(&vecs.day1.first_height),
            day3_first_height: CachedVec::new(&vecs.day3.first_height),
            week1_first_height: CachedVec::new(&vecs.week1.first_height),
            month1_first_height: CachedVec::new(&vecs.month1.first_height),
            month3_first_height: CachedVec::new(&vecs.month3.first_height),
            month6_first_height: CachedVec::new(&vecs.month6.first_height),
            year1_first_height: CachedVec::new(&vecs.year1.first_height),
            year10_first_height: CachedVec::new(&vecs.year10.first_height),
            halving_identity: CachedVec::new(&vecs.halving.identity),
            epoch_identity: CachedVec::new(&vecs.epoch.identity),
        }
    }
}
