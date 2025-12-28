use brk_types::{Bitcoin, Dollars, Sats, StoredF32, StoredF64};
use vecdb::{BinaryTransform, UnaryTransform};

/// (Dollars, Dollars) -> Dollars addition
/// Used for computing total = profit + loss
pub struct DollarsPlus;

impl BinaryTransform<Dollars, Dollars, Dollars> for DollarsPlus {
    #[inline(always)]
    fn apply(lhs: Dollars, rhs: Dollars) -> Dollars {
        lhs + rhs
    }
}

/// (Dollars, Dollars) -> Dollars subtraction
/// Used for computing net = profit - loss
pub struct DollarsMinus;

impl BinaryTransform<Dollars, Dollars, Dollars> for DollarsMinus {
    #[inline(always)]
    fn apply(lhs: Dollars, rhs: Dollars) -> Dollars {
        lhs - rhs
    }
}

/// (Dollars, Dollars) -> StoredF32 ratio
/// Used for computing percentage ratios like profit/total, loss/total, etc.
pub struct Ratio32;

impl BinaryTransform<Dollars, Dollars, StoredF32> for Ratio32 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> StoredF32 {
        StoredF32::from(numerator / denominator)
    }
}

/// (Dollars, Dollars) -> -StoredF32 (negated ratio)
/// Computes -(a/b) directly to avoid lazy-from-lazy chains.
pub struct NegRatio32;

impl BinaryTransform<Dollars, Dollars, StoredF32> for NegRatio32 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> StoredF32 {
        -StoredF32::from(numerator / denominator)
    }
}

// === Unary Transforms ===

/// Sats -> Bitcoin (divide by 1e8)
pub struct SatsToBitcoin;

impl UnaryTransform<Sats, Bitcoin> for SatsToBitcoin {
    #[inline(always)]
    fn apply(sats: Sats) -> Bitcoin {
        Bitcoin::from(sats)
    }
}

/// Sats -> StoredF64 via Bitcoin (for coinblocks/coindays)
pub struct SatsToStoredF64;

impl UnaryTransform<Sats, StoredF64> for SatsToStoredF64 {
    #[inline(always)]
    fn apply(sats: Sats) -> StoredF64 {
        StoredF64::from(Bitcoin::from(sats))
    }
}

/// Sats -> Sats/2 (for supply_half)
pub struct HalveSats;

impl UnaryTransform<Sats, Sats> for HalveSats {
    #[inline(always)]
    fn apply(sats: Sats) -> Sats {
        sats / 2
    }
}
