use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::Cents;

/// Bucket type for cost basis aggregation.
/// Options: raw (no aggregation), lin200/lin500/lin1000 (linear $200/$500/$1000),
/// log10/log50/log100/log200 (logarithmic with 10/50/100/200 buckets per decade).
#[derive(
    Debug, Display, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CostBasisBucket {
    #[default]
    Raw,
    Lin200,
    Lin500,
    Lin1000,
    Log10,
    Log50,
    Log100,
    Log200,
}

impl CostBasisBucket {
    /// Returns the linear bucket size in cents, if this is a linear bucket type.
    fn linear_size_cents(&self) -> Option<u64> {
        match self {
            Self::Lin200 => Some(20_000),
            Self::Lin500 => Some(50_000),
            Self::Lin1000 => Some(100_000),
            _ => None,
        }
    }

    /// Returns the number of buckets per decade, if this is a log bucket type.
    fn log_buckets_per_decade(&self) -> Option<u32> {
        match self {
            Self::Log10 => Some(10),
            Self::Log50 => Some(50),
            Self::Log100 => Some(100),
            Self::Log200 => Some(200),
            _ => None,
        }
    }

    /// Compute bucket floor for a given price in cents.
    /// Returns None for Raw (no bucketing).
    pub fn bucket_floor(&self, price_cents: Cents) -> Option<Cents> {
        match self {
            Self::Raw => None,
            Self::Lin200 | Self::Lin500 | Self::Lin1000 => {
                let size = self.linear_size_cents().unwrap();
                Some((price_cents / size) * size)
            }
            Self::Log10 | Self::Log50 | Self::Log100 | Self::Log200 => {
                if price_cents == Cents::ZERO {
                    return Some(Cents::ZERO);
                }
                let n = self.log_buckets_per_decade().unwrap();
                // Bucket index = floor(n * log10(price))
                // Floor = 10^(bucket_index / n)
                let log_price = f64::from(price_cents).log10();
                let bucket_idx = (n as f64 * log_price).floor() as i32;
                let floor = 10_f64.powf(bucket_idx as f64 / n as f64);
                Some(Cents::from(floor.round() as u64))
            }
        }
    }
}
