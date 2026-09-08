use bitview_compute::NumericValue;
use bitview_transforms::{CentsSignedToDollars, CentsUnsignedToDollars};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Cents, CentsSigned, Dollars, Version};
use schemars::JsonSchema;
use vecdb::{CacheBudget, Database, Rw, StorageMode, UnaryTransform};

use crate::{IndexSources, LazyPerBlock, PerBlock};

/// Trait that associates a cents type with its transform to Dollars.
pub trait FiatType: NumericValue + JsonSchema {
    type ToDollars: UnaryTransform<Self, Dollars>;
}

impl FiatType for Cents {
    type ToDollars = CentsUnsignedToDollars;
}

impl FiatType for CentsSigned {
    type ToDollars = CentsSignedToDollars;
}

/// Height-indexed fiat monetary value: cents (eager, integer) + usd (lazy, float).
/// Generic over `C` to support both `Cents` (unsigned) and `CentsSigned` (signed).
#[derive(Traversable)]
pub struct FiatPerBlock<C: FiatType, M: StorageMode = Rw> {
    /// Reported in US dollars.
    pub usd: LazyPerBlock<Dollars, C>,
    /// Reported in US cents; 100 cents equal one US dollar.
    pub cents: PerBlock<C, M>,
}

impl<C: FiatType> FiatPerBlock<C> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        let cents = PerBlock::forced_import(cache, db, &format!("{name}_cents"), version, indexes)?;
        let usd = LazyPerBlock::from_resolutions::<C::ToDollars>(name, version, &cents);
        Ok(Self { usd, cents })
    }
}
