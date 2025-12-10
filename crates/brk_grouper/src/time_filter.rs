use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeFilter {
    LowerThan(usize),
    Range(Range<usize>),
    GreaterOrEqual(usize),
}

impl TimeFilter {
    pub fn contains(&self, days: usize) -> bool {
        match self {
            TimeFilter::LowerThan(max) => days < *max,
            TimeFilter::Range(r) => r.contains(&days),
            TimeFilter::GreaterOrEqual(min) => days >= *min,
        }
    }

    pub fn includes(&self, other: &TimeFilter) -> bool {
        match self {
            TimeFilter::LowerThan(max) => match other {
                TimeFilter::LowerThan(max2) => max >= max2,
                TimeFilter::Range(range) => range.end <= *max,
                TimeFilter::GreaterOrEqual(_) => false,
            },
            TimeFilter::GreaterOrEqual(min) => match other {
                TimeFilter::Range(range) => range.start >= *min,
                TimeFilter::GreaterOrEqual(min2) => min <= min2,
                TimeFilter::LowerThan(_) => false,
            },
            TimeFilter::Range(_) => false,
        }
    }

    /// Returns true if this filter includes day 0 (UTXOs less than 1 day old)
    pub fn includes_first_day(&self) -> bool {
        match self {
            TimeFilter::LowerThan(_) => true,
            TimeFilter::Range(r) => r.start == 0,
            TimeFilter::GreaterOrEqual(_) => false,
        }
    }

    pub fn to_name_suffix(&self) -> String {
        match self {
            // Special cases for common filters
            TimeFilter::LowerThan(1) => "up_to_1d_old".to_string(),
            TimeFilter::LowerThan(7) => "up_to_1w_old".to_string(),
            TimeFilter::LowerThan(30) => "up_to_1m_old".to_string(),
            TimeFilter::LowerThan(60) => "up_to_2m_old".to_string(),
            TimeFilter::LowerThan(90) => "up_to_3m_old".to_string(),
            TimeFilter::LowerThan(120) => "up_to_4m_old".to_string(),
            TimeFilter::LowerThan(150) => "sth".to_string(),
            TimeFilter::LowerThan(180) => "up_to_6m_old".to_string(),
            TimeFilter::LowerThan(365) => "up_to_1y_old".to_string(),
            TimeFilter::LowerThan(730) => "up_to_2y_old".to_string(),
            TimeFilter::LowerThan(1095) => "up_to_3y_old".to_string(),
            TimeFilter::LowerThan(1460) => "up_to_4y_old".to_string(),
            TimeFilter::LowerThan(1825) => "up_to_5y_old".to_string(),
            TimeFilter::LowerThan(2190) => "up_to_6y_old".to_string(),
            TimeFilter::LowerThan(2555) => "up_to_7y_old".to_string(),
            TimeFilter::LowerThan(2920) => "up_to_8y_old".to_string(),
            TimeFilter::LowerThan(3650) => "up_to_10y_old".to_string(),
            TimeFilter::LowerThan(4380) => "up_to_12y_old".to_string(),
            TimeFilter::LowerThan(5475) => "up_to_15y_old".to_string(),

            TimeFilter::GreaterOrEqual(1) => "at_least_1d_old".to_string(),
            TimeFilter::GreaterOrEqual(7) => "at_least_1w_old".to_string(),
            TimeFilter::GreaterOrEqual(30) => "at_least_1m_old".to_string(),
            TimeFilter::GreaterOrEqual(60) => "at_least_2m_old".to_string(),
            TimeFilter::GreaterOrEqual(90) => "at_least_3m_old".to_string(),
            TimeFilter::GreaterOrEqual(120) => "at_least_4m_old".to_string(),
            TimeFilter::GreaterOrEqual(150) => "lth".to_string(),
            TimeFilter::GreaterOrEqual(180) => "at_least_6m_old".to_string(),
            TimeFilter::GreaterOrEqual(365) => "at_least_1y_old".to_string(),
            TimeFilter::GreaterOrEqual(730) => "at_least_2y_old".to_string(),
            TimeFilter::GreaterOrEqual(1095) => "at_least_3y_old".to_string(),
            TimeFilter::GreaterOrEqual(1460) => "at_least_4y_old".to_string(),
            TimeFilter::GreaterOrEqual(1825) => "at_least_5y_old".to_string(),
            TimeFilter::GreaterOrEqual(2190) => "at_least_6y_old".to_string(),
            TimeFilter::GreaterOrEqual(2555) => "at_least_7y_old".to_string(),
            TimeFilter::GreaterOrEqual(2920) => "at_least_8y_old".to_string(),
            TimeFilter::GreaterOrEqual(3650) => "at_least_10y_old".to_string(),
            TimeFilter::GreaterOrEqual(4380) => "at_least_12y_old".to_string(),
            TimeFilter::GreaterOrEqual(5475) => "at_least_15y_old".to_string(),

            // Range special cases
            TimeFilter::Range(r) if *r == (0..1) => "up_to_1d".to_string(),
            TimeFilter::Range(r) if *r == (1..7) => "at_least_1d_up_to_1w_old".to_string(),
            TimeFilter::Range(r) if *r == (7..30) => "at_least_1w_up_to_1m_old".to_string(),
            TimeFilter::Range(r) if *r == (30..60) => "at_least_1m_up_to_2m_old".to_string(),
            TimeFilter::Range(r) if *r == (60..90) => "at_least_2m_up_to_3m_old".to_string(),
            TimeFilter::Range(r) if *r == (90..120) => "at_least_3m_up_to_4m_old".to_string(),
            TimeFilter::Range(r) if *r == (120..150) => "at_least_4m_up_to_5m_old".to_string(),
            TimeFilter::Range(r) if *r == (150..180) => "at_least_5m_up_to_6m_old".to_string(),
            TimeFilter::Range(r) if *r == (180..365) => "at_least_6m_up_to_1y_old".to_string(),
            TimeFilter::Range(r) if *r == (365..730) => "at_least_1y_up_to_2y_old".to_string(),
            TimeFilter::Range(r) if *r == (730..1095) => "at_least_2y_up_to_3y_old".to_string(),
            TimeFilter::Range(r) if *r == (1095..1460) => "at_least_3y_up_to_4y_old".to_string(),
            TimeFilter::Range(r) if *r == (1460..1825) => "at_least_4y_up_to_5y_old".to_string(),
            TimeFilter::Range(r) if *r == (1825..2190) => "at_least_5y_up_to_6y_old".to_string(),
            TimeFilter::Range(r) if *r == (2190..2555) => "at_least_6y_up_to_7y_old".to_string(),
            TimeFilter::Range(r) if *r == (2555..2920) => "at_least_7y_up_to_8y_old".to_string(),
            TimeFilter::Range(r) if *r == (2920..3650) => "at_least_8y_up_to_10y_old".to_string(),
            TimeFilter::Range(r) if *r == (3650..4380) => "at_least_10y_up_to_12y_old".to_string(),
            TimeFilter::Range(r) if *r == (4380..5475) => "at_least_12y_up_to_15y_old".to_string(),

            // Fallback generic names
            TimeFilter::LowerThan(d) => format!("up_to_{}d", d),
            TimeFilter::GreaterOrEqual(d) => format!("at_least_{}d", d),
            TimeFilter::Range(r) => format!("{}d_to_{}d", r.start, r.end),
        }
    }
}
