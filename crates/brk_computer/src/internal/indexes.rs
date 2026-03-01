//! Base generic struct with 15 type parameters — one per time period/epoch index.
//!
//! Foundation for all per-index types. Replaces the repetitive 15-field pattern
//! found throughout height_derived types.

use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct Indexes<M10, M30, H1, H4, H12, D1, D3, W1, Mo1, Mo3, Mo6, Y1, Y10, HE, DE> {
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

/// Imperative counterpart to `indexes_from!` — calls `$period!(field)` for each
/// period field and `$epoch!(field)` for each epoch field.
#[macro_export]
macro_rules! indexes_apply {
    ($period:ident, $epoch:ident) => {
        $period!(minute10);
        $period!(minute30);
        $period!(hour1);
        $period!(hour4);
        $period!(hour12);
        $period!(day1);
        $period!(day3);
        $period!(week1);
        $period!(month1);
        $period!(month3);
        $period!(month6);
        $period!(year1);
        $period!(year10);
        $epoch!(halvingepoch);
        $epoch!(difficultyepoch);
    };
    ($m:ident) => {
        $crate::indexes_apply!($m, $m)
    };
}
