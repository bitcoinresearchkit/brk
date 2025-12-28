use brk_types::{Bitcoin, Close, Dollars, Sats, StoredF32};
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

/// Sats -> Sats/2 (for supply_half)
pub struct HalveSats;

impl UnaryTransform<Sats, Sats> for HalveSats {
    #[inline(always)]
    fn apply(sats: Sats) -> Sats {
        sats / 2
    }
}

/// Sats -> Bitcoin/2 (halve then convert to bitcoin)
/// Avoids lazy-from-lazy by combining both transforms
pub struct HalveSatsToBitcoin;

impl UnaryTransform<Sats, Bitcoin> for HalveSatsToBitcoin {
    #[inline(always)]
    fn apply(sats: Sats) -> Bitcoin {
        Bitcoin::from(sats / 2)
    }
}

/// Dollars -> Dollars/2 (for supply_half_usd)
pub struct HalveDollars;

impl UnaryTransform<Dollars, Dollars> for HalveDollars {
    #[inline(always)]
    fn apply(dollars: Dollars) -> Dollars {
        dollars.halved()
    }
}

/// Dollars * StoredF32 -> Dollars (price × ratio)
pub struct PriceTimesRatio;

impl BinaryTransform<Dollars, StoredF32, Dollars> for PriceTimesRatio {
    #[inline(always)]
    fn apply(price: Dollars, ratio: StoredF32) -> Dollars {
        price * ratio
    }
}

/// Close<Dollars> * Sats -> Dollars (price × sats / 1e8)
/// Same as PriceTimesSats but accepts Close<Dollars> price source.
pub struct ClosePriceTimesSats;

impl BinaryTransform<Close<Dollars>, Sats, Dollars> for ClosePriceTimesSats {
    #[inline(always)]
    fn apply(price: Close<Dollars>, sats: Sats) -> Dollars {
        *price * Bitcoin::from(sats)
    }
}

/// Close<Dollars> * Sats -> Dollars/2 (price × sats / 1e8 / 2)
/// Computes halved dollars directly from sats, avoiding lazy-from-lazy chains.
pub struct HalfClosePriceTimesSats;

impl BinaryTransform<Close<Dollars>, Sats, Dollars> for HalfClosePriceTimesSats {
    #[inline(always)]
    fn apply(price: Close<Dollars>, sats: Sats) -> Dollars {
        (*price * Bitcoin::from(sats)).halved()
    }
}

// === Constant Transforms (using const generics) ===

use brk_types::{StoredI16, StoredU16};

/// Returns a constant u16 value, ignoring the input.
pub struct ReturnU16<const V: u16>;

impl<S, const V: u16> UnaryTransform<S, StoredU16> for ReturnU16<V> {
    #[inline(always)]
    fn apply(_: S) -> StoredU16 {
        StoredU16::new(V)
    }
}

/// Returns a constant i16 value, ignoring the input.
pub struct ReturnI16<const V: i16>;

impl<S, const V: i16> UnaryTransform<S, StoredI16> for ReturnI16<V> {
    #[inline(always)]
    fn apply(_: S) -> StoredI16 {
        StoredI16::new(V)
    }
}

/// Returns a constant f32 value from tenths (V=382 -> 38.2), ignoring the input.
pub struct ReturnF32Tenths<const V: u16>;

impl<S, const V: u16> UnaryTransform<S, StoredF32> for ReturnF32Tenths<V> {
    #[inline(always)]
    fn apply(_: S) -> StoredF32 {
        StoredF32::from(V as f32 / 10.0)
    }
}

/// Dollars * (V/10) -> Dollars (e.g., V=8 -> * 0.8, V=24 -> * 2.4)
pub struct DollarsTimesTenths<const V: u16>;

impl<const V: u16> UnaryTransform<Dollars, Dollars> for DollarsTimesTenths<V> {
    #[inline(always)]
    fn apply(d: Dollars) -> Dollars {
        d * (V as f64 / 10.0)
    }
}