//! Compile-time type-state for price-dependent data.
//!
// TODO: Remove this once Phase 3 (Pricing migration) is complete
#![allow(dead_code)]
//!
//! This module provides the `Pricing` trait which enables compile-time
//! differentiation between priced and unpriced data variants. Instead of
//! using `Option<T>` for price-dependent fields, structs use `P: Pricing`
//! with associated types that are either concrete types (for `Priced`) or
//! `()` (for `Unpriced`).
//!
//! Benefits:
//! - LSP/autocomplete visibility: no Options cluttering suggestions
//! - Compile-time guarantees: cannot access price data on `Unpriced` variants
//! - Zero runtime overhead: `()` is a ZST (zero-sized type)

use brk_traversable::Traversable;

/// Type-state trait for price-dependent data.
///
/// Implements the type-state pattern using associated types:
/// - `Priced`: associated types resolve to concrete data types
/// - `Unpriced`: associated types resolve to `()`
///
/// # Associated Types
///
/// | Type | Usage | Priced | Unpriced |
/// |------|-------|--------|----------|
/// | `Data` | Computer top-level | `PricingData` | `()` |
/// | `PriceRef<'a>` | Function params | `&price::Vecs` | `()` |
/// | `ComputedDollarsHeight` | Value wrappers (Height) | `ComputedFromHeight<Dollars>` | `()` |
/// | `ComputedDollarsDateIndex` | Value wrappers (DateIndex) | `ComputedVecsDate<Dollars>` | `()` |
/// | `StdDevBandsUsd` | StdDev USD bands | `StdDevBandsUsdData` | `()` |
/// | `RatioUsd` | Ratio USD variants | `RatioUsdData` | `()` |
/// | `BasePriced` | Base metrics | `BasePricedData` | `()` |
/// | `ExtendedPriced` | Extended metrics | `ExtendedPricedData` | `()` |
/// | `AdjustedPriced` | Adjusted metrics | `AdjustedPricedData` | `()` |
/// | `RelToAllPriced` | Rel-to-all metrics | `RelToAllPricedData` | `()` |
pub trait Pricing: 'static + Clone + Send + Sync {
    // === Top-level ===

    /// Top-level pricing data - PricingData for Priced, () for Unpriced
    type Data: Clone + Send + Sync + Traversable;

    /// Reference to price vecs for import functions
    type PriceRef<'a>: Copy;

    // === Value wrappers (used in 20+ places) ===

    /// Computed dollars with Height index
    type ComputedDollarsHeight: Clone + Send + Sync + Traversable;

    /// Computed dollars with DateIndex index
    type ComputedDollarsDateIndex: Clone + Send + Sync + Traversable;

    // === Specialized structs ===

    /// StdDev USD bands (13 fields grouped)
    type StdDevBandsUsd: Clone + Send + Sync + Traversable;

    /// Ratio USD data
    type RatioUsd: Clone + Send + Sync + Traversable;

    // === Distribution metrics ===

    /// Base-level priced metrics (realized + unrealized)
    type BasePriced: Clone + Send + Sync + Traversable;

    /// Extended-level priced metrics
    type ExtendedPriced: Clone + Send + Sync + Traversable;

    /// Adjusted metrics
    type AdjustedPriced: Clone + Send + Sync + Traversable;

    /// Dollar-based relative-to-all metrics
    type RelToAllPriced: Clone + Send + Sync + Traversable;
}

/// Marker type for priced data.
///
/// When `P = Priced`, all associated types resolve to their concrete
/// data types containing price-denominated values.
#[derive(Clone, Copy, Default, Debug)]
pub struct Priced;

/// Marker type for unpriced data.
///
/// When `P = Unpriced`, all associated types resolve to `()`,
/// effectively removing those fields at compile time with zero overhead.
#[derive(Clone, Copy, Default, Debug)]
pub struct Unpriced;

// Note: The actual type implementations for `Priced` and `Unpriced`
// will be added in Phase 3 when we migrate the concrete data types.
// For now, we provide placeholder implementations using () for all types
// to allow incremental migration.

impl Pricing for Priced {
    // Placeholder implementations - will be replaced with concrete types in Phase 3
    type Data = ();
    type PriceRef<'a> = ();
    type ComputedDollarsHeight = ();
    type ComputedDollarsDateIndex = ();
    type StdDevBandsUsd = ();
    type RatioUsd = ();
    type BasePriced = ();
    type ExtendedPriced = ();
    type AdjustedPriced = ();
    type RelToAllPriced = ();
}

impl Pricing for Unpriced {
    type Data = ();
    type PriceRef<'a> = ();
    type ComputedDollarsHeight = ();
    type ComputedDollarsDateIndex = ();
    type StdDevBandsUsd = ();
    type RatioUsd = ();
    type BasePriced = ();
    type ExtendedPriced = ();
    type AdjustedPriced = ();
    type RelToAllPriced = ();
}
