//! ComputedHeightDateBytes - height + dateindex BytesVec storage.
//!
//! Use this for simple cases where both height and dateindex are stored BytesVecs
//! without any lazy derivations. For OHLC-type data.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Height, Version};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{BytesVec, BytesVecValue, Database, Formattable, ImportableVec};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct ComputedHeightDateBytes<T>
where
    T: BytesVecValue + Formattable + Serialize + JsonSchema,
{
    pub height: BytesVec<Height, T>,
    pub dateindex: BytesVec<DateIndex, T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedHeightDateBytes<T>
where
    T: BytesVecValue + Formattable + Serialize + JsonSchema + 'static,
{
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        let v = version + VERSION;

        Ok(Self {
            height: BytesVec::forced_import(db, name, v)?,
            dateindex: BytesVec::forced_import(db, name, v)?,
        })
    }
}
