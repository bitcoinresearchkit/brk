use crate::{Sats, Term, Timestamp};

/// Represents the age of a UTXO or address balance.
/// Encapsulates all age-related calculations in one type-safe struct.
#[derive(Debug, Clone, Copy)]
pub struct Age {
    /// Age in hours (primary internal unit for cohort boundaries)
    hours: usize,
    /// Age in blocks (for satblocks_destroyed calculation)
    blocks: usize,
    /// Age in days as float (for satdays_destroyed - established terminology)
    days: f64,
}

impl Age {
    /// Create from timestamps and block count
    #[inline]
    pub fn new(current_timestamp: Timestamp, prev_timestamp: Timestamp, blocks: usize) -> Self {
        Self {
            hours: current_timestamp.difference_in_hours_between(prev_timestamp),
            blocks,
            days: current_timestamp.difference_in_days_between_float(prev_timestamp),
        }
    }

    /// Hours old (for cohort bucket lookup via HOURS_* boundaries)
    #[inline]
    pub fn hours(&self) -> usize {
        self.hours
    }

    /// Blocks old (for satblocks_destroyed calculation)
    #[inline]
    pub fn blocks(&self) -> usize {
        self.blocks
    }

    /// Days old as float (for satdays_destroyed - established terminology)
    #[inline]
    pub fn days(&self) -> f64 {
        self.days
    }

    /// STH or LTH based on age (5 months = 3600 hours threshold)
    #[inline]
    pub fn term(&self) -> Term {
        if self.hours >= Term::THRESHOLD_HOURS {
            Term::Lth
        } else {
            Term::Sth
        }
    }

    /// Calculate satblocks destroyed for given supply
    #[inline]
    pub fn satblocks_destroyed(&self, supply: Sats) -> Sats {
        Sats::from(u64::from(supply) * self.blocks as u64)
    }

    /// Calculate satdays destroyed for given supply
    #[inline]
    pub fn satdays_destroyed(&self, supply: Sats) -> Sats {
        Sats::from((u64::from(supply) as f64 * self.days).floor() as u64)
    }
}
