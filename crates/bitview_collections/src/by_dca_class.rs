#[cfg(feature = "storage")]
use bitview_traversable::Traversable;
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
#[derive(Clone, Default)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct ByDcaClass<T> {
    /// Uses January 1, 2015 as the strategy start date.
    pub from_2015: T,
    /// Uses January 1, 2016 as the strategy start date.
    pub from_2016: T,
    /// Uses January 1, 2017 as the strategy start date.
    pub from_2017: T,
    /// Uses January 1, 2018 as the strategy start date.
    pub from_2018: T,
    /// Uses January 1, 2019 as the strategy start date.
    pub from_2019: T,
    /// Uses January 1, 2020 as the strategy start date.
    pub from_2020: T,
    /// Uses January 1, 2021 as the strategy start date.
    pub from_2021: T,
    /// Uses January 1, 2022 as the strategy start date.
    pub from_2022: T,
    /// Uses January 1, 2023 as the strategy start date.
    pub from_2023: T,
    /// Uses January 1, 2024 as the strategy start date.
    pub from_2024: T,
    /// Uses January 1, 2025 as the strategy start date.
    pub from_2025: T,
    /// Uses January 1, 2026 as the strategy start date.
    pub from_2026: T,
}

impl<T> ByDcaClass<T> {
    pub fn try_new<F, E>(mut create: F) -> Result<ByDcaClass<T>, E>
    where
        F: FnMut(&'static str, u16, Day1) -> Result<T, E>,
    {
        Self::try_from_class(&DCA_CLASS_YEARS, |name, year, day, _| {
            create(name, year, day)
        })
    }

    pub fn try_from_class<U, F, E>(class: &ByDcaClass<U>, mut create: F) -> Result<ByDcaClass<T>, E>
    where
        F: FnMut(&'static str, u16, Day1, &U) -> Result<T, E>,
    {
        macro_rules! fields {
        ($($field:ident),+ $(,)?) => {
            Ok(ByDcaClass {
                $($field: create(
                    DCA_CLASS_NAMES.$field,
                    DCA_CLASS_YEARS.$field,
                    Day1::try_from(Date::new(DCA_CLASS_YEARS.$field, 1, 1)).unwrap(),
                    &class.$field,
                )?),+
            })
        };
    }
        fields!(
            from_2015, from_2016, from_2017, from_2018, from_2019, from_2020, from_2021, from_2022,
            from_2023, from_2024, from_2025, from_2026,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_preserve_every_class_name_year_date_and_source() {
        let mut years = Vec::new();
        let classes = ByDcaClass::try_new(|name, year, day| {
            assert_eq!(name, format!("from_{year}"));
            assert_eq!(day, Day1::try_from(Date::new(year, 1, 1)).unwrap());
            years.push(year);
            Ok::<_, ()>(name.to_owned())
        })
        .unwrap();
        assert_eq!(years, (2015..=2026).collect::<Vec<_>>());

        let fields = [
            &classes.from_2015,
            &classes.from_2016,
            &classes.from_2017,
            &classes.from_2018,
            &classes.from_2019,
            &classes.from_2020,
            &classes.from_2021,
            &classes.from_2022,
            &classes.from_2023,
            &classes.from_2024,
            &classes.from_2025,
            &classes.from_2026,
        ];
        for (field, year) in fields.into_iter().zip(2015..=2026) {
            assert_eq!(field, &format!("from_{year}"));
        }

        let mut seen = 0;
        let mapped = ByDcaClass::try_from_class(&classes, |name, year, day, source| {
            assert_eq!(name, source);
            assert_eq!(year, 2015 + seen as u16);
            assert_eq!(day, Day1::try_from(Date::new(year, 1, 1)).unwrap());
            assert!(std::ptr::eq(source, fields[seen]));
            seen += 1;
            Ok::<_, ()>(name)
        })
        .unwrap();
        assert_eq!(seen, 12);
        assert_eq!(mapped.from_2015, "from_2015");
        assert_eq!(mapped.from_2026, "from_2026");
    }

    #[test]
    fn both_constructors_stop_at_each_possible_error() {
        for fail_at in 0..12 {
            for from_class in [false, true] {
                let mut seen = Vec::new();
                let mut create = |name, year, day| {
                    seen.push((name, year, day));
                    if seen.len() == fail_at + 1 {
                        Err(year)
                    } else {
                        Ok(())
                    }
                };
                let result = if from_class {
                    ByDcaClass::try_from_class(&DCA_CLASS_NAMES, |name, year, day, source| {
                        assert_eq!(name, *source);
                        create(name, year, day)
                    })
                } else {
                    ByDcaClass::try_new(create)
                };
                assert!(matches!(result, Err(year) if year == 2015 + fail_at as u16));
                assert_eq!(seen.len(), fail_at + 1);
                for ((name, year, day), expected_year) in seen.into_iter().zip(2015..=2026) {
                    assert_eq!(year, expected_year);
                    assert_eq!(name, format!("from_{expected_year}"));
                    assert_eq!(day, Day1::try_from(Date::new(expected_year, 1, 1)).unwrap());
                }
            }
        }
    }
}
