use std::collections::VecDeque;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{
    Cents, Close, Date, DateIndex, Height, High, Low, OHLCCents, Open, OutputType, Sats, StoredU32,
    StoredU64, TxIndex,
};
use tracing::info;
use vecdb::{
    AnyStoredVec, AnyVec, Exit, GenericStoredVec, IterableVec, TypedVecIterator, VecIndex,
    VecIterator,
};

use super::{
    Vecs,
    config::OracleConfig,
    histogram::{Histogram, TOTAL_BINS},
    stencil::{find_best_price, is_round_sats, refine_price},
};
use crate::{ComputeIndexes, indexes};

impl Vecs {
    /// Compute oracle prices from on-chain data
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Validate versions
        self.price_cents
            .validate_computed_version_or_reset(indexer.vecs.outputs.value.version())?;
        self.ohlc_cents
            .validate_computed_version_or_reset(indexes.dateindex.date.version())?;

        let last_height = Height::from(indexer.vecs.blocks.timestamp.len());
        let start_height = starting_indexes
            .height
            .min(Height::from(self.price_cents.len()));

        if start_height >= last_height {
            return Ok(());
        }

        // Create buffered iterators ONCE (16KB buffered reads, reused across blocks)
        let mut height_to_first_txindex_iter = indexer.vecs.transactions.first_txindex.into_iter();
        let mut txindex_to_first_txinindex_iter =
            indexer.vecs.transactions.first_txinindex.into_iter();
        let mut txindex_to_first_txoutindex_iter =
            indexer.vecs.transactions.first_txoutindex.into_iter();
        let mut txindex_to_base_size_iter = indexer.vecs.transactions.base_size.into_iter();
        let mut txindex_to_total_size_iter = indexer.vecs.transactions.total_size.into_iter();
        let mut txoutindex_to_value_iter = indexer.vecs.outputs.value.into_iter();
        let mut txoutindex_to_outputtype_iter = indexer.vecs.outputs.outputtype.into_iter();
        let mut txinindex_to_outpoint_iter = indexer.vecs.inputs.outpoint.into_iter();
        let mut height_to_dateindex_iter = indexes.height.dateindex.iter();
        let mut txindex_to_input_count_iter = indexes.txindex.input_count.iter();
        let mut txindex_to_output_count_iter = indexes.txindex.output_count.iter();
        let mut dateindex_to_first_height_iter = indexes.dateindex.first_height.iter();

        // Sliding window state - use sparse storage for per-block histograms
        // Each block has ~40 outputs → ~40 sparse entries vs 1600 bins
        let mut window_sparse: VecDeque<Vec<(u16, f64)>> = VecDeque::with_capacity(2016);
        let mut window_tx_counts: VecDeque<usize> = VecDeque::with_capacity(2016);
        let mut aggregated_histogram = Histogram::new();
        let mut total_qualifying_txs: usize = 0;
        let mut scratch_histogram = Histogram::new();

        // Incremental by-bin index for refine_price (avoids O(80k) rebuild per block)
        // Stores (bin, sats) pairs per block for removal tracking
        let mut window_by_bin_entries: VecDeque<Vec<(u16, Sats)>> = VecDeque::with_capacity(2016);
        // Aggregated view: non-round sats grouped by histogram bin
        let mut aggregated_by_bin: [Vec<Sats>; TOTAL_BINS] = std::array::from_fn(|_| Vec::new());

        // Track current date for same-day check
        let mut current_dateindex = DateIndex::from(0usize);
        let mut current_date_first_txindex = TxIndex::from(0usize);

        // Previous price for fallback (default ~$100,000)
        let mut prev_price = if start_height > Height::ZERO {
            self.price_cents
                .iter()?
                .get(start_height.decremented().unwrap())
                .unwrap_or(Cents::from(10_000_000i64))
        } else {
            Cents::from(10_000_000i64)
        };

        // Progress tracking
        let total_blocks = last_height.to_usize() - start_height.to_usize();
        let mut last_progress = 0u8;
        let total_txs = indexer.vecs.transactions.height.len();

        // Sparse entries for current block (reused buffer)
        let mut block_sparse: Vec<(u16, f64)> = Vec::with_capacity(80);

