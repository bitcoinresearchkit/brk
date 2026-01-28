use serde::{Deserialize, Serialize};

use super::Dollars;

/// Compact unsigned cents (u32) - memory-efficient for map keys.
/// Supports values from $0.00 to $42,949,672.95 (u32::MAX / 100).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CentsUnsignedCompact(u32);

impl CentsUnsignedCompact {
    pub const ZERO: Self = Self(0);
    pub const MAX: Self = Self(u32::MAX);

    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn inner(self) -> u32 {
        self.0
    }

    #[inline]
    pub fn to_dollars(self) -> Dollars {
        Dollars::from(self.0 as f64 / 100.0)
    }

    #[inline]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }

    #[inline]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl From<Dollars> for CentsUnsignedCompact {
    #[inline]
    fn from(value: Dollars) -> Self {
        let f = f64::from(value);
        if f.is_nan() || f < 0.0 {
            Self::ZERO
        } else {
            let cents = (f * 100.0).round();
            debug_assert!(
                cents <= u32::MAX as f64,
                "Price ${} exceeds CentsUnsignedCompact max (~$42.9M)",
                f
            );
            Self(cents as u32)
        }
    }
}

impl From<CentsUnsignedCompact> for Dollars {
    #[inline]
    fn from(value: CentsUnsignedCompact) -> Self {
        value.to_dollars()
    }
}

impl From<u32> for CentsUnsignedCompact {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<CentsUnsignedCompact> for u32 {
    #[inline]
    fn from(value: CentsUnsignedCompact) -> Self {
        value.0
    }
}

impl From<CentsUnsignedCompact> for f64 {
    #[inline]
    fn from(value: CentsUnsignedCompact) -> Self {
        value.0 as f64
    }
}

impl std::fmt::Display for CentsUnsignedCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
