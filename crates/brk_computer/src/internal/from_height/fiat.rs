use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSigned, Dollars, Version};
use schemars::JsonSchema;
use vecdb::{Database, ReadableCloneableVec, Rw, StorageMode, UnaryTransform};

use super::{ComputedFromHeightLast, LazyFromHeightLast};
use crate::{
    indexes,
    internal::{CentsSignedToDollars, CentsUnsignedToDollars, NumericValue},
};

/// Trait that associates a cents type with its transform to Dollars.
pub trait CentsType: NumericValue + JsonSchema {
    type ToDollars: UnaryTransform<Self, Dollars>;
}

impl CentsType for Cents {
    type ToDollars = CentsUnsignedToDollars;
}

impl CentsType for CentsSigned {
    type ToDollars = CentsSignedToDollars;
}

/// Height-indexed fiat monetary value: cents (eager, integer) + usd (lazy, float).
/// Generic over `C` to support both `Cents` (unsigned) and `CentsSigned` (signed).
#[derive(Traversable)]
pub struct FiatFromHeightLast<C: CentsType, M: StorageMode = Rw> {
    pub cents: ComputedFromHeightLast<C, M>,
    pub usd: LazyFromHeightLast<Dollars, C>,
}

impl<C: CentsType> FiatFromHeightLast<C> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let cents = ComputedFromHeightLast::forced_import(
            db,
            &format!("{name}_cents"),
            version,
            indexes,
        )?;
        let usd = LazyFromHeightLast::from_computed::<C::ToDollars>(
            &format!("{name}_usd"),
            version,
            cents.height.read_only_boxed_clone(),
            &cents,
        );
        Ok(Self { cents, usd })
    }
}
