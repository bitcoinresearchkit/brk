use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ValueBlockSumCum};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        let indexes_to_opreturn = ValueBlockSumCum::forced_import(
            db,
            "opreturn_supply",
            version,
            indexes,
            compute_dollars,
        )?;

        let indexes_to_unspendable = ValueBlockSumCum::forced_import(
            db,
            "unspendable_supply",
            version,
            indexes,
            compute_dollars,
        )?;

        Ok(Self {
            indexes_to_opreturn,
            indexes_to_unspendable,
        })
    }
}
