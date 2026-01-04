use brk_types::{HalvingEpoch, OutputType, Sats, Year};

use super::{AmountFilter, CohortContext, Term, TimeFilter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    All,
    Term(Term),
    Time(TimeFilter),
    Amount(AmountFilter),
    Epoch(HalvingEpoch),
    Year(Year),
    Type(OutputType),
}

impl Filter {
    pub fn is_all(&self) -> bool {
        matches!(self, Filter::All)
    }

    /// Returns true if this filter includes day 0 (only applicable to time-based filters)
    pub fn includes_first_day(&self) -> bool {
        match self {
            Filter::All => true,
            Filter::Term(Term::Sth) => true,
            Filter::Term(Term::Lth) => false,
            Filter::Time(t) => t.includes_first_day(),
            _ => false,
        }
    }

    /// Check if a time value (hours) is contained by this filter
    pub fn contains_time(&self, hours: usize) -> bool {
        match self {
            Filter::All => true,
            Filter::Term(Term::Sth) => hours < Term::THRESHOLD_HOURS,
            Filter::Term(Term::Lth) => hours >= Term::THRESHOLD_HOURS,
            Filter::Time(t) => t.contains(hours),
            _ => false,
        }
    }

    /// Check if an amount value (sats) is contained by this filter
    pub fn contains_amount(&self, sats: Sats) -> bool {
        match self {
            Filter::All => true,
            Filter::Amount(a) => a.contains(sats),
            _ => false,
        }
    }

    /// Check if this filter includes another filter (for aggregation)
    pub fn includes(&self, other: &Filter) -> bool {
        match (self, other) {
            (Filter::All, _) => true,
            (Filter::Term(Term::Sth), Filter::Time(t)) => {
                matches!(t, TimeFilter::LowerThan(h) if *h <= Term::THRESHOLD_HOURS)
                    || matches!(t, TimeFilter::Range(r) if r.end <= Term::THRESHOLD_HOURS)
            }
            (Filter::Term(Term::Lth), Filter::Time(t)) => {
                matches!(t, TimeFilter::GreaterOrEqual(h) if *h >= Term::THRESHOLD_HOURS)
                    || matches!(t, TimeFilter::Range(r) if r.start >= Term::THRESHOLD_HOURS)
            }
            (Filter::Time(t1), Filter::Time(t2)) => t1.includes(t2),
            (Filter::Amount(a1), Filter::Amount(a2)) => a1.includes(a2),
            _ => false,
        }
    }

    /// Whether to compute extended metrics (realized cap ratios, profit/loss ratios, percentiles)
    /// For UTXO context: true only for age range cohorts (Range) and aggregate cohorts (All, Term)
    /// For Address context: always false
    pub fn is_extended(&self, context: CohortContext) -> bool {
        match context {
            CohortContext::Address => false,
            CohortContext::Utxo => {
                matches!(
                    self,
                    Filter::All | Filter::Term(_) | Filter::Time(TimeFilter::Range(_))
                )
            }
        }
    }

    /// Whether to compute metrics relative to the "all" baseline
    /// False only for All itself (it IS the baseline)
    pub fn compute_rel_to_all(&self) -> bool {
        !matches!(self, Filter::All)
    }

    /// Whether to compute adjusted metrics (adjusted SOPR, adjusted value created/destroyed)
    /// For UTXO context: true for All, STH, and max_age (LowerThan)
    /// For Address context: always false
    /// Note: LTH doesn't need adjusted (everything >= 5 months is already > 1 hour)
    /// Note: age ranges don't need adjusted (0-1h data lives in its own cohort)
    pub fn compute_adjusted(&self, context: CohortContext) -> bool {
        match context {
            CohortContext::Address => false,
            CohortContext::Utxo => matches!(
                self,
                Filter::All | Filter::Term(Term::Sth) | Filter::Time(TimeFilter::LowerThan(_))
            ),
        }
    }
}
