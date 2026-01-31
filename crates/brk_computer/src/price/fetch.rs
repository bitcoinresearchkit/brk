use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{DateIndex, Height, OHLCCentsUnsigned};
use vecdb::{
    AnyStoredVec, AnyVec, Exit, GenericStoredVec, IterableVec, TypedVecIterator, VecIndex,
};

use crate::{ComputeIndexes, indexes, utils::OptionExt};

use super::Vecs;

impl Vecs {
    pub fn fetch(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let Some(fetcher) = self.fetcher.as_mut() else {
            return Ok(());
        };

        // Validate computed versions against dependencies
        let height_dep_version = indexer.vecs.blocks.timestamp.version();
        self.cents
            .ohlc
            .height
            .validate_computed_version_or_reset(height_dep_version)?;

        let dateindex_dep_version = indexes.dateindex.date.version();
        self.cents
            .ohlc
            .dateindex
            .validate_computed_version_or_reset(dateindex_dep_version)?;

        let height_to_timestamp = &indexer.vecs.blocks.timestamp;
        let index = starting_indexes
            .height
            .min(Height::from(self.cents.ohlc.height.len()));
        let mut prev_timestamp = index
            .decremented()
            .map(|prev_i| height_to_timestamp.iter().unwrap().get_unwrap(prev_i));
        height_to_timestamp
            .iter()?
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(i, v)| -> Result<()> {
                self.cents.ohlc.height.truncate_push_at(
                    i,
                    fetcher.get_height(i.into(), v, prev_timestamp).unwrap(),
                )?;
                prev_timestamp = Some(v);
                Ok(())
            })?;
        {
            let _lock = exit.lock();
            self.cents.ohlc.height.write()?;
        }

        let index = starting_indexes
            .dateindex
            .min(DateIndex::from(self.cents.ohlc.dateindex.len()));
        let mut prev = Some(index.decremented().map_or(OHLCCentsUnsigned::default(), |prev_i| {
            self.cents.ohlc.dateindex.iter().unwrap().get_unwrap(prev_i)
        }));
        indexes
            .dateindex
            .date
            .iter()
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(i, d)| -> Result<()> {
                let ohlc = if i.to_usize() + 100 >= self.cents.ohlc.dateindex.len()
                    && let Ok(mut ohlc) = fetcher.get_date(d)
                {
                    let prev_open = *prev.u().close;
                    *ohlc.open = prev_open;
                    *ohlc.high = (*ohlc.high).max(prev_open);
                    *ohlc.low = (*ohlc.low).min(prev_open);
                    ohlc
                } else {
                    prev.clone().unwrap()
                };

                prev.replace(ohlc.clone());

                self.cents.ohlc.dateindex.truncate_push_at(i, ohlc)?;

                Ok(())
            })?;
        {
            let _lock = exit.lock();
            self.cents.ohlc.dateindex.write()?;
        }

        Ok(())
    }
}
