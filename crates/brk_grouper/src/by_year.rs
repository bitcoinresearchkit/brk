use brk_traversable::Traversable;
use brk_types::{Timestamp, Year};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::Filter;

#[derive(Default, Clone, Traversable)]
pub struct ByYear<T> {
    pub _2009: T,
    pub _2010: T,
    pub _2011: T,
    pub _2012: T,
    pub _2013: T,
    pub _2014: T,
    pub _2015: T,
    pub _2016: T,
    pub _2017: T,
    pub _2018: T,
    pub _2019: T,
    pub _2020: T,
    pub _2021: T,
    pub _2022: T,
    pub _2023: T,
    pub _2024: T,
    pub _2025: T,
    pub _2026: T,
}

impl<T> ByYear<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter) -> T,
    {
        Self {
            _2009: create(Filter::Year(Year::new(2009))),
            _2010: create(Filter::Year(Year::new(2010))),
            _2011: create(Filter::Year(Year::new(2011))),
            _2012: create(Filter::Year(Year::new(2012))),
            _2013: create(Filter::Year(Year::new(2013))),
            _2014: create(Filter::Year(Year::new(2014))),
            _2015: create(Filter::Year(Year::new(2015))),
            _2016: create(Filter::Year(Year::new(2016))),
            _2017: create(Filter::Year(Year::new(2017))),
            _2018: create(Filter::Year(Year::new(2018))),
            _2019: create(Filter::Year(Year::new(2019))),
            _2020: create(Filter::Year(Year::new(2020))),
            _2021: create(Filter::Year(Year::new(2021))),
            _2022: create(Filter::Year(Year::new(2022))),
            _2023: create(Filter::Year(Year::new(2023))),
            _2024: create(Filter::Year(Year::new(2024))),
            _2025: create(Filter::Year(Year::new(2025))),
            _2026: create(Filter::Year(Year::new(2026))),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [
            &self._2009,
            &self._2010,
            &self._2011,
            &self._2012,
            &self._2013,
            &self._2014,
            &self._2015,
            &self._2016,
            &self._2017,
            &self._2018,
            &self._2019,
            &self._2020,
            &self._2021,
            &self._2022,
            &self._2023,
            &self._2024,
            &self._2025,
            &self._2026,
        ]
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self._2009,
            &mut self._2010,
            &mut self._2011,
            &mut self._2012,
            &mut self._2013,
            &mut self._2014,
            &mut self._2015,
            &mut self._2016,
            &mut self._2017,
            &mut self._2018,
            &mut self._2019,
            &mut self._2020,
            &mut self._2021,
            &mut self._2022,
            &mut self._2023,
            &mut self._2024,
            &mut self._2025,
            &mut self._2026,
        ]
        .into_iter()
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [
            &mut self._2009,
            &mut self._2010,
            &mut self._2011,
            &mut self._2012,
            &mut self._2013,
            &mut self._2014,
            &mut self._2015,
            &mut self._2016,
            &mut self._2017,
            &mut self._2018,
            &mut self._2019,
            &mut self._2020,
            &mut self._2021,
            &mut self._2022,
            &mut self._2023,
            &mut self._2024,
            &mut self._2025,
            &mut self._2026,
        ]
        .into_par_iter()
    }

    pub fn mut_vec_from_timestamp(&mut self, timestamp: Timestamp) -> &mut T {
        let year = Year::from(timestamp);
        self.get_mut(year)
    }

    pub fn get_mut(&mut self, year: Year) -> &mut T {
        match u16::from(year) {
            2009 => &mut self._2009,
            2010 => &mut self._2010,
            2011 => &mut self._2011,
            2012 => &mut self._2012,
            2013 => &mut self._2013,
            2014 => &mut self._2014,
            2015 => &mut self._2015,
            2016 => &mut self._2016,
            2017 => &mut self._2017,
            2018 => &mut self._2018,
            2019 => &mut self._2019,
            2020 => &mut self._2020,
            2021 => &mut self._2021,
            2022 => &mut self._2022,
            2023 => &mut self._2023,
            2024 => &mut self._2024,
            2025 => &mut self._2025,
            2026 => &mut self._2026,
            _ => todo!("Year {} not yet supported", u16::from(year)),
        }
    }
}
