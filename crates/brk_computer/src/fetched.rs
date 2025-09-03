use std::path::Path;

use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_structs::{DateIndex, Height, OHLCCents, Version};
use vecdb::{
    AnyCollectableVec, AnyIterableVec, AnyStoredVec, AnyVec, Database, Exit, GenericStoredVec,
    RawVec, StoredIndex, VecIterator,
};

use super::{Indexes, indexes};

#[derive(Clone)]
pub struct Vecs {
    db: Database,
    fetcher: Fetcher,

    pub dateindex_to_price_ohlc_in_cents: RawVec<DateIndex, OHLCCents>,
    pub height_to_price_ohlc_in_cents: RawVec<Height, OHLCCents>,
}

impl Vecs {
    pub fn forced_import(parent: &Path, fetcher: Fetcher, version: Version) -> Result<Self> {
        let db = Database::open(&parent.join("fetched"))?;

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
            this.vecs()
                .into_iter()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

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
        self.db.flush_then_punch()?;
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
        height_to_timestamp
            .iter_at(index)
            .try_for_each(|(i, v)| -> Result<()> {
                let v = v.into_owned();
                self.height_to_price_ohlc_in_cents.forced_push_at(
                    i,
                    self.fetcher
                        .get_height(
                            i,
                            v,
                            i.decremented().map(|prev_i| {
                                height_to_timestamp.into_iter().unwrap_get_inner(prev_i)
                            }),
                        )
                        .unwrap(),
                    exit,
                )?;
                Ok(())
            })?;
        self.height_to_price_ohlc_in_cents.safe_flush(exit)?;

        let index = starting_indexes
            .dateindex
            .min(DateIndex::from(self.dateindex_to_price_ohlc_in_cents.len()));
        let mut prev = None;
        indexes
            .dateindex_to_date
            .iter_at(index)
            .try_for_each(|(i, v)| -> Result<()> {
                let d = v.into_owned();
                if prev.is_none() {
                    let i = i.unwrap_to_usize();
                    prev.replace(if i > 0 {
                        self.dateindex_to_price_ohlc_in_cents
                            .into_iter()
                            .unwrap_get_inner_(i - 1)
                    } else {
                        OHLCCents::default()
                    });
                }

                let ohlc = if i.unwrap_to_usize() + 100
                    >= self.dateindex_to_price_ohlc_in_cents.len()
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
                    .forced_push_at(i, ohlc, exit)?;

                Ok(())
            })?;
        self.dateindex_to_price_ohlc_in_cents.safe_flush(exit)?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        vec![
            &self.dateindex_to_price_ohlc_in_cents as &dyn AnyCollectableVec,
            &self.height_to_price_ohlc_in_cents,
        ]
    }
}
