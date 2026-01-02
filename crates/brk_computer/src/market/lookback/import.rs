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
    ) -> Result<Self> {
        let v0 = Version::ZERO;
        let last = VecBuilderOptions::default().add_last();

        let price_1d_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_1d_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_1w_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_1w_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_1m_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_1m_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_3m_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_3m_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_6m_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_6m_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_1y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_1y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_2y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_2y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_3y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_3y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_4y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_4y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_5y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_5y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_6y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_6y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_8y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_8y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_10y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_10y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;

        Ok(Self {
            price_1d_ago,
            price_1w_ago,
            price_1m_ago,
            price_3m_ago,
            price_6m_ago,
            price_1y_ago,
            price_2y_ago,
            price_3y_ago,
            price_4y_ago,
            price_5y_ago,
            price_6y_ago,
            price_8y_ago,
            price_10y_ago,
        })
    }
}
