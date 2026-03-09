use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints32, Cents, Height, Indexes, StoredF32, Version};
use vecdb::{Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{Bp32ToFloat, ComputedFromHeight, LazyFromHeight},
};

#[derive(Traversable)]
pub struct ComputedFromHeightRatio<M: StorageMode = Rw> {
    pub bps: ComputedFromHeight<BasisPoints32, M>,
    pub ratio: LazyFromHeight<StoredF32, BasisPoints32>,
}

const VERSION: Version = Version::TWO;

impl ComputedFromHeightRatio {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Self::forced_import_raw(db, &format!("{name}_ratio"), version, indexes)
    }

    pub(crate) fn forced_import_raw(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let bps = ComputedFromHeight::forced_import(db, &format!("{name}_bps"), v, indexes)?;

        let ratio = LazyFromHeight::from_computed::<Bp32ToFloat>(
            name,
            v,
            bps.height.read_only_boxed_clone(),
            &bps,
        );

        Ok(Self { bps, ratio })
    }

    pub(crate) fn compute_ratio(
        &mut self,
        starting_indexes: &Indexes,
        close_price: &impl ReadableVec<Height, Cents>,
        metric_price: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        self.bps.height.compute_transform2(
            starting_indexes.height,
            close_price,
            metric_price,
            |(i, close, price, ..)| {
                if price == Cents::ZERO {
                    (i, BasisPoints32::from(1.0))
                } else {
                    (i, BasisPoints32::from(f64::from(close) / f64::from(price)))
                }
            },
            exit,
        )?;
        Ok(())
    }
}
