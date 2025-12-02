use std::ops::Range;

use brk_types::Sats;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmountFilter {
    LowerThan(Sats),
    Range(Range<Sats>),
    GreaterOrEqual(Sats),
}

impl AmountFilter {
    pub fn contains(&self, sats: Sats) -> bool {
        match self {
            AmountFilter::LowerThan(max) => sats < *max,
            AmountFilter::Range(r) => sats >= r.start && sats < r.end,
            AmountFilter::GreaterOrEqual(min) => sats >= *min,
        }
    }

    pub fn includes(&self, other: &AmountFilter) -> bool {
        match self {
            AmountFilter::LowerThan(max) => match other {
                AmountFilter::LowerThan(max2) => max >= max2,
                AmountFilter::Range(range) => range.end <= *max,
                AmountFilter::GreaterOrEqual(_) => false,
            },
            AmountFilter::GreaterOrEqual(min) => match other {
                AmountFilter::Range(range) => range.start >= *min,
                AmountFilter::GreaterOrEqual(min2) => min <= min2,
                AmountFilter::LowerThan(_) => false,
            },
            AmountFilter::Range(_) => false,
        }
    }

    pub fn to_name_suffix(&self) -> String {
        match self {
            AmountFilter::LowerThan(s) if *s == Sats::_1 => "with_0sats".to_string(),
            AmountFilter::LowerThan(s) => format!("under_{}", format_sats(*s)),
            AmountFilter::GreaterOrEqual(s) => format!("above_{}", format_sats(*s)),
            AmountFilter::Range(r) => {
                format!("{}_{}", format_sats(r.start), format_sats(r.end))
            }
        }
    }
}

fn format_sats(sats: Sats) -> String {
    match sats {
        s if s == Sats::ZERO => "0sats".to_string(),
        s if s == Sats::_1 => "1sat".to_string(),
        s if s == Sats::_10 => "10sats".to_string(),
        s if s == Sats::_100 => "100sats".to_string(),
        s if s == Sats::_1K => "1k_sats".to_string(),
        s if s == Sats::_10K => "10k_sats".to_string(),
        s if s == Sats::_100K => "100k_sats".to_string(),
        s if s == Sats::_1M => "1m_sats".to_string(),
        s if s == Sats::_10M => "10m_sats".to_string(),
        s if s == Sats::_1BTC => "1btc".to_string(),
        s if s == Sats::_10BTC => "10btc".to_string(),
        s if s == Sats::_100BTC => "100btc".to_string(),
        s if s == Sats::_1K_BTC => "1k_btc".to_string(),
        s if s == Sats::_10K_BTC => "10k_btc".to_string(),
        s if s == Sats::_100K_BTC => "100k_btc".to_string(),
        _ => format!("{}sats", u64::from(sats)),
    }
}
