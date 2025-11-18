use std::path::Path;

use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Height, OHLCCents, Version};
use vecdb::{
    AnyStoredVec, AnyVec, Database, Exit, GenericStoredVec, IterableVec, PAGE_SIZE, RawVec,
    TypedVecIterator, VecIndex,
};

use super::{Indexes, indexes};

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,
    fetcher: Fetcher,

    pub dateindex_to_price_ohlc_in_cents: RawVec<DateIndex, OHLCCents>,
    pub height_to_price_ohlc_in_cents: RawVec<Height, OHLCCents>,
}

impl Vecs {
    pub fn forced_import(parent: &Path, fetcher: Fetcher, version: Version) -> Result<Self> {
        let db = Database::open(&parent.join("fetched"))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let this = Self {
            fetcher,

            dateindex_to_price_ohlc_in_cents: RawVec::forced_import(
                &db,
                "price_ohlc_in_cents",
                version + Version::ZERO,
            )?,
            height_to_price_ohlc_in_cents: RawVec::forced_import(
                &db,
                "price_ohlc_in_cents",
                version + Version::ZERO,
            )?,

            db,
        };

        this.db.retain_regions(
            this.iter_any_writable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        this.db.compact()?;

        Ok(this)
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, indexes, starting_indexes, exit)?;
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let height_to_timestamp = &indexer.vecs.height_to_timestamp;
        let index = starting_indexes
            .height
            .min(Height::from(self.height_to_price_ohlc_in_cents.len()));
        let mut prev_timestamp = index
            .decremented()
            .map(|prev_i| height_to_timestamp.iter().unwrap().get_unwrap(prev_i));
        height_to_timestamp
            .iter()?
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(i, v)| -> Result<()> {
                self.height_to_price_ohlc_in_cents.truncate_push_at(
                    i,
                    self.fetcher
                        .get_height(i.into(), v, prev_timestamp)
                        .unwrap(),
                )?;
                prev_timestamp = Some(v);
                Ok(())
            })?;
        self.height_to_price_ohlc_in_cents.safe_flush(exit)?;

        let index = starting_indexes
            .dateindex
            .min(DateIndex::from(self.dateindex_to_price_ohlc_in_cents.len()));
        let mut prev = Some(index.decremented().map_or(OHLCCents::default(), |prev_i| {
            self.dateindex_to_price_ohlc_in_cents
                .iter()
                .unwrap()
                .get_unwrap(prev_i)
        }));
        indexes
            .dateindex_to_date
            .iter()
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(i, d)| -> Result<()> {
                let ohlc = if i.to_usize() + 100 >= self.dateindex_to_price_ohlc_in_cents.len()
                    && let Ok(mut ohlc) = self.fetcher.get_date(d)
                {
                    let prev_open = *prev.as_ref().unwrap().close;
                    *ohlc.open = prev_open;
                    *ohlc.high = (*ohlc.high).max(prev_open);
                    *ohlc.low = (*ohlc.low).min(prev_open);
                    ohlc
                } else {
                    prev.clone().unwrap()
                };

                prev.replace(ohlc.clone());

                self.dateindex_to_price_ohlc_in_cents
                    .truncate_push_at(i, ohlc)?;

                Ok(())
            })?;
        self.dateindex_to_price_ohlc_in_cents.safe_flush(exit)?;

        Ok(())
    }
}