        // Cached config (only changes at year boundaries)
        let mut cached_year = 0u16;
        let mut config = OracleConfig::for_year(2009);
        let mut cached_slide_range = config.slide_range();

        // Process each block
        for height in start_height.to_usize()..last_height.to_usize() {
            let height = Height::from(height);

            // Log progress every 1%
            let progress =
                ((height.to_usize() - start_height.to_usize()) * 100 / total_blocks.max(1)) as u8;
            if progress > last_progress {
                last_progress = progress;
                info!("Oracle price computation: {}%", progress);
            }

            // Get transaction range for this block
            let first_txindex = height_to_first_txindex_iter.get_at_unwrap(height.to_usize());
            let next_first_txindex = height_to_first_txindex_iter
                .get_at(height.to_usize() + 1)
                .unwrap_or(TxIndex::from(total_txs));

            let block_dateindex = height_to_dateindex_iter.get_unwrap(height);

            // Update current date's first txindex on date transition
            if block_dateindex != current_dateindex {
                current_dateindex = block_dateindex;
                if let Some(first_height_of_date) =
                    dateindex_to_first_height_iter.get(block_dateindex)
                {
                    current_date_first_txindex = height_to_first_txindex_iter
                        .get_at(first_height_of_date.to_usize())
                        .unwrap_or(first_txindex);
                }

                // Update config if year changed
                let year = Date::from(block_dateindex).year();
                if year != cached_year {
                    cached_year = year;
                    config = OracleConfig::for_year(year);
                    cached_slide_range = config.slide_range();
                }
            }

            let tx_start = first_txindex.to_usize() + 1; // skip coinbase
            let tx_end = next_first_txindex.to_usize();

            // Clear per-block state
            block_sparse.clear();
            let mut block_by_bin: Vec<(u16, Sats)> = Vec::with_capacity(40); // (bin, sats) for non-round outputs
            let mut block_tx_count = 0usize;

            // Sequential iteration with buffered reads (cache-friendly)
            for txindex in tx_start..tx_end {
                // Check output_count FIRST - ~95% of txs don't have exactly 2 outputs
                // This avoids fetching input_count for most transactions
                let output_count: StoredU64 =
                    txindex_to_output_count_iter.get_unwrap(TxIndex::from(txindex));
                if *output_count != 2 {
                    continue;
                }

                let input_count: StoredU64 =
                    txindex_to_input_count_iter.get_unwrap(TxIndex::from(txindex));
                if *input_count > 5 || *input_count == 0 {
                    continue;
                }

                let first_txoutindex = txindex_to_first_txoutindex_iter.get_at_unwrap(txindex);
                let first_txinindex = txindex_to_first_txinindex_iter.get_at_unwrap(txindex);

                // Check outputs: no OP_RETURN, collect values
                let mut has_opreturn = false;
                let mut values: [Sats; 2] = [Sats::ZERO; 2];
                for i in 0..2usize {
                    let txoutindex = first_txoutindex.to_usize() + i;
                    let outputtype = txoutindex_to_outputtype_iter.get_at_unwrap(txoutindex);
                    if outputtype == OutputType::OpReturn {
                        has_opreturn = true;
                        break;
                    }
                    values[i] = txoutindex_to_value_iter.get_at_unwrap(txoutindex);
                }
                if has_opreturn {
                    continue;
                }

                // Check witness size (SegWit era only, activated Aug 2017)
                // Pre-SegWit transactions have no witness data
                if cached_year >= 2017 {
                    let base_size: StoredU32 = txindex_to_base_size_iter.get_at_unwrap(txindex);
                    let total_size: StoredU32 = txindex_to_total_size_iter.get_at_unwrap(txindex);
                    if *total_size - *base_size > 500 {
                        continue;
                    }
                }

                // Check inputs: no same-day spend
                let mut disqualified = false;
                for i in 0..*input_count as usize {
                    let txinindex = first_txinindex.to_usize() + i;
                    let outpoint = txinindex_to_outpoint_iter.get_at_unwrap(txinindex);
                    if !outpoint.is_coinbase() && outpoint.txindex() >= current_date_first_txindex {
                        disqualified = true;
                        break;
                    }
                }

                if disqualified {
                    continue;
                }

                // Transaction qualifies!
                block_tx_count += 1;
                for sats in values {
                    if let Some(bin) = Histogram::sats_to_bin(sats) {
                        block_sparse.push((bin as u16, 1.0));
                        // Track non-round outputs for refine_price
                        if !is_round_sats(sats) {
                            block_by_bin.push((bin as u16, sats));
                        }
                    }
                }
            }

            // Update sliding window using sparse operations
            let window_size = config.blocks_per_window as usize;
            while window_sparse.len() >= window_size {
                if let Some(old_sparse) = window_sparse.pop_front() {
                    aggregated_histogram.subtract_sparse(&old_sparse);
                }
                if let Some(old_count) = window_tx_counts.pop_front() {
                    total_qualifying_txs -= old_count;
                }
                // Remove old by-bin entries from aggregated view
                if let Some(old_by_bin) = window_by_bin_entries.pop_front() {
                    for (bin, sats) in old_by_bin {
                        let vec = &mut aggregated_by_bin[bin as usize];
                        if let Some(pos) = vec.iter().position(|&s| s == sats) {
                            vec.swap_remove(pos);
                        }
                    }
                }
            }

            aggregated_histogram.add_sparse(&block_sparse);
            total_qualifying_txs += block_tx_count;
            window_sparse.push_back(block_sparse.clone());
            window_tx_counts.push_back(block_tx_count);

            // Add new by-bin entries to aggregated view
            for &(bin, sats) in &block_by_bin {
                aggregated_by_bin[bin as usize].push(sats);
            }
            window_by_bin_entries.push_back(block_by_bin);

            // Compute price
            let price_cents = if total_qualifying_txs >= config.min_tx_count as usize {
                scratch_histogram.copy_from(&aggregated_histogram);
                scratch_histogram.smooth_round_btc();
                scratch_histogram.normalize();

                let (min_slide, max_slide) = cached_slide_range;

                if let Some(rough_price) = find_best_price(&scratch_histogram, min_slide, max_slide)
                {
                    refine_price(&aggregated_by_bin, rough_price)
                } else {
                    prev_price
                }
            } else {
                prev_price
            };

            prev_price = price_cents;

            self.price_cents
                .truncate_push_at(height.to_usize(), price_cents)?;
        }

