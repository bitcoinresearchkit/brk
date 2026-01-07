use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedVecsDateAverage};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        let indexes_to_btc = ComputedVecsDateAverage::forced_import(
            db,
            "btc_velocity",
            version,
            indexes,
        )?;

        let indexes_to_usd = compute_dollars.then(|| {
            ComputedVecsDateAverage::forced_import(
                db,
                "usd_velocity",
                version,
                indexes,
            )
            .unwrap()
        });

        Ok(Self {
            indexes_to_btc,
            indexes_to_usd,
        })
    }
}
