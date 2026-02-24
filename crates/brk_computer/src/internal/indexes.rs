//! Base generic struct with 17 type parameters — one per time period/epoch index.
//!
//! Foundation for all per-index types. Replaces the repetitive 17-field pattern
//! found throughout height_derived types.

use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct Indexes<M1, M5, M10, M30, H1, H4, H12, D1, D3, W1, Mo1, Mo3, Mo6, Y1, Y10, HE, DE> {
    pub minute1: M1,
    pub minute5: M5,
    pub minute10: M10,
    pub minute30: M30,
    pub hour1: H1,
    pub hour4: H4,
    pub hour12: H12,
    pub day1: D1,
    pub day3: D3,
    pub week1: W1,
    pub month1: Mo1,
    pub month3: Mo3,
    pub month6: Mo6,
    pub year1: Y1,
    pub year10: Y10,
    pub halvingepoch: HE,
    pub difficultyepoch: DE,
}

/// Helper macro to construct an `Indexes` by applying a macro to each field.
///
/// Usage:
/// ```ignore
/// indexes_from!(period, epoch)
/// ```
/// where `period!($field)` and `epoch!($field)` are locally-defined macros.
#[macro_export]
macro_rules! indexes_from {
    ($period:ident, $epoch:ident) => {
        $crate::internal::Indexes {
            minute1: $period!(minute1),
            minute5: $period!(minute5),
            minute10: $period!(minute10),
            minute30: $period!(minute30),
            hour1: $period!(hour1),
            hour4: $period!(hour4),
            hour12: $period!(hour12),
            day1: $period!(day1),
            day3: $period!(day3),
            week1: $period!(week1),
            month1: $period!(month1),
            month3: $period!(month3),
            month6: $period!(month6),
            year1: $period!(year1),
            year10: $period!(year10),
            halvingepoch: $epoch!(halvingepoch),
            difficultyepoch: $epoch!(difficultyepoch),
        }
    };
    // Variant where period and epoch use the same macro
    ($m:ident) => {
        $crate::indexes_from!($m, $m)
    };
}

/// Helper macro to apply a function/macro to each field of an `Indexes` value.
#[macro_export]
macro_rules! indexes_map {
    ($indexes:expr, |$field:ident| $body:expr) => {{
        let src = $indexes;
        $crate::internal::Indexes {
            minute1: { let $field = src.minute1; $body },
            minute5: { let $field = src.minute5; $body },
            minute10: { let $field = src.minute10; $body },
            minute30: { let $field = src.minute30; $body },
            hour1: { let $field = src.hour1; $body },
            hour4: { let $field = src.hour4; $body },
            hour12: { let $field = src.hour12; $body },
            day1: { let $field = src.day1; $body },
            day3: { let $field = src.day3; $body },
            week1: { let $field = src.week1; $body },
            month1: { let $field = src.month1; $body },
            month3: { let $field = src.month3; $body },
            month6: { let $field = src.month6; $body },
            year1: { let $field = src.year1; $body },
            year10: { let $field = src.year10; $body },
            halvingepoch: { let $field = src.halvingepoch; $body },
            difficultyepoch: { let $field = src.difficultyepoch; $body },
        }
    }};
}
