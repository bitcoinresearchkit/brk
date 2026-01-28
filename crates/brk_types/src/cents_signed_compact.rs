use serde::{Deserialize, Serialize};

use super::Dollars;

/// Compact signed cents (i32) - memory-efficient for map keys.
/// Supports prices from -$21,474,836.47 to $21,474,836.47 (i32 range / 100).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CentsSignedCompact(i32);

impl CentsSignedCompact {
    pub const ZERO: Self = Self(0);

    #[inline]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn inner(self) -> i32 {
        self.0
    }

    #[inline]
    pub fn is_negative(self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub fn to_dollars(self) -> Dollars {
        Dollars::from(self.0 as f64 / 100.0)
    }

    #[inline]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl From<Dollars> for CentsSignedCompact {
    #[inline]
    fn from(value: Dollars) -> Self {
        let f = f64::from(value);
        if f.is_nan() {
            Self::ZERO
        } else {
            let cents = (f * 100.0).round();
            debug_assert!(
                cents >= i32::MIN as f64 && cents <= i32::MAX as f64,
                "Price ${} exceeds CentsSignedCompact range (~$21.5M)",
                f
            );
            Self(cents as i32)
        }
    }
}

impl From<CentsSignedCompact> for Dollars {
    #[inline]
    fn from(value: CentsSignedCompact) -> Self {
        value.to_dollars()
    }
}

impl From<i32> for CentsSignedCompact {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<CentsSignedCompact> for i32 {
    #[inline]
    fn from(value: CentsSignedCompact) -> Self {
        value.0
    }
}

impl From<CentsSignedCompact> for f64 {
    #[inline]
    fn from(value: CentsSignedCompact) -> Self {
        value.0 as f64
    }
}

impl std::fmt::Display for CentsSignedCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}