        // Write height prices
        {
            let _lock = exit.lock();
            self.price_cents.write()?;
        }

        info!("Oracle price computation: 100%");

        // Aggregate to daily OHLC
        self.compute_daily_ohlc(indexes, starting_indexes, exit)?;

        Ok(())
    }

    /// Aggregate per-block prices to daily OHLC
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

        for dateindex in start_dateindex.to_usize()..last_dateindex.to_usize() {
            let dateindex = DateIndex::from(dateindex);
            let first_height = dateindex_to_first_height_iter.get_unwrap(dateindex);
            let count = height_count_iter.get_unwrap(dateindex);

            if *count == 0 || first_height >= last_height {
                continue;
            }

            let count = *count as usize;

            // Compute OHLC from block prices
            let mut open = None;
            let mut high = Cents::from(0i64);
            let mut low = Cents::from(i64::MAX);
            let mut close = Cents::from(0i64);
            let mut tx_count = 0u32;

            for i in 0..count {
                let height = first_height + Height::from(i);
                if height >= last_height {
                    break;
                }

                if let Some(price) = height_to_price_iter.get(height) {
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
                    tx_count += 1;
                }
            }

            let ohlc = if let Some(open_price) = open {
                OHLCCents {
                    open: Open::new(open_price),
                    high: High::new(high),
                    low: Low::new(low),
                    close: Close::new(close),
                }
            } else {
                // No prices for this day, use previous
                if dateindex > DateIndex::from(0usize) {
                    self.ohlc_cents
                        .iter()?
                        .get(dateindex.decremented().unwrap())
                        .unwrap_or_default()
                } else {
                    OHLCCents::default()
                }
            };

            self.ohlc_cents
                .truncate_push_at(dateindex.to_usize(), ohlc)?;
            self.tx_count
                .truncate_push_at(dateindex.to_usize(), StoredU32::from(tx_count))?;
        }

        // Write daily data
        {
            let _lock = exit.lock();
            self.ohlc_cents.write()?;
            self.tx_count.write()?;
        }

        Ok(())
    }
}
