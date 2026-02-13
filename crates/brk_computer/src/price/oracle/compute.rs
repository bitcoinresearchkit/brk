use std::ops::Range;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_oracle::{Config, NUM_BINS, Oracle, START_HEIGHT, bin_to_cents, cents_to_bin};
use brk_types::{
    CentsUnsigned, Close, DateIndex, Height, High, Low, OHLCCentsUnsigned, OHLCDollars, Open,
    OutputType, Sats, TxIndex, TxOutIndex,
};
use tracing::info;
use vecdb::{
    AnyStoredVec, AnyVec, Exit, GenericStoredVec, IterableVec, TypedVecIterator, VecIndex,
    VecIterator,
};

use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_prices(indexer, starting_indexes, exit)?;
        self.compute_daily_ohlc(indexes, starting_indexes, exit)?;
        self.compute_split_and_ohlc(starting_indexes, exit)?;
        Ok(())
    }

    fn compute_split_and_ohlc(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Destructure to allow simultaneous borrows of different fields
        let Self {
            price_cents,
            ohlc_cents,
            split,
            ohlc,
            ohlc_dollars,
        } = self;

        // Open: first-value aggregation
        split.open.height.compute_transform(
            starting_indexes.height,
            &*price_cents,
            |(h, price, ..)| (h, Open::new(price)),
            exit,
        )?;
        split.open.compute_rest(starting_indexes, exit, |v| {
            v.compute_transform(
                starting_indexes.dateindex,
                &*ohlc_cents,
                |(di, ohlc_val, ..)| (di, ohlc_val.open),
                exit,
            )?;
            Ok(())
        })?;

        // High: max-value aggregation
        split.high.height.compute_transform(
            starting_indexes.height,
            &*price_cents,
            |(h, price, ..)| (h, High::new(price)),
            exit,
        )?;
        split.high.compute_rest(starting_indexes, exit, |v| {
            v.compute_transform(
                starting_indexes.dateindex,
                &*ohlc_cents,
                |(di, ohlc_val, ..)| (di, ohlc_val.high),
                exit,
            )?;
            Ok(())
        })?;

        // Low: min-value aggregation
        split.low.height.compute_transform(
            starting_indexes.height,
            &*price_cents,
            |(h, price, ..)| (h, Low::new(price)),
            exit,
        )?;
        split.low.compute_rest(starting_indexes, exit, |v| {
            v.compute_transform(
                starting_indexes.dateindex,
                &*ohlc_cents,
                |(di, ohlc_val, ..)| (di, ohlc_val.low),
                exit,
            )?;
            Ok(())
        })?;

        // Close: last-value aggregation
        split.close.height.compute_transform(
            starting_indexes.height,
            &*price_cents,
            |(h, price, ..)| (h, Close::new(price)),
            exit,
        )?;
        split.close.compute_rest(starting_indexes, exit, |v| {
            v.compute_transform(
                starting_indexes.dateindex,
                &*ohlc_cents,
                |(di, ohlc_val, ..)| (di, ohlc_val.close),
                exit,
            )?;
            Ok(())
        })?;

        // Period OHLC aggregates - time based
        ohlc.dateindex.compute_transform4(
            starting_indexes.dateindex,
            &split.open.dateindex,
            &split.high.dateindex,
            &split.low.dateindex,
            &split.close.dateindex,
            |(i, open, high, low, close, _)| {
                (i, OHLCCentsUnsigned { open, high, low, close })
            },
            exit,
        )?;

        ohlc.week.compute_transform4(
            starting_indexes.weekindex,
            &*split.open.weekindex,
            &*split.high.weekindex,
            &*split.low.weekindex,
            &*split.close.weekindex,
            |(i, open, high, low, close, _)| {
                (i, OHLCCentsUnsigned { open, high, low, close })
            },
            exit,
        )?;

        ohlc.month.compute_transform4(
            starting_indexes.monthindex,
            &*split.open.monthindex,
            &*split.high.monthindex,
            &*split.low.monthindex,
            &*split.close.monthindex,
            |(i, open, high, low, close, _)| {
                (i, OHLCCentsUnsigned { open, high, low, close })
            },
            exit,
        )?;

        ohlc.quarter.compute_transform4(
            starting_indexes.quarterindex,
            &*split.open.quarterindex,
            &*split.high.quarterindex,
            &*split.low.quarterindex,
            &*split.close.quarterindex,
            |(i, open, high, low, close, _)| {
                (i, OHLCCentsUnsigned { open, high, low, close })
            },
            exit,
        )?;

        ohlc.semester.compute_transform4(
            starting_indexes.semesterindex,
            &*split.open.semesterindex,
            &*split.high.semesterindex,
            &*split.low.semesterindex,
            &*split.close.semesterindex,
            |(i, open, high, low, close, _)| {
                (i, OHLCCentsUnsigned { open, high, low, close })
            },
            exit,
        )?;

        ohlc.year.compute_transform4(
            starting_indexes.yearindex,
            &*split.open.yearindex,
            &*split.high.yearindex,
            &*split.low.yearindex,
            &*split.close.yearindex,
            |(i, open, high, low, close, _)| {
                (i, OHLCCentsUnsigned { open, high, low, close })
            },
            exit,
        )?;

        ohlc.decade.compute_transform4(
            starting_indexes.decadeindex,
            &*split.open.decadeindex,
            &*split.high.decadeindex,
            &*split.low.decadeindex,
            &*split.close.decadeindex,
            |(i, open, high, low, close, _)| {
                (i, OHLCCentsUnsigned { open, high, low, close })
            },
            exit,
        )?;

        // Period OHLC aggregates - chain based
        ohlc.height.compute_transform4(
            starting_indexes.height,
            &split.open.height,
            &split.high.height,
            &split.low.height,
            &split.close.height,
            |(i, open, high, low, close, _)| {
                (i, OHLCCentsUnsigned { open, high, low, close })
            },
            exit,
        )?;

        ohlc.difficultyepoch.compute_transform4(
            starting_indexes.difficultyepoch,
            &*split.open.difficultyepoch,
            &*split.high.difficultyepoch,
            &*split.low.difficultyepoch,
            &*split.close.difficultyepoch,
            |(i, open, high, low, close, _)| {
                (i, OHLCCentsUnsigned { open, high, low, close })
            },
            exit,
        )?;

        // OHLC dollars - transform cents to dollars at every period level
        macro_rules! cents_to_dollars {
            ($field:ident, $idx:expr) => {
                ohlc_dollars.$field.compute_transform(
                    $idx,
                    &ohlc.$field,
                    |(i, c, ..)| (i, OHLCDollars::from(c)),
                    exit,
                )?;
            };
        }

        cents_to_dollars!(dateindex, starting_indexes.dateindex);
        cents_to_dollars!(week, starting_indexes.weekindex);
        cents_to_dollars!(month, starting_indexes.monthindex);
        cents_to_dollars!(quarter, starting_indexes.quarterindex);
        cents_to_dollars!(semester, starting_indexes.semesterindex);
        cents_to_dollars!(year, starting_indexes.yearindex);
        cents_to_dollars!(decade, starting_indexes.decadeindex);
        cents_to_dollars!(height, starting_indexes.height);
        cents_to_dollars!(difficultyepoch, starting_indexes.difficultyepoch);

        Ok(())
    }

    fn compute_prices(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let source_version =
            indexer.vecs.outputs.value.version() + indexer.vecs.outputs.outputtype.version();
        self.price_cents
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = indexer.vecs.blocks.timestamp.len();

        if total_heights <= START_HEIGHT {
            return Ok(());
        }

        // Reorg: truncate to starting_indexes
        let truncate_to = self
            .price_cents
            .len()
            .min(starting_indexes.height.to_usize());
        self.price_cents.truncate_if_needed_at(truncate_to)?;

        if self.price_cents.len() < START_HEIGHT {
            for line in brk_oracle::PRICES.lines().skip(self.price_cents.len()) {
                if self.price_cents.len() >= START_HEIGHT {
                    break;
                }
                let dollars: f64 = line.parse().unwrap_or(0.0);
                let cents = (dollars * 100.0).round() as u64;
                self.price_cents.push(CentsUnsigned::new(cents));
            }
        }

        if self.price_cents.len() >= total_heights {
            return Ok(());
        }

        let config = Config::default();
        let committed = self.price_cents.len();
        let prev_cents = self.price_cents
            .iter()?
            .get(Height::from(committed - 1))
            .unwrap();
        let seed_bin = cents_to_bin(prev_cents.inner() as f64);
        let warmup = config.window_size.min(committed - START_HEIGHT);
        let mut oracle = Oracle::from_checkpoint(seed_bin, config, |o| {
            Self::feed_blocks(o, indexer, (committed - warmup)..committed);
        });

        let num_new = total_heights - committed;
        info!(
            "Computing oracle prices: {} to {} ({warmup} warmup)",
            committed, total_heights
        );

        let ref_bins = Self::feed_blocks(&mut oracle, indexer, committed..total_heights);

        for (i, ref_bin) in ref_bins.into_iter().enumerate() {
            self.price_cents.push(CentsUnsigned::new(bin_to_cents(ref_bin)));

            let progress = ((i + 1) * 100 / num_new) as u8;
            if i > 0 && progress > ((i * 100 / num_new) as u8) {
                info!("Oracle price computation: {}%", progress);
            }
        }

        {
            let _lock = exit.lock();
            self.price_cents.write()?;
        }

        info!(
            "Oracle prices complete: {} committed",
            self.price_cents.len()
        );

        Ok(())
    }

    /// Returns an Oracle seeded from the last committed price, with the last
    /// window_size blocks already processed. Ready for additional blocks (e.g. mempool).
    pub fn live_oracle(&self, indexer: &Indexer) -> Result<Oracle> {
        let config = Config::default();
        let height = indexer.vecs.blocks.timestamp.len();
        let last_cents = self.price_cents
            .iter()?
            .get(Height::from(self.price_cents.len() - 1))
            .unwrap();
        let seed_bin = cents_to_bin(last_cents.inner() as f64);
        let window_size = config.window_size;
        let oracle = Oracle::from_checkpoint(seed_bin, config, |o| {
            Self::feed_blocks(o, indexer, height.saturating_sub(window_size)..height);
        });

        Ok(oracle)
    }

    /// Feed a range of blocks from the indexer into an Oracle (skipping coinbase),
    /// returning per-block ref_bin values.
    fn feed_blocks(oracle: &mut Oracle, indexer: &Indexer, range: Range<usize>) -> Vec<f64> {
        let total_txs = indexer.vecs.transactions.height.len();
        let total_outputs = indexer.vecs.outputs.value.len();

        let mut first_txindex_iter = indexer.vecs.transactions.first_txindex.into_iter();
        let mut first_txoutindex_iter = indexer.vecs.transactions.first_txoutindex.into_iter();
        let mut out_first_iter = indexer.vecs.outputs.first_txoutindex.into_iter();
        let mut value_iter = indexer.vecs.outputs.value.into_iter();
        let mut outputtype_iter = indexer.vecs.outputs.outputtype.into_iter();

        let mut ref_bins = Vec::with_capacity(range.len());

        for h in range {
            let first_txindex: TxIndex = first_txindex_iter.get_at_unwrap(h);
            let next_first_txindex = first_txindex_iter
                .get_at(h + 1)
                .unwrap_or(TxIndex::from(total_txs));

            let out_start = if first_txindex.to_usize() + 1 < next_first_txindex.to_usize() {
                first_txoutindex_iter
                    .get_at_unwrap(first_txindex.to_usize() + 1)
                    .to_usize()
            } else {
                out_first_iter
                    .get_at(h + 1)
                    .unwrap_or(TxOutIndex::from(total_outputs))
                    .to_usize()
            };
            let out_end = out_first_iter
                .get_at(h + 1)
                .unwrap_or(TxOutIndex::from(total_outputs))
                .to_usize();

            let mut hist = [0u32; NUM_BINS];
            for i in out_start..out_end {
                let sats: Sats = value_iter.get_at_unwrap(i);
                let output_type: OutputType = outputtype_iter.get_at_unwrap(i);
                if let Some(bin) = oracle.output_to_bin(sats, output_type) {
                    hist[bin] += 1;
                }
            }

            ref_bins.push(oracle.process_histogram(&hist));
        }

        ref_bins
    }

    fn compute_daily_ohlc(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let last_dateindex = DateIndex::from(indexes.dateindex.date.len());
        let start_dateindex = starting_indexes
            .dateindex
            .min(DateIndex::from(self.ohlc_cents.len()));

        if start_dateindex >= last_dateindex {
            return Ok(());
        }

        let last_height = Height::from(self.price_cents.len());
        let mut height_to_price_iter = self.price_cents.iter()?;
        let mut dateindex_to_first_height_iter = indexes.dateindex.first_height.iter();
        let mut height_count_iter = indexes.dateindex.height_count.iter();

        for dateindex_usize in start_dateindex.to_usize()..last_dateindex.to_usize() {
            let dateindex = DateIndex::from(dateindex_usize);
            let first_height = dateindex_to_first_height_iter.get_unwrap(dateindex);
            let count = height_count_iter.get_unwrap(dateindex);

            if *count == 0 || first_height >= last_height {
                self.ohlc_cents
                    .truncate_push(dateindex, self.previous_ohlc(dateindex)?)?;
                continue;
            }

            let count = *count as usize;
            let mut open = None;
            let mut high = CentsUnsigned::ZERO;
            let mut low = CentsUnsigned::MAX;
            let mut close = CentsUnsigned::ZERO;

            for i in 0..count {
                let height = first_height + Height::from(i);
                if height >= last_height {
                    break;
                }

                if let Some(price) = height_to_price_iter.get(height) {
                    if price == CentsUnsigned::ZERO {
                        continue;
                    }
                    if open.is_none() {
                        open = Some(price);
                    }
                    if price > high {
                        high = price;
                    }
                    if price < low {
                        low = price;
                    }
                    close = price;
                }
            }

            let ohlc = if let Some(open_price) = open {
                OHLCCentsUnsigned {
                    open: Open::new(open_price),
                    high: High::new(high),
                    low: Low::new(low),
                    close: Close::new(close),
                }
            } else {
                self.previous_ohlc(dateindex)?
            };

            self.ohlc_cents.truncate_push(dateindex, ohlc)?;
        }

        {
            let _lock = exit.lock();
            self.ohlc_cents.write()?;
        }

        Ok(())
    }

    fn previous_ohlc(&self, dateindex: DateIndex) -> Result<OHLCCentsUnsigned> {
        Ok(if dateindex > DateIndex::from(0usize) {
            self.ohlc_cents
                .iter()?
                .get(dateindex.decremented().unwrap())
                .unwrap_or_default()
        } else {
            OHLCCentsUnsigned::default()
        })
    }
}
