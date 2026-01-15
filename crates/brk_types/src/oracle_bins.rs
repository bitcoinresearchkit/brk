use std::{fmt::Display, mem::size_of};

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::SeqAccess, de::Visitor};
use vecdb::{Bytes, Formattable};

use crate::Sats;

/// Number of bins for the phase histogram
pub const PHASE_BINS: usize = 100;

/// Phase histogram: counts per bin for frac(log10(sats))
///
/// Used for on-chain price discovery. Each bin represents 1% of the
/// log10 fractional range [0, 1). Values are u16 (max 65535 per bin).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OracleBins {
    pub bins: [u16; PHASE_BINS],
}

impl Default for OracleBins {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Display for OracleBins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OracleBins(peak={})", self.peak_bin())
    }
}

impl Serialize for OracleBins {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.bins.as_slice().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OracleBins {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct BinsVisitor;

        impl<'de> Visitor<'de> for BinsVisitor {
            type Value = OracleBins;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an array of {} u16 values", PHASE_BINS)
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut bins = [0u16; PHASE_BINS];
                for (i, bin) in bins.iter_mut().enumerate() {
                    *bin = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;
                }
                Ok(OracleBins { bins })
            }
        }

        deserializer.deserialize_seq(BinsVisitor)
    }
}

impl JsonSchema for OracleBins {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OracleBins".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // Represent as array of u16 values
        Vec::<u16>::json_schema(_gen)
    }
}

impl OracleBins {
    pub const ZERO: Self = Self {
        bins: [0; PHASE_BINS],
    };

    /// Get the bin index for a sats value
    /// Filters: min 1k sats, max 100k BTC (matches Python 1e-5 to 1e5 BTC)
    #[inline]
    pub fn sats_to_bin(sats: Sats) -> Option<usize> {
        if sats < Sats::_1K || sats > Sats::_100K_BTC {
            return None;
        }
        let log_sats = f64::from(sats).log10();
        let phase = log_sats.fract();
        let phase = if phase < 0.0 { phase + 1.0 } else { phase };
        Some(((phase * PHASE_BINS as f64) as usize).min(PHASE_BINS - 1))
    }

    /// Add a count to the bin for this sats value
    #[inline]
    pub fn add(&mut self, sats: Sats) {
        if let Some(bin) = Self::sats_to_bin(sats) {
            self.bins[bin] = self.bins[bin].saturating_add(1);
        }
    }

    /// Find the peak bin (index with highest count)
    pub fn peak_bin(&self) -> usize {
        self.bins
            .iter()
            .enumerate()
            .max_by_key(|(_, count)| *count)
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    /// Get total count across all bins
    pub fn total_count(&self) -> u32 {
        self.bins.iter().map(|&c| c as u32).sum()
    }
}

impl Bytes for OracleBins {
    type Array = [u8; size_of::<Self>()];

    fn to_bytes(&self) -> Self::Array {
        let mut arr = [0u8; size_of::<Self>()];
        for (i, &count) in self.bins.iter().enumerate() {
            let bytes = count.to_le_bytes();
            arr[i * 2] = bytes[0];
            arr[i * 2 + 1] = bytes[1];
        }
        arr
    }

    fn from_bytes(bytes: &[u8]) -> vecdb::Result<Self> {
        if bytes.len() < size_of::<Self>() {
            return Err(vecdb::Error::WrongLength {
                received: bytes.len(),
                expected: size_of::<Self>(),
            });
        }
        let mut bins = [0u16; PHASE_BINS];
        for (i, bin) in bins.iter_mut().enumerate() {
            *bin = u16::from_le_bytes([bytes[i * 2], bytes[i * 2 + 1]]);
        }
        Ok(Self { bins })
    }
}

impl Formattable for OracleBins {
    fn may_need_escaping() -> bool {
        false
    }
}
