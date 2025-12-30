use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, PAGE_SIZE};

use crate::{indexes, price};

use super::{
    AthVecs, DcaVecs, HistoryVecs, MovingAverageVecs, RangeVecs, Vecs, VolatilityVecs,
};

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join(super::DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let version = parent_version + Version::ZERO;

        let price = price.expect("price required for market");

        let ath = AthVecs::forced_import(&db, version, indexes, price)?;
        let volatility = VolatilityVecs::forced_import(&db, version, indexes)?;
        let range = RangeVecs::forced_import(&db, version, indexes)?;
        let moving_average = MovingAverageVecs::forced_import(&db, version, indexes, Some(price))?;
        let history = HistoryVecs::forced_import(&db, version, indexes, price)?;
        let dca = DcaVecs::forced_import(&db, version, indexes, price)?;

        let this = Self {
            db,
            ath,
            volatility,
            range,
            moving_average,
            history,
            dca,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

        Ok(this)
    }
}
