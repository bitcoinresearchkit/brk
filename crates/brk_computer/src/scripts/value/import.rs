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
        let height_to_opreturn_value = EagerVec::forced_import(db, "opreturn_value", version)?;

        let indexes_to_opreturn_value = ComputedValueVecsFromHeight::forced_import(
            db,
            "opreturn_value",
            Source::Vec(height_to_opreturn_value.boxed_clone()),
            version,
            VecBuilderOptions::default()
                .add_sum()
                .add_cumulative()
                .add_average()
                .add_minmax(),
            compute_dollars,
            indexes,
        )?;

        Ok(Self {
            height_to_opreturn_value,
            indexes_to_opreturn_value,
        })
    }
}
