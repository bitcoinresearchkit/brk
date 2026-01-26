use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::{ByLookbackPeriod, Vecs};
use crate::{indexes, internal::Price};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let price_ago = ByLookbackPeriod::try_new(|name, _days| {
            Price::forced_import(db, &format!("price_{name}_ago"), version, indexes)
        })?;

        Ok(Self { price_ago })
    }
}
