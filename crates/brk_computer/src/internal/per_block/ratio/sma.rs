use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints32, Height, Indexes, StoredF32, Version};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, indexes};

use super::RatioPerBlock;

#[derive(Traversable)]
pub struct RatioSma<M: StorageMode = Rw> {
    pub all: RatioPerBlock<BasisPoints32, M>,
    pub _1w: RatioPerBlock<BasisPoints32, M>,
    pub _1m: RatioPerBlock<BasisPoints32, M>,
    pub _1y: RatioPerBlock<BasisPoints32, M>,
    pub _2y: RatioPerBlock<BasisPoints32, M>,
    pub _4y: RatioPerBlock<BasisPoints32, M>,
}

const VERSION: Version = Version::new(4);

impl RatioSma {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        macro_rules! import {
            ($suffix:expr) => {
                RatioPerBlock::forced_import_raw(
                    db,
                    &format!("{name}_ratio_sma_{}", $suffix),
                    v,
                    indexes,
                )?
            };
        }

        Ok(Self {
            all: import!("all"),
            _1w: import!("1w"),
            _1m: import!("1m"),
            _1y: import!("1y"),
            _2y: import!("2y"),
            _4y: import!("4y"),
        })
    }

    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        ratio_source: &impl ReadableVec<Height, StoredF32>,
    ) -> Result<()> {
        // Expanding SMA (all history)
        self.all.bps.height.compute_sma_(
            starting_indexes.height,
            ratio_source,
            usize::MAX,
            exit,
            None,
        )?;

        // Rolling SMAs
        for (sma, lookback) in [
            (&mut self._1w, &blocks.lookback.height_1w_ago),
            (&mut self._1m, &blocks.lookback.height_1m_ago),
            (&mut self._1y, &blocks.lookback.height_1y_ago),
            (&mut self._2y, &blocks.lookback.height_2y_ago),
            (&mut self._4y, &blocks.lookback.height_4y_ago),
        ] {
            sma.bps.height.compute_rolling_average(
                starting_indexes.height,
                lookback,
                ratio_source,
                exit,
            )?;
        }

        Ok(())
    }
}
