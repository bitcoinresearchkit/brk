use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedVecsFromDateIndex, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        let v0 = Version::ZERO;

        let indexes_to_btc = ComputedVecsFromDateIndex::forced_import(
            db,
            "btc_velocity",
            Source::Compute,
            version + v0,
            indexes,
            VecBuilderOptions::default().add_average(),
        )?;

        let indexes_to_usd = compute_dollars.then(|| {
            ComputedVecsFromDateIndex::forced_import(
                db,
                "usd_velocity",
                Source::Compute,
                version + v0,
                indexes,
                VecBuilderOptions::default().add_average(),
            )
            .unwrap()
        });

        Ok(Self {
            indexes_to_btc,
            indexes_to_usd,
        })
    }
}
