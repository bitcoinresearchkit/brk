use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, PAGE_SIZE};

use crate::{distribution, indexes, price, transactions};

use super::{
    AthVecs, DcaVecs, IndicatorsVecs, LookbackVecs, MovingAverageVecs, RangeVecs, ReturnsVecs,
    Vecs, VolatilityVecs,
};

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        distribution: &distribution::Vecs,
        transactions: &transactions::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join(super::DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let version = parent_version;

        let price = price.expect("price required for market");

        let ath = AthVecs::forced_import(&db, version, indexes, price)?;
        let lookback = LookbackVecs::forced_import(&db, version, indexes)?;
        let returns = ReturnsVecs::forced_import(&db, version, indexes, price, &lookback)?;
        let volatility = VolatilityVecs::forced_import(version, &returns);
        let range = RangeVecs::forced_import(&db, version, indexes)?;
        let moving_average = MovingAverageVecs::forced_import(&db, version, indexes, Some(price))?;
        let dca = DcaVecs::forced_import(&db, version, indexes, price)?;
        let indicators = IndicatorsVecs::forced_import(
            &db,
            version,
            indexes,
            true,
            distribution,
            transactions,
            &moving_average,
        )?;

        let this = Self {
            db,
            ath,
            lookback,
            returns,
            volatility,
            range,
            moving_average,
            dca,
            indicators,
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
