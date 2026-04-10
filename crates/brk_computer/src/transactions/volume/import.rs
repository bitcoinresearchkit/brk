use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{AmountPerBlockCumulativeRolling, PerBlock, WindowStartVec, Windows},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let v = version + Version::TWO;
        Ok(Self {
            transfer_volume: AmountPerBlockCumulativeRolling::forced_import(
                db,
                "transfer_volume_bis",
                version,
                indexes,
                cached_starts,
            )?,
            tx_per_sec: Windows::try_from_fn(|suffix| {
                PerBlock::forced_import(db, &format!("tx_per_sec_{suffix}"), v, indexes)
            })?,
            outputs_per_sec: Windows::try_from_fn(|suffix| {
                PerBlock::forced_import(db, &format!("outputs_per_sec_{suffix}"), v, indexes)
            })?,
            inputs_per_sec: Windows::try_from_fn(|suffix| {
                PerBlock::forced_import(db, &format!("inputs_per_sec_{suffix}"), v, indexes)
            })?,
        })
    }
}
