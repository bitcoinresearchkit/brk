use brk_traversable::Traversable;
use brk_types::{Date, Day1};

/// DCA class years
pub const DCA_CLASS_YEARS: ByDcaClass<u16> = ByDcaClass {
    from_2015: 2015,
    from_2016: 2016,
    from_2017: 2017,
    from_2018: 2018,
    from_2019: 2019,
    from_2020: 2020,
    from_2021: 2021,
    from_2022: 2022,
    from_2023: 2023,
    from_2024: 2024,
    from_2025: 2025,
    from_2026: 2026,
};

/// DCA class names
pub const DCA_CLASS_NAMES: ByDcaClass<&'static str> = ByDcaClass {
    from_2015: "from_2015",
    from_2016: "from_2016",
    from_2017: "from_2017",
    from_2018: "from_2018",
    from_2019: "from_2019",
    from_2020: "from_2020",
    from_2021: "from_2021",
    from_2022: "from_2022",
    from_2023: "from_2023",
    from_2024: "from_2024",
    from_2025: "from_2025",
    from_2026: "from_2026",
};

/// Generic wrapper for DCA year class data
#[derive(Clone, Default, Traversable)]
pub struct ByDcaClass<T> {
    pub from_2015: T,
    pub from_2016: T,
    pub from_2017: T,
    pub from_2018: T,
    pub from_2019: T,
    pub from_2020: T,
    pub from_2021: T,
    pub from_2022: T,
    pub from_2023: T,
    pub from_2024: T,
    pub from_2025: T,
    pub from_2026: T,
}

impl<T> ByDcaClass<T> {
    pub(crate) fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(&'static str, u16, Day1) -> Result<T, E>,
    {
        let n = DCA_CLASS_NAMES;
        let y = DCA_CLASS_YEARS;
        Ok(Self {
            from_2015: create(n.from_2015, y.from_2015, Self::day1(y.from_2015))?,
            from_2016: create(n.from_2016, y.from_2016, Self::day1(y.from_2016))?,
            from_2017: create(n.from_2017, y.from_2017, Self::day1(y.from_2017))?,
            from_2018: create(n.from_2018, y.from_2018, Self::day1(y.from_2018))?,
            from_2019: create(n.from_2019, y.from_2019, Self::day1(y.from_2019))?,
            from_2020: create(n.from_2020, y.from_2020, Self::day1(y.from_2020))?,
            from_2021: create(n.from_2021, y.from_2021, Self::day1(y.from_2021))?,
            from_2022: create(n.from_2022, y.from_2022, Self::day1(y.from_2022))?,
            from_2023: create(n.from_2023, y.from_2023, Self::day1(y.from_2023))?,
            from_2024: create(n.from_2024, y.from_2024, Self::day1(y.from_2024))?,
            from_2025: create(n.from_2025, y.from_2025, Self::day1(y.from_2025))?,
            from_2026: create(n.from_2026, y.from_2026, Self::day1(y.from_2026))?,
        })
    }

    pub(crate) fn day1(year: u16) -> Day1 {
        Day1::try_from(Date::new(year, 1, 1)).unwrap()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        [
            &self.from_2015,
            &self.from_2016,
            &self.from_2017,
            &self.from_2018,
            &self.from_2019,
            &self.from_2020,
            &self.from_2021,
            &self.from_2022,
            &self.from_2023,
            &self.from_2024,
            &self.from_2025,
            &self.from_2026,
        ]
        .into_iter()
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self.from_2015,
            &mut self.from_2016,
            &mut self.from_2017,
            &mut self.from_2018,
            &mut self.from_2019,
            &mut self.from_2020,
            &mut self.from_2021,
            &mut self.from_2022,
            &mut self.from_2023,
            &mut self.from_2024,
            &mut self.from_2025,
            &mut self.from_2026,
        ]
        .into_iter()
    }

    pub(crate) fn start_days() -> [Day1; 12] {
        let y = DCA_CLASS_YEARS;
        [
            Self::day1(y.from_2015),
            Self::day1(y.from_2016),
            Self::day1(y.from_2017),
            Self::day1(y.from_2018),
            Self::day1(y.from_2019),
            Self::day1(y.from_2020),
            Self::day1(y.from_2021),
            Self::day1(y.from_2022),
            Self::day1(y.from_2023),
            Self::day1(y.from_2024),
            Self::day1(y.from_2025),
            Self::day1(y.from_2026),
        ]
    }
}
