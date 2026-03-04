use std::path::Path;

use brk_error::Result;
use brk_types::Version;

use crate::{
    indexes,
    internal::{finalize_db, open_db},
};

use super::{
    AthVecs, DcaVecs, IndicatorsVecs, LookbackVecs, MovingAverageVecs, RangeVecs, ReturnsVecs,
    Vecs, VolatilityVecs,
};

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = open_db(parent_path, super::DB_NAME, 1_000_000)?;
        let version = parent_version;

        let ath = AthVecs::forced_import(&db, version, indexes)?;
        let lookback = LookbackVecs::forced_import(&db, version, indexes)?;
        let returns = ReturnsVecs::forced_import(&db, version, indexes)?;
        let volatility = VolatilityVecs::forced_import(&db, version, indexes, &returns)?;
        let range = RangeVecs::forced_import(&db, version, indexes)?;
        let moving_average = MovingAverageVecs::forced_import(&db, version, indexes)?;
        let dca = DcaVecs::forced_import(&db, version, indexes)?;
        let indicators = IndicatorsVecs::forced_import(&db, version, indexes)?;

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
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
