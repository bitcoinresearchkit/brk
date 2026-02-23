use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::{ByLookbackPeriod, Vecs};
use crate::{indexes, internal::PriceFromHeight};

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let price_ago = ByLookbackPeriod::try_new(|name, _days| {
            PriceFromHeight::forced_import(db, &format!("price_{name}_ago"), version, indexes)
        })?;

        Ok(Self { price_ago })
    }
}
