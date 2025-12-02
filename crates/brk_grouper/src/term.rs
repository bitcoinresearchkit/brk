/// Classification for short-term vs long-term holders.
/// The threshold is 150 days (approximately 5 months).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Term {
    /// Short-Term Holder: < 150 days
    Sth,
    /// Long-Term Holder: >= 150 days
    Lth,
}

impl Term {
    pub const THRESHOLD_DAYS: usize = 150;

    pub fn to_name(&self) -> &'static str {
        match self {
            Term::Sth => "sth",
            Term::Lth => "lth",
        }
    }
}
