use brk_traversable::Traversable;
use brk_types::{Date, DateIndex};

/// DCA class years
pub const DCA_CLASS_YEARS: ByDcaClass<u16> = ByDcaClass {
    _2015: 2015,
    _2016: 2016,
    _2017: 2017,
    _2018: 2018,
    _2019: 2019,
    _2020: 2020,
    _2021: 2021,
    _2022: 2022,
    _2023: 2023,
    _2024: 2024,
    _2025: 2025,
};

/// DCA class names
pub const DCA_CLASS_NAMES: ByDcaClass<&'static str> = ByDcaClass {
    _2015: "dca_class_2015",
    _2016: "dca_class_2016",
    _2017: "dca_class_2017",
    _2018: "dca_class_2018",
    _2019: "dca_class_2019",
    _2020: "dca_class_2020",
    _2021: "dca_class_2021",
    _2022: "dca_class_2022",
    _2023: "dca_class_2023",
    _2024: "dca_class_2024",
    _2025: "dca_class_2025",
};

/// Generic wrapper for DCA year class data
#[derive(Default, Clone, Traversable)]
pub struct ByDcaClass<T> {
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
}

impl<T> ByDcaClass<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(&'static str, u16, DateIndex) -> T,
    {
        let n = DCA_CLASS_NAMES;
        let y = DCA_CLASS_YEARS;
        Self {
            _2015: create(n._2015, y._2015, Self::dateindex(y._2015)),
            _2016: create(n._2016, y._2016, Self::dateindex(y._2016)),
            _2017: create(n._2017, y._2017, Self::dateindex(y._2017)),
            _2018: create(n._2018, y._2018, Self::dateindex(y._2018)),
            _2019: create(n._2019, y._2019, Self::dateindex(y._2019)),
            _2020: create(n._2020, y._2020, Self::dateindex(y._2020)),
            _2021: create(n._2021, y._2021, Self::dateindex(y._2021)),
            _2022: create(n._2022, y._2022, Self::dateindex(y._2022)),
            _2023: create(n._2023, y._2023, Self::dateindex(y._2023)),
            _2024: create(n._2024, y._2024, Self::dateindex(y._2024)),
            _2025: create(n._2025, y._2025, Self::dateindex(y._2025)),
        }
    }

    pub fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(&'static str, u16, DateIndex) -> Result<T, E>,
    {
        let n = DCA_CLASS_NAMES;
        let y = DCA_CLASS_YEARS;
        Ok(Self {
            _2015: create(n._2015, y._2015, Self::dateindex(y._2015))?,
            _2016: create(n._2016, y._2016, Self::dateindex(y._2016))?,
            _2017: create(n._2017, y._2017, Self::dateindex(y._2017))?,
            _2018: create(n._2018, y._2018, Self::dateindex(y._2018))?,
            _2019: create(n._2019, y._2019, Self::dateindex(y._2019))?,
            _2020: create(n._2020, y._2020, Self::dateindex(y._2020))?,
            _2021: create(n._2021, y._2021, Self::dateindex(y._2021))?,
            _2022: create(n._2022, y._2022, Self::dateindex(y._2022))?,
            _2023: create(n._2023, y._2023, Self::dateindex(y._2023))?,
            _2024: create(n._2024, y._2024, Self::dateindex(y._2024))?,
            _2025: create(n._2025, y._2025, Self::dateindex(y._2025))?,
        })
    }

    pub fn dateindex(year: u16) -> DateIndex {
        DateIndex::try_from(Date::new(year, 1, 1)).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [
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
        ]
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
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
        ]
        .into_iter()
    }

    pub fn iter_mut_with_dateindex(&mut self) -> impl Iterator<Item = (&mut T, DateIndex)> {
        let y = DCA_CLASS_YEARS;
        [
            (&mut self._2015, Self::dateindex(y._2015)),
            (&mut self._2016, Self::dateindex(y._2016)),
            (&mut self._2017, Self::dateindex(y._2017)),
            (&mut self._2018, Self::dateindex(y._2018)),
            (&mut self._2019, Self::dateindex(y._2019)),
            (&mut self._2020, Self::dateindex(y._2020)),
            (&mut self._2021, Self::dateindex(y._2021)),
            (&mut self._2022, Self::dateindex(y._2022)),
            (&mut self._2023, Self::dateindex(y._2023)),
            (&mut self._2024, Self::dateindex(y._2024)),
            (&mut self._2025, Self::dateindex(y._2025)),
        ]
        .into_iter()
    }

    pub fn dateindexes() -> [DateIndex; 11] {
        let y = DCA_CLASS_YEARS;
        [
            Self::dateindex(y._2015),
            Self::dateindex(y._2016),
            Self::dateindex(y._2017),
            Self::dateindex(y._2018),
            Self::dateindex(y._2019),
            Self::dateindex(y._2020),
            Self::dateindex(y._2021),
            Self::dateindex(y._2022),
            Self::dateindex(y._2023),
            Self::dateindex(y._2024),
            Self::dateindex(y._2025),
        ]
    }

    pub fn zip<U>(self, other: ByDcaClass<U>) -> ByDcaClass<(T, U)> {
        ByDcaClass {
            _2015: (self._2015, other._2015),
            _2016: (self._2016, other._2016),
            _2017: (self._2017, other._2017),
            _2018: (self._2018, other._2018),
            _2019: (self._2019, other._2019),
            _2020: (self._2020, other._2020),
            _2021: (self._2021, other._2021),
            _2022: (self._2022, other._2022),
            _2023: (self._2023, other._2023),
            _2024: (self._2024, other._2024),
            _2025: (self._2025, other._2025),
        }
    }

    pub fn zip_ref<'a, U>(&'a self, other: &'a ByDcaClass<U>) -> ByDcaClass<(&'a T, &'a U)> {
        ByDcaClass {
            _2015: (&self._2015, &other._2015),
            _2016: (&self._2016, &other._2016),
            _2017: (&self._2017, &other._2017),
            _2018: (&self._2018, &other._2018),
            _2019: (&self._2019, &other._2019),
            _2020: (&self._2020, &other._2020),
            _2021: (&self._2021, &other._2021),
            _2022: (&self._2022, &other._2022),
            _2023: (&self._2023, &other._2023),
            _2024: (&self._2024, &other._2024),
            _2025: (&self._2025, &other._2025),
        }
    }

    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> ByDcaClass<U> {
        ByDcaClass {
            _2015: f(self._2015),
            _2016: f(self._2016),
            _2017: f(self._2017),
            _2018: f(self._2018),
            _2019: f(self._2019),
            _2020: f(self._2020),
            _2021: f(self._2021),
            _2022: f(self._2022),
            _2023: f(self._2023),
            _2024: f(self._2024),
            _2025: f(self._2025),
        }
    }
}
