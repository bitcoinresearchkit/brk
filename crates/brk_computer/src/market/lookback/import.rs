use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::{ByLookbackPeriod, Vecs};
use crate::{
    indexes,
    internal::{ComputedVecsFromDateIndex, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let last = VecBuilderOptions::default().add_last();

        let price_ago = ByLookbackPeriod::try_new(|name, _days| {
            ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("price_{name}_ago"),
                Source::Compute,
                version,
                indexes,
                last,
            )
        })?;

        Ok(Self { price_ago })
    }
}
