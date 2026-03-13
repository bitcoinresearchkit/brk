use std::path::Path;

use brk_error::Result;
use brk_types::Version;

use super::Vecs;
use crate::{
    indexes,
    internal::{PerBlock, PercentPerBlock, RatioPerBlock, db_utils::{finalize_db, open_db}},
};

const VERSION: Version = Version::new(1);

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = open_db(parent_path, super::DB_NAME, 100_000)?;
        let v = parent_version + VERSION;

        let puell_multiple = RatioPerBlock::forced_import_raw(&db, "puell_multiple", v, indexes)?;
        let nvt = RatioPerBlock::forced_import_raw(&db, "nvt", v, indexes)?;
        let gini = PercentPerBlock::forced_import(&db, "gini", v, indexes)?;
        let rhodl_ratio = RatioPerBlock::forced_import_raw(&db, "rhodl_ratio", v, indexes)?;
        let thermocap_multiple =
            RatioPerBlock::forced_import_raw(&db, "thermocap_multiple", v, indexes)?;
        let coindays_destroyed_supply_adjusted =
            PerBlock::forced_import(&db, "coindays_destroyed_supply_adjusted", v, indexes)?;
        let coinyears_destroyed_supply_adjusted =
            PerBlock::forced_import(&db, "coinyears_destroyed_supply_adjusted", v, indexes)?;
        let dormancy = super::vecs::DormancyVecs {
            supply_adjusted: PerBlock::forced_import(&db, "dormancy_supply_adjusted", v, indexes)?,
            flow: PerBlock::forced_import(&db, "dormancy_flow", v, indexes)?,
        };
        let stock_to_flow = PerBlock::forced_import(&db, "stock_to_flow", v, indexes)?;
        let seller_exhaustion_constant =
            PerBlock::forced_import(&db, "seller_exhaustion_constant", v, indexes)?;

        let this = Self {
            db,
            puell_multiple,
            nvt,
            gini,
            rhodl_ratio,
            thermocap_multiple,
            coindays_destroyed_supply_adjusted,
            coinyears_destroyed_supply_adjusted,
            dormancy,
            stock_to_flow,
            seller_exhaustion_constant,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
