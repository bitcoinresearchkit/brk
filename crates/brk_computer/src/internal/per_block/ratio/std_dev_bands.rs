use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredF32, Version};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, indexes, internal::StdDevPerBlockExtended};

use super::RatioSma;

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
            ($period:expr, $days:expr) => {
                StdDevPerBlockExtended::forced_import(
                    db,
                    name,
                    $period,
                    $days,
                    v,
                    indexes,
                )?
            };
        }

        Ok(Self {
            all: import_sd!("", usize::MAX),
            _1y: import_sd!("1y", 365),
            _2y: import_sd!("2y", 2 * 365),
            _4y: import_sd!("4y", 4 * 365),
        })
    }

    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        ratio_source: &impl ReadableVec<Height, StoredF32>,
        series_price: &impl ReadableVec<Height, Cents>,
        sma: &RatioSma,
    ) -> Result<()> {
        for (sd, sma_ratio) in [
            (&mut self.all, &sma.all.ratio.height),
            (&mut self._4y, &sma._4y.ratio.height),
            (&mut self._2y, &sma._2y.ratio.height),
            (&mut self._1y, &sma._1y.ratio.height),
        ] {
            sd.compute_all(blocks, starting_indexes, exit, ratio_source, sma_ratio)?;
            sd.compute_cents_bands(starting_indexes, series_price, sma_ratio, exit)?;
        }

        Ok(())
    }
}
