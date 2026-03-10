use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredF32, Version};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, indexes, internal::StdDevPerBlockExtended};

#[derive(Traversable)]
pub struct RatioPerBlockStdDevBands<M: StorageMode = Rw> {
    pub all: StdDevPerBlockExtended<M>,
    pub _4y: StdDevPerBlockExtended<M>,
    pub _2y: StdDevPerBlockExtended<M>,
    pub _1y: StdDevPerBlockExtended<M>,
}

const VERSION: Version = Version::new(4);

impl RatioPerBlockStdDevBands {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        macro_rules! import_sd {
            ($suffix:expr, $period:expr, $days:expr) => {
                StdDevPerBlockExtended::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    $period,
                    $days,
                    v,
                    indexes,
                )?
            };
        }

        Ok(Self {
            all: import_sd!("ratio", "", usize::MAX),
            _1y: import_sd!("ratio", "1y", 365),
            _2y: import_sd!("ratio", "2y", 2 * 365),
            _4y: import_sd!("ratio", "4y", 4 * 365),
        })
    }

    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        ratio_source: &impl ReadableVec<Height, StoredF32>,
        metric_price: &impl ReadableVec<Height, Cents>,
    ) -> Result<()> {
        for sd in [
            &mut self.all,
            &mut self._4y,
            &mut self._2y,
            &mut self._1y,
        ] {
            sd.compute_all(blocks, starting_indexes, exit, ratio_source)?;
            sd.compute_cents_bands(starting_indexes, metric_price, exit)?;
        }

        Ok(())
    }
}
