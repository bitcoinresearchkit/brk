use serde::{Deserialize, Serialize};

use super::Dollars;

/// Compact representation of USD cents as i32.
///
/// Used as a memory-efficient BTreeMap key instead of Dollars (f64).
/// Supports prices from $0.00 to $21,474,836.47 (i32::MAX / 100).
///
/// Memory savings: 4 bytes vs 8 bytes per key, plus eliminates
/// floating-point precision issues that create duplicate keys.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CentsCompact(i32);

impl CentsCompact {
    pub const ZERO: Self = Self(0);

    /// Convert to Dollars for display/computation
    #[inline]
    pub fn to_dollars(self) -> Dollars {
        Dollars::from(self.0 as f64 / 100.0)
    }
}

impl From<Dollars> for CentsCompact {
    #[inline]
    fn from(value: Dollars) -> Self {
        let f = f64::from(value);
        if f.is_nan() || f < 0.0 {
            Self::ZERO
        } else {
            let cents = (f * 100.0).round();
            assert!(
                cents <= i32::MAX as f64,
                "Price ${} exceeds CentsCompact max (~$21.5M)",
                f
            );
            Self(cents as i32)
        }
    }
}

impl From<i32> for CentsCompact {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<CentsCompact> for i32 {
    #[inline]
    fn from(value: CentsCompact) -> Self {
        value.0
    }
}

impl From<CentsCompact> for Dollars {
    #[inline]
    fn from(value: CentsCompact) -> Self {
        value.to_dollars()
    }
}
