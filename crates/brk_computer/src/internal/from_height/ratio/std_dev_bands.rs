use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredF32, Version};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, indexes, internal::ComputedFromHeightStdDevExtended};

#[derive(Traversable)]
pub struct ComputedFromHeightRatioStdDevBands<M: StorageMode = Rw> {
    pub ratio_sd: ComputedFromHeightStdDevExtended<M>,
    pub ratio_sd_4y: ComputedFromHeightStdDevExtended<M>,
    pub ratio_sd_2y: ComputedFromHeightStdDevExtended<M>,
    pub ratio_sd_1y: ComputedFromHeightStdDevExtended<M>,
}

const VERSION: Version = Version::new(4);

impl ComputedFromHeightRatioStdDevBands {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        macro_rules! import_sd {
            ($suffix:expr, $period:expr, $days:expr) => {
                ComputedFromHeightStdDevExtended::forced_import(
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
            ratio_sd: import_sd!("ratio", "", usize::MAX),
            ratio_sd_1y: import_sd!("ratio", "1y", 365),
            ratio_sd_2y: import_sd!("ratio", "2y", 2 * 365),
            ratio_sd_4y: import_sd!("ratio", "4y", 4 * 365),
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
            &mut self.ratio_sd,
            &mut self.ratio_sd_4y,
            &mut self.ratio_sd_2y,
            &mut self.ratio_sd_1y,
        ] {
            sd.compute_all(blocks, starting_indexes, exit, ratio_source)?;
            sd.compute_cents_bands(starting_indexes, metric_price, exit)?;
        }

        Ok(())
    }
}
