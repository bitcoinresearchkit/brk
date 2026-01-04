use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedValueVecsFromHeight, Source, VecBuilderOptions},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        let height_to_opreturn = EagerVec::forced_import(db, "opreturn_supply", version)?;

        let indexes_to_opreturn = ComputedValueVecsFromHeight::forced_import(
            db,
            "opreturn_supply",
            Source::Vec(height_to_opreturn.boxed_clone()),
            version,
            VecBuilderOptions::default().add_last().add_cumulative(),
            compute_dollars,
            indexes,
        )?;

        let height_to_unspendable = EagerVec::forced_import(db, "unspendable_supply", version)?;

        let indexes_to_unspendable = ComputedValueVecsFromHeight::forced_import(
            db,
            "unspendable_supply",
            Source::Vec(height_to_unspendable.boxed_clone()),
            version,
            VecBuilderOptions::default().add_last().add_cumulative(),
            compute_dollars,
            indexes,
        )?;

        Ok(Self {
            height_to_opreturn,
            height_to_unspendable,
            indexes_to_opreturn,
            indexes_to_unspendable,
        })
    }
}
