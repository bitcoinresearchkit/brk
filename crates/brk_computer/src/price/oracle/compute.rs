//! # Phase Oracle - On-chain Price Discovery
//!
//! Uses `frac(log10(sats))` to bin outputs into 100 bins per block.
//! The peak bin indicates the price decade (cyclical: $6.3, $63, $630, $6300 all map to same bin).
//! Monthly/yearly calibration anchors resolve the decade ambiguity.
//!
//! ## What Worked
//!
//! **Transaction filters (in `compute_pair_index`):**
//! - `output_count == 2` - payment + change pattern
//! - `input_count <= 5` - matches Python UTXOracle
//! - `witness_size <= 2500` bytes total
//! - No OP_RETURN outputs
//! - No P2TR (taproot) outputs - significantly cleaned up 2021+ data
//! - No P2MS, Empty, Unknown outputs - allowlist approach
//! - No same-day spends - inputs must spend outputs confirmed on earlier days
//! - No both-outputs-round - skip tx if both outputs are round BTC amounts (±0.1%)
//!
//! **Output filters (in `OracleBins::sats_to_bin`):**
//! - Per-output min/max: 1k sats to 100k BTC (matches Python's 1e-5 to 1e5 BTC)
//!
//! **Peak finding:**
//! - Skip bin 0 when finding peak - round BTC amounts (0.001, 0.01, 0.1, 1.0 BTC) cluster there
//!
//! **Anchors:**
//! - Monthly anchors 2010-2020 for better decade selection in volatile early years
//! - Yearly anchors 2021+ when prices are more stable
//!
//! ## What Didn't Work
//!
//! - **Skip all round bins (0, 10, 20, ..., 90) before 2020** - made results worse, not better
//! - **Top-N tie-breaking with prev_price** - caused drift
//! - **50% margin threshold for round bin avoidance** - still had issues
//! - **Transaction-level min sats filter** - Python filters per-output, not per-tx
//! - **Heel-based weighted peak** - noise also has spread, not just isolated spikes
//! - **Top-3 with non-round preference (50% threshold)** - inconsistent results
//! - **Neighbor-weighted scoring** - inconsistent, round BTC has correlated neighbors
//!
//! ## Not Yet Tried
//!
//! - **Tighter witness filter** - Python uses 500 bytes per input max, we use 2500 total
//! - **Multi-block smoothing** - aggregate histograms across N blocks
//! - **Minimum histogram count threshold** - fall back to anchor if total_count < N
//! - **Blend with UTXOracle port** - we compute both, could validate/combine
//!
//! ## Known Limitations
//!
//! - Pre-2017 data is noisy due to low transaction volume (weak signal)
//! - 2017 SegWit activation era has some spikes

use std::collections::VecDeque;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{
    Cents, Close, Date, DateIndex, Height, High, Low, OHLCCents, Open, OracleBins, OracleBinsV2,
    OutputType, PHASE_BINS, PairOutputIndex, Sats, StoredU32, StoredU64, TxIndex,
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
    phase_v2::{PhaseHistogramV2, find_best_phase, phase_range_from_anchor, phase_to_price},
    stencil::{find_best_price, is_round_sats, refine_price},
};
use crate::{ComputeIndexes, indexes, price::cents};

/// Flush interval for periodic writes during oracle computation.
const FLUSH_INTERVAL: usize = 10_000;

impl Vecs {
    /// Compute oracle prices from on-chain data
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price_cents: &cents::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Step 1: Compute pair output index (all 2-output transactions)
        self.compute_pair_index(indexer, indexes, starting_indexes, exit)?;

        // Step 2: Compute phase histograms (Layer 4)
        self.compute_phase_histograms(starting_indexes, exit)?;

        // Step 3: Compute phase oracle prices (Layer 5)
        self.compute_phase_prices(starting_indexes, exit)?;

        // Step 4: Compute phase daily average
        self.compute_phase_daily_average(indexes, starting_indexes, exit)?;

        // Step 6: Compute UTXOracle prices (Python port)
        self.compute_prices(indexer, indexes, starting_indexes, exit)?;

        // Step 7: Aggregate to daily OHLC
        self.compute_daily_ohlc(indexes, starting_indexes, exit)?;

        // Step 8: Compute Phase Oracle V2 (round USD template matching)
        // 8a: Per-block 200-bin histograms (uses ALL outputs, not pair-filtered)
        self.compute_phase_v2_histograms(indexer, indexes, starting_indexes, exit)?;

        // 8b: Per-block prices using cross-correlation with weekly anchors
        self.compute_phase_v2_prices(indexes, price_cents, starting_indexes, exit)?;

        // 8c: Per-block prices using direct peak finding (like V1)
        self.compute_phase_v2_peak_prices(indexes, price_cents, starting_indexes, exit)?;

        // 8d: Daily distributions from per-block prices
        self.compute_phase_v2_daily(indexes, starting_indexes, exit)?;

        // Step 9: Compute Phase Oracle V3 (BASE + uniqueVal filter)
        // 9a: Per-block histograms with uniqueVal filtering (only outputs with unique values in tx)
        self.compute_phase_v3_histograms(indexer, indexes, starting_indexes, exit)?;

        // 9b: Per-block prices using cross-correlation
        self.compute_phase_v3_prices(indexes, price_cents, starting_indexes, exit)?;

        // 9c: Per-block prices using direct peak finding (like V1)
        self.compute_phase_v3_peak_prices(indexes, price_cents, starting_indexes, exit)?;

        // 9d: Daily distributions from per-block prices
        self.compute_phase_v3_daily(indexes, starting_indexes, exit)?;

        Ok(())
    }

    /// Compute the pair output index: all transactions with exactly 2 outputs
    ///
    /// This is Layer 1 of the oracle computation - identifies all candidate
    /// transactions for the payment+change pattern.
    fn compute_pair_index(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Validate version - combine all source vec versions
        let source_version = indexes.txindex.output_count.version()
            + indexes.txindex.input_count.version()
            + indexer.vecs.transactions.base_size.version()
            + indexer.vecs.transactions.total_size.version()
            + indexer.vecs.outputs.outputtype.version()
            + indexer.vecs.outputs.value.version()
            + indexer.vecs.inputs.outpoint.version()
            + indexes.height.dateindex.version();
        self.pairoutputindex_to_txindex
            .validate_computed_version_or_reset(source_version)?;
        self.height_to_first_pairoutputindex
            .validate_computed_version_or_reset(source_version)?;
        self.output0_value
            .validate_computed_version_or_reset(source_version)?;
        self.output1_value
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = indexer.vecs.blocks.timestamp.len();
        let total_txs = indexer.vecs.transactions.height.len();

        // Determine starting height (handle rollback + sync)
        let start_height = self
            .height_to_first_pairoutputindex
            .len()
            .min(starting_indexes.height.to_usize());

        // Truncation point for pair vecs: first_pairoutputindex of start_height block
        // (i.e., keep all pairs from blocks before start_height)
        let pair_truncate_len =
            if start_height > 0 && start_height <= self.height_to_first_pairoutputindex.len() {
                self.height_to_first_pairoutputindex
                    .iter()?
                    .get(Height::from(start_height))
                    .map(|idx| idx.to_usize())
                    .unwrap_or(self.pairoutputindex_to_txindex.len())
            } else if start_height == 0 {
                0
            } else {
                self.pairoutputindex_to_txindex.len()
            }
            .min(self.pairoutputindex_to_txindex.len())
            .min(self.output0_value.len())
            .min(self.output1_value.len());

        // Truncate all vecs together
        self.height_to_first_pairoutputindex
            .truncate_if_needed_at(start_height)?;
        self.pairoutputindex_to_txindex
            .truncate_if_needed_at(pair_truncate_len)?;
        self.output0_value
            .truncate_if_needed_at(pair_truncate_len)?;
        self.output1_value
            .truncate_if_needed_at(pair_truncate_len)?;

        if start_height >= total_heights {
            return Ok(());
        }

        info!(
            "Computing pair index from height {} to {}",
            start_height, total_heights
        );

        let mut height_to_first_txindex_iter = indexer.vecs.transactions.first_txindex.into_iter();
        let mut txindex_to_output_count_iter = indexes.txindex.output_count.iter();
        let mut txindex_to_input_count_iter = indexes.txindex.input_count.iter();
        let mut txindex_to_base_size_iter = indexer.vecs.transactions.base_size.into_iter();
        let mut txindex_to_total_size_iter = indexer.vecs.transactions.total_size.into_iter();
        let mut txindex_to_first_txoutindex_iter =
            indexer.vecs.transactions.first_txoutindex.into_iter();
        let mut txindex_to_first_txinindex_iter =
            indexer.vecs.transactions.first_txinindex.into_iter();
        let mut txoutindex_to_outputtype_iter = indexer.vecs.outputs.outputtype.into_iter();
        let mut txoutindex_to_value_iter = indexer.vecs.outputs.value.into_iter();
        let mut txinindex_to_outpoint_iter = indexer.vecs.inputs.outpoint.into_iter();
        let mut height_to_dateindex_iter = indexes.height.dateindex.iter();
        let mut dateindex_to_first_height_iter = indexes.dateindex.first_height.iter();

        // Track current date for same-day spend check
        let mut current_dateindex = DateIndex::from(0usize);
        let mut current_date_first_txindex = TxIndex::from(0usize);

        let mut last_progress = (start_height * 100 / total_heights.max(1)) as u8;

        for height in start_height..total_heights {
            // Record first pairoutputindex for this block
            let first_pairoutputindex =
                PairOutputIndex::from(self.pairoutputindex_to_txindex.len());
            self.height_to_first_pairoutputindex
                .push(first_pairoutputindex);

            // Get transaction range for this block
            let first_txindex = height_to_first_txindex_iter.get_at_unwrap(height);
            let next_first_txindex = height_to_first_txindex_iter
                .get_at(height + 1)
                .unwrap_or(TxIndex::from(total_txs));

            // Update current date tracking for same-day spend check
            let block_dateindex = height_to_dateindex_iter.get_unwrap(Height::from(height));
            if block_dateindex != current_dateindex {
                current_dateindex = block_dateindex;
                if let Some(first_height) = dateindex_to_first_height_iter.get(block_dateindex) {
                    current_date_first_txindex = height_to_first_txindex_iter
                        .get_at(first_height.to_usize())
                        .unwrap_or(first_txindex);
                }
            }

            // Skip coinbase (first tx in block)
            let tx_start = first_txindex.to_usize() + 1;
            let tx_end = next_first_txindex.to_usize();

            for txindex in tx_start..tx_end {
                // Check output count first (most common filter)
                let output_count: StoredU64 =
                    txindex_to_output_count_iter.get_unwrap(TxIndex::from(txindex));
                if *output_count != 2 {
                    continue;
                }

                // Filter: 1-5 inputs (same as UTXOracle)
                let input_count: StoredU64 =
                    txindex_to_input_count_iter.get_unwrap(TxIndex::from(txindex));
                if *input_count == 0 || *input_count > 5 {
                    continue;
                }

                // Filter: max 2500 bytes total witness size
                let base_size: StoredU32 = txindex_to_base_size_iter.get_at_unwrap(txindex);
                let total_size: StoredU32 = txindex_to_total_size_iter.get_at_unwrap(txindex);
                let witness_size = *total_size - *base_size;
                if witness_size > 2500 {
                    continue;
                }

                // Filter: only standard payment types (no OP_RETURN, P2TR, P2MS, Empty, Unknown)
                let first_txoutindex = txindex_to_first_txoutindex_iter.get_at_unwrap(txindex);
                let out0_type =
                    txoutindex_to_outputtype_iter.get_at_unwrap(first_txoutindex.to_usize());
                let out1_type =
                    txoutindex_to_outputtype_iter.get_at_unwrap(first_txoutindex.to_usize() + 1);
                if !matches!(
                    out0_type,
                    OutputType::P2PK65
                        | OutputType::P2PK33
                        | OutputType::P2PKH
                        | OutputType::P2SH
                        | OutputType::P2WPKH
                        | OutputType::P2WSH
                        | OutputType::P2A
                ) || !matches!(
                    out1_type,
                    OutputType::P2PK65
                        | OutputType::P2PK33
                        | OutputType::P2PKH
                        | OutputType::P2SH
                        | OutputType::P2WPKH
                        | OutputType::P2WSH
                        | OutputType::P2A
                ) {
                    continue;
                }

                // Filter: no same-day spends (input spending output confirmed today)
                let first_txinindex = txindex_to_first_txinindex_iter.get_at_unwrap(txindex);
                let mut has_same_day_spend = false;
                for i in 0..*input_count as usize {
                    let txinindex = first_txinindex.to_usize() + i;
                    let outpoint = txinindex_to_outpoint_iter.get_at_unwrap(txinindex);
                    if !outpoint.is_coinbase() && outpoint.txindex() >= current_date_first_txindex {
                        has_same_day_spend = true;
                        break;
                    }
                }
                if has_same_day_spend {
                    continue;
                }

                // Get output values (Layer 3)
                let value0: Sats =
                    txoutindex_to_value_iter.get_at_unwrap(first_txoutindex.to_usize());
                let value1: Sats =
                    txoutindex_to_value_iter.get_at_unwrap(first_txoutindex.to_usize() + 1);

                // Filter: skip if BOTH outputs are round BTC amounts (not price-related)
                if value0.is_round_btc() && value1.is_round_btc() {
                    continue;
                }

                // Store Layer 1 & 3 data
                // Note: min/max sats filtering done per-output in OracleBins::sats_to_bin
                self.pairoutputindex_to_txindex.push(TxIndex::from(txindex));
                self.output0_value.push(value0);
                self.output1_value.push(value1);
            }

            // Log and flush every 1%
            let progress = (height * 100 / total_heights.max(1)) as u8;
            if progress > last_progress {
                last_progress = progress;
                info!("Pair index computation: {}%", progress);

                let _lock = exit.lock();
                self.pairoutputindex_to_txindex.write()?;
                self.height_to_first_pairoutputindex.write()?;
                self.output0_value.write()?;
                self.output1_value.write()?;
            }
        }

        // Final write
        {
            let _lock = exit.lock();
            self.pairoutputindex_to_txindex.write()?;
            self.height_to_first_pairoutputindex.write()?;
            self.output0_value.write()?;
            self.output1_value.write()?;
        }

        info!(
            "Pair index complete: {} pairs across {} blocks",
            self.pairoutputindex_to_txindex.len(),
            self.height_to_first_pairoutputindex.len()
        );

        Ok(())
    }

    /// Compute phase histograms per block (Layer 4)
    ///
    /// Bins output values by frac(log10(sats)) into 100 bins per block.
    fn compute_phase_histograms(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let source_version = self.pairoutputindex_to_txindex.version();
        self.phase_histogram
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = self.height_to_first_pairoutputindex.len();
        let total_pairs = self.pairoutputindex_to_txindex.len();

        let start_height = self
            .phase_histogram
            .len()
            .min(starting_indexes.height.to_usize());

        self.phase_histogram.truncate_if_needed_at(start_height)?;

        if start_height >= total_heights {
            return Ok(());
        }

        info!(
            "Computing phase histograms from height {} to {}",
            start_height, total_heights
        );

        let mut output0_iter = self.output0_value.iter()?;
        let mut output1_iter = self.output1_value.iter()?;
        let mut height_to_first_pair_iter = self.height_to_first_pairoutputindex.iter()?;

        let mut last_progress = (start_height * 100 / total_heights.max(1)) as u8;

        for height in start_height..total_heights {
            // Get pair range for this block
            let first_pair = height_to_first_pair_iter
                .get_unwrap(Height::from(height))
                .to_usize();
            let next_first_pair = height_to_first_pair_iter
                .get(Height::from(height + 1))
                .map(|p| p.to_usize())
                .unwrap_or(total_pairs);

            // Build phase histogram
            let mut histogram = OracleBins::ZERO;

            for pair_idx in first_pair..next_first_pair {
                let pair_idx = PairOutputIndex::from(pair_idx);

                let sats0: Sats = output0_iter.get_unwrap(pair_idx);
                let sats1: Sats = output1_iter.get_unwrap(pair_idx);

                histogram.add(sats0);
                histogram.add(sats1);
            }

            self.phase_histogram.push(histogram);

            // Progress logging
            let progress = (height * 100 / total_heights.max(1)) as u8;
            if progress > last_progress {
                last_progress = progress;
                info!("Phase histogram computation: {}%", progress);

                let _lock = exit.lock();
                self.phase_histogram.write()?;
            }
        }

        // Final write
        {
            let _lock = exit.lock();
            self.phase_histogram.write()?;
        }

        info!(
            "Phase histograms complete: {} blocks",
            self.phase_histogram.len()
        );

        Ok(())
    }

    /// Compute phase oracle prices (Layer 5)
    ///
    /// Derives prices from phase histograms using peak finding.
    /// Uses monthly calibration anchors (2010-2020) then yearly (2021+).
    fn compute_phase_prices(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        /// Monthly calibration anchors 2010-2020, then yearly 2021+
        /// Format: (first_height_of_period, open_price)
        const ANCHORS: [(u32, f64); 129] = [
            // 2010 (monthly from Oct)
            (82_998, 0.06), // 2010-10-01
            (88_893, 0.19), // 2010-11-01
            (94_802, 0.20), // 2010-12-01
            // 2011
            (100_410, 0.30),  // 2011-01-01
            (105_571, 0.55),  // 2011-02-01
            (111_137, 0.86),  // 2011-03-01
            (116_039, 0.78),  // 2011-04-01
            (121_127, 3.50),  // 2011-05-01
            (127_866, 8.74),  // 2011-06-01
            (134_122, 16.10), // 2011-07-01
            (139_036, 13.35), // 2011-08-01
            (143_409, 8.19),  // 2011-09-01
            (147_566, 5.14),  // 2011-10-01
            (151_315, 3.24),  // 2011-11-01
            (155_452, 2.97),  // 2011-12-01
            // 2012
            (160_037, 4.72),  // 2012-01-01
            (164_781, 5.48),  // 2012-02-01
            (169_136, 4.86),  // 2012-03-01
            (173_805, 4.90),  // 2012-04-01
            (178_015, 4.94),  // 2012-05-01
            (182_429, 5.18),  // 2012-06-01
            (186_964, 6.68),  // 2012-07-01
            (191_737, 9.35),  // 2012-08-01
            (196_616, 10.16), // 2012-09-01
            (201_311, 12.40), // 2012-10-01
            (205_919, 11.20), // 2012-11-01
            (210_350, 12.56), // 2012-12-01
            // 2013
            (214_563, 13.51),   // 2013-01-01
            (219_007, 20.41),   // 2013-02-01
            (223_665, 33.38),   // 2013-03-01
            (229_008, 93.03),   // 2013-04-01
            (233_975, 139.22),  // 2013-05-01
            (238_952, 128.81),  // 2013-06-01
            (244_160, 97.51),   // 2013-07-01
            (249_525, 106.21),  // 2013-08-01
            (255_362, 141.00),  // 2013-09-01
            (260_989, 141.89),  // 2013-10-01
            (267_188, 211.17),  // 2013-11-01
            (272_375, 1205.80), // 2013-12-01
            // 2014
            (277_996, 739.28), // 2014-01-01
            (283_468, 805.22), // 2014-02-01
            (288_370, 549.99), // 2014-03-01
            (293_483, 456.98), // 2014-04-01
            (298_513, 449.02), // 2014-05-01
            (303_552, 626.21), // 2014-06-01
            (308_672, 640.79), // 2014-07-01
            (313_404, 580.00), // 2014-08-01
            (318_531, 477.81), // 2014-09-01
            (323_269, 387.00), // 2014-10-01
            (327_939, 336.82), // 2014-11-01
            (332_363, 379.89), // 2014-12-01
            // 2015
            (336_861, 322.30), // 2015-01-01
            (341_392, 215.80), // 2015-02-01
            (345_611, 255.70), // 2015-03-01
            (350_162, 244.51), // 2015-04-01
            (354_416, 236.11), // 2015-05-01
            (358_881, 228.70), // 2015-06-01
            (363_263, 262.89), // 2015-07-01
            (367_846, 284.45), // 2015-08-01
            (372_441, 231.35), // 2015-09-01
            (376_910, 236.49), // 2015-10-01
            (381_470, 316.00), // 2015-11-01
            (386_119, 376.88), // 2015-12-01
            // 2016
            (391_182, 429.02), // 2016-01-01
            (396_049, 365.52), // 2016-02-01
            (400_601, 438.99), // 2016-03-01
            (405_179, 416.02), // 2016-04-01
            (409_638, 446.60), // 2016-05-01
            (414_258, 530.69), // 2016-06-01
            (418_723, 671.91), // 2016-07-01
            (423_088, 624.22), // 2016-08-01
            (427_737, 573.80), // 2016-09-01
            (432_284, 609.67), // 2016-10-01
            (436_828, 697.69), // 2016-11-01
            (441_341, 742.33), // 2016-12-01
            // 2017
            (446_033, 970.41),  // 2017-01-01
            (450_945, 968.74),  // 2017-02-01
            (455_200, 1190.37), // 2017-03-01
            (459_832, 1080.82), // 2017-04-01
            (464_270, 1362.02), // 2017-05-01
            (469_122, 2299.05), // 2017-06-01
            (473_593, 2455.42), // 2017-07-01
            (478_479, 2865.02), // 2017-08-01
            (482_885, 4737.93), // 2017-09-01
            (487_740, 4334.18), // 2017-10-01
            (492_558, 6439.52), // 2017-11-01
            (496_932, 9968.39), // 2017-12-01
            // 2018
            (501_961, 13888.32), // 2018-01-01
            (507_016, 10115.79), // 2018-02-01
            (511_385, 10306.80), // 2018-03-01
            (516_040, 6922.18),  // 2018-04-01
            (520_650, 9243.39),  // 2018-05-01
            (525_367, 7486.93),  // 2018-06-01
            (529_967, 6386.45),  // 2018-07-01
            (534_613, 7725.93),  // 2018-08-01
            (539_416, 7016.31),  // 2018-09-01
            (543_835, 6565.64),  // 2018-10-01
            (548_214, 6305.13),  // 2018-11-01
            (552_084, 3971.61),  // 2018-12-01
            // 2019
            (556_459, 3692.35),  // 2019-01-01
            (560_984, 3411.57),  // 2019-02-01
            (565_109, 3792.17),  // 2019-03-01
            (569_659, 4095.32),  // 2019-04-01
            (573_997, 5269.55),  // 2019-05-01
            (578_718, 8542.59),  // 2019-06-01
            (583_237, 10754.91), // 2019-07-01
            (588_007, 10085.57), // 2019-08-01
            (592_683, 9600.93),  // 2019-09-01
            (597_318, 8303.79),  // 2019-10-01
            (601_842, 9152.56),  // 2019-11-01
            (606_088, 7554.92),  // 2019-12-01
            // 2020
            (610_691, 7167.07),  // 2020-01-01
            (615_428, 9333.17),  // 2020-02-01
            (619_582, 8526.76),  // 2020-03-01
            (623_837, 6424.03),  // 2020-04-01
            (628_350, 8627.93),  // 2020-05-01
            (632_542, 9448.95),  // 2020-06-01
            (637_091, 9134.01),  // 2020-07-01
            (641_680, 11354.08), // 2020-08-01
            (646_201, 11657.26), // 2020-09-01
            (650_732, 10779.19), // 2020-10-01
            (654_933, 13809.85), // 2020-11-01
            (658_977, 19698.14), // 2020-12-01
            // 2021+ (yearly)
            (663_913, 28_980.45), // 2021-01-01
            (716_599, 46_195.56), // 2022-01-01
            (769_787, 16_528.89), // 2023-01-01
            (823_786, 42_241.10), // 2024-01-01
            (877_259, 93_576.00), // 2025-01-01
            (930_341, 87_648.22), // 2026-01-01
        ];

        /// Find the calibration price for a given height
        fn anchor_price_for_height(height: usize) -> Option<f64> {
            let mut result = None;
            for &(anchor_height, price) in &ANCHORS {
                if height >= anchor_height as usize {
                    result = Some(price);
                } else {
                    break;
                }
            }
            result
        }

        let source_version = self.phase_histogram.version();
        self.phase_price_cents
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = self.phase_histogram.len();

        let start_height = self
            .phase_price_cents
            .len()
            .min(starting_indexes.height.to_usize());

        self.phase_price_cents.truncate_if_needed_at(start_height)?;

        if start_height >= total_heights {
            return Ok(());
        }

        info!(
            "Computing phase prices from height {} to {}",
            start_height, total_heights
        );

        let mut histogram_iter = self.phase_histogram.iter()?;
        let mut last_progress = (start_height * 100 / total_heights.max(1)) as u8;

        // Fixed exponent calibrated for ~$63,000 (ceil(log10(63000)) = 5)
        const EXPONENT: f64 = 5.0;

        /// Convert a bin to price using anchor for decade selection
        fn bin_to_price(bin: usize, anchor_price: f64) -> f64 {
            let phase = (bin as f64 + 0.5) / PHASE_BINS as f64;
            let raw_price = 10.0_f64.powf(EXPONENT - phase);
            let decade_ratio = (anchor_price / raw_price).log10().round();
            raw_price * 10.0_f64.powf(decade_ratio)
        }

        for height in start_height..total_heights {
            // Before first anchor (pre-Oct 2010), output 0
            let anchor_price = match anchor_price_for_height(height) {
                Some(price) => price,
                None => {
                    self.phase_price_cents.push(Cents::ZERO);
                    continue;
                }
            };

            let histogram = histogram_iter.get_unwrap(Height::from(height));

            // Skip empty histograms, use anchor price
            if histogram.total_count() == 0 {
                let price_cents = Cents::from((anchor_price * 100.0) as i64);
                self.phase_price_cents.push(price_cents);
                continue;
            }

            // Find peak bin, skipping bin 0 (round BTC amounts cluster there)
            let peak_bin = histogram
                .bins
                .iter()
                .enumerate()
                .filter(|(bin, _)| *bin != 0)
                .max_by_key(|(_, count)| *count)
                .map(|(bin, _)| bin)
                .unwrap_or(0);

            let price = bin_to_price(peak_bin, anchor_price);

            // Clamp to reasonable range ($0.001 to $10M)
            let price = price.clamp(0.001, 10_000_000.0);

            let price_cents = Cents::from((price * 100.0) as i64);
            self.phase_price_cents.push(price_cents);

            // Progress logging
            let progress = (height * 100 / total_heights.max(1)) as u8;
            if progress > last_progress {
                last_progress = progress;
                info!("Phase price computation: {}%", progress);

                let _lock = exit.lock();
                self.phase_price_cents.write()?;
            }
        }

        // Final write
        {
            let _lock = exit.lock();
            self.phase_price_cents.write()?;
        }

        info!(
            "Phase prices complete: {} blocks",
            self.phase_price_cents.len()
        );

        Ok(())
    }

    /// Compute daily distribution (min, max, average, percentiles) from phase oracle prices
    fn compute_phase_daily_average(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        info!("Computing phase daily distribution");

        self.phase_daily_cents.compute(
            starting_indexes.dateindex,
            &self.phase_price_cents,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )?;

        info!(
            "Phase daily distribution complete: {} days",
            self.phase_daily_cents.len()
        );

        Ok(())
    }

    /// Compute oracle prices from on-chain data (UTXOracle port)
    fn compute_prices(
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
        let mut last_progress =
            (start_height.to_usize() * 100 / last_height.to_usize().max(1)) as u8;
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
            let progress = (height.to_usize() * 100 / last_height.to_usize().max(1)) as u8;
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
                for (i, value) in values.iter_mut().enumerate() {
                    let txoutindex = first_txoutindex.to_usize() + i;
                    let outputtype = txoutindex_to_outputtype_iter.get_at_unwrap(txoutindex);
                    if outputtype == OutputType::OpReturn {
                        has_opreturn = true;
                        break;
                    }
                    *value = txoutindex_to_value_iter.get_at_unwrap(txoutindex);
                }
                if has_opreturn {
                    continue;
                }

                // Check witness size per input (SegWit era only, activated Aug 2017)
                // Pre-SegWit transactions have no witness data
                // Python checks each input's witness ≤ 500 bytes; we approximate with average
                if cached_year >= 2017 {
                    let base_size: StoredU32 = txindex_to_base_size_iter.get_at_unwrap(txindex);
                    let total_size: StoredU32 = txindex_to_total_size_iter.get_at_unwrap(txindex);
                    let witness_size = *total_size - *base_size;
                    if witness_size / *input_count as u32 > 500 {
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

            // Periodic flush to avoid data loss on long computations
            if height.to_usize() % FLUSH_INTERVAL == 0 {
                let _lock = exit.lock();
                self.price_cents.write()?;
            }
        }

        // Final write
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

            self.ohlc_cents.truncate_push(dateindex, ohlc)?;
            self.tx_count
                .truncate_push(dateindex, StoredU32::from(tx_count))?;
        }

        // Write daily data
        {
            let _lock = exit.lock();
            self.ohlc_cents.write()?;
            self.tx_count.write()?;
        }

        Ok(())
    }

    /// Compute Phase Oracle V2 - Step 1: Per-block 200-bin phase histograms
    ///
    /// Uses ALL outputs (like Python test), filtered only by sats range (1k-100k BTC).
    /// This is different from the pair-filtered approach used by UTXOracle.
    fn compute_phase_v2_histograms(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let source_version = indexer.vecs.outputs.value.version();
        self.phase_v2_histogram
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = indexer.vecs.blocks.timestamp.len();

        let start_height = self
            .phase_v2_histogram
            .len()
            .min(starting_indexes.height.to_usize());

        self.phase_v2_histogram
            .truncate_if_needed_at(start_height)?;

        if start_height >= total_heights {
            return Ok(());
        }

        info!(
            "Computing phase V2 histograms from height {} to {}",
            start_height, total_heights
        );

        let mut height_to_first_txindex_iter = indexer.vecs.transactions.first_txindex.into_iter();
        let mut txindex_to_first_txoutindex_iter =
            indexer.vecs.transactions.first_txoutindex.into_iter();
        let mut txindex_to_output_count_iter = indexes.txindex.output_count.iter();
        let mut txoutindex_to_value_iter = indexer.vecs.outputs.value.into_iter();

        let total_txs = indexer.vecs.transactions.height.len();
        let mut last_progress = (start_height * 100 / total_heights.max(1)) as u8;

        for height in start_height..total_heights {
            // Get transaction range for this block
            let first_txindex = height_to_first_txindex_iter.get_at_unwrap(height);
            let next_first_txindex = height_to_first_txindex_iter
                .get_at(height + 1)
                .unwrap_or(TxIndex::from(total_txs));

            // Build phase histogram from ALL outputs in this block
            let mut histogram = OracleBinsV2::ZERO;

            for txindex in first_txindex.to_usize()..next_first_txindex.to_usize() {
                // Get output count and first output for this transaction
                let first_txoutindex = txindex_to_first_txoutindex_iter.get_at_unwrap(txindex);
                let output_count: StoredU64 =
                    txindex_to_output_count_iter.get_unwrap(TxIndex::from(txindex));

                for i in 0..*output_count as usize {
                    let txoutindex = first_txoutindex.to_usize() + i;
                    let sats: Sats = txoutindex_to_value_iter.get_at_unwrap(txoutindex);
                    // OracleBinsV2::add already filters by sats range (1k to 100k BTC)
                    histogram.add(sats);
                }
            }

            self.phase_v2_histogram.push(histogram);

            // Progress logging
            let progress = (height * 100 / total_heights.max(1)) as u8;
            if progress > last_progress {
                last_progress = progress;
                info!("Phase V2 histogram computation: {}%", progress);

                let _lock = exit.lock();
                self.phase_v2_histogram.write()?;
            }
        }

        // Final write
        {
            let _lock = exit.lock();
            self.phase_v2_histogram.write()?;
        }

        info!(
            "Phase V2 histograms complete: {} blocks",
            self.phase_v2_histogram.len()
        );

        Ok(())
    }

    /// Compute Phase Oracle V2 - Step 2: Per-block prices using cross-correlation
    fn compute_phase_v2_prices(
        &mut self,
        indexes: &indexes::Vecs,
        price_cents: &cents::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let source_version = self.phase_v2_histogram.version();
        self.phase_v2_price_cents
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = self.phase_v2_histogram.len();

        let start_height = self
            .phase_v2_price_cents
            .len()
            .min(starting_indexes.height.to_usize());

        self.phase_v2_price_cents
            .truncate_if_needed_at(start_height)?;

        if start_height >= total_heights {
            return Ok(());
        }

        info!(
            "Computing phase V2 prices from height {} to {}",
            start_height, total_heights
        );

        let mut histogram_iter = self.phase_v2_histogram.iter()?;
        let mut height_to_dateindex_iter = indexes.height.dateindex.iter();

        // For weekly OHLC anchors
        let mut price_ohlc_iter = price_cents.ohlc.dateindex.iter()?;
        let mut dateindex_to_weekindex_iter = indexes.dateindex.weekindex.iter();
        let mut weekindex_to_first_dateindex_iter = indexes.weekindex.first_dateindex.iter();
        let mut weekindex_dateindex_count_iter = indexes.weekindex.dateindex_count.iter();

        let mut last_progress = (start_height * 100 / total_heights.max(1)) as u8;

        // Track previous price for fallback
        let mut prev_price_cents = if start_height > 0 {
            self.phase_v2_price_cents
                .iter()?
                .get(Height::from(start_height - 1))
                .unwrap_or(Cents::from(10_000_000i64))
        } else {
            Cents::from(10_000_000i64) // Default ~$100k
        };

        for height in start_height..total_heights {
            let height_idx = Height::from(height);
            let histogram: OracleBinsV2 = histogram_iter.get_unwrap(height_idx);

            // Get weekly anchor for this block's date
            let dateindex = height_to_dateindex_iter.get(height_idx);
            let weekly_bounds: Option<(f64, f64)> = dateindex.and_then(|di| {
                let wi = dateindex_to_weekindex_iter.get(di)?;
                let first_di = weekindex_to_first_dateindex_iter.get(wi)?;
                let count = weekindex_dateindex_count_iter
                    .get(wi)
                    .map(|c| *c as usize)?;

                let mut low = Cents::from(i64::MAX);
                let mut high = Cents::from(0i64);

                for i in 0..count {
                    let di = DateIndex::from(first_di.to_usize() + i);
                    if let Some(ohlc) = price_ohlc_iter.get(di) {
                        if *ohlc.low < low {
                            low = *ohlc.low;
                        }
                        if *ohlc.high > high {
                            high = *ohlc.high;
                        }
                    }
                }

                if i64::from(low) > 0 && i64::from(high) > 0 {
                    Some((
                        i64::from(low) as f64 / 100.0,
                        i64::from(high) as f64 / 100.0,
                    ))
                } else {
                    None
                }
            });

            // Compute price using cross-correlation
            let price_cents = if histogram.total_count() >= 10 {
                // Convert OracleBinsV2 to PhaseHistogramV2
                let mut phase_hist = PhaseHistogramV2::new();
                for (i, &count) in histogram.bins.iter().enumerate() {
                    if count > 0 {
                        let phase = (i as f64 + 0.5) / 200.0;
                        let log_sats = 6.0 + phase;
                        let sats = 10.0_f64.powf(log_sats);
                        for _ in 0..count {
                            phase_hist.add(Sats::from(sats as u64));
                        }
                    }
                }

                if let Some((low, high)) = weekly_bounds {
                    // Have weekly anchor - constrained search
                    let (phase_min, phase_max) = phase_range_from_anchor(low, high, 0.05);
                    let (best_phase, _corr) =
                        find_best_phase(&phase_hist, 2, Some(phase_min), Some(phase_max));
                    let price = phase_to_price(best_phase, low, high);
                    Cents::from((price * 100.0) as i64)
                } else {
                    // No anchor - use previous price as reference
                    let anchor_low = (i64::from(prev_price_cents) as f64 / 100.0) * 0.5;
                    let anchor_high = (i64::from(prev_price_cents) as f64 / 100.0) * 2.0;
                    let (best_phase, _corr) = find_best_phase(&phase_hist, 2, None, None);
                    let price = phase_to_price(best_phase, anchor_low, anchor_high);
                    Cents::from((price * 100.0) as i64)
                }
            } else {
                // Too few outputs - use previous price
                prev_price_cents
            };

            prev_price_cents = price_cents;
            self.phase_v2_price_cents.push(price_cents);

            // Progress logging
            let progress = (height * 100 / total_heights.max(1)) as u8;
            if progress > last_progress {
                last_progress = progress;
                info!("Phase V2 price computation: {}%", progress);

                let _lock = exit.lock();
                self.phase_v2_price_cents.write()?;
            }
        }

        // Final write
        {
            let _lock = exit.lock();
            self.phase_v2_price_cents.write()?;
        }

        info!(
            "Phase V2 prices complete: {} blocks",
            self.phase_v2_price_cents.len()
        );

        Ok(())
    }

    /// Compute Phase Oracle V2 - Peak prices using direct peak finding (like V1)
    fn compute_phase_v2_peak_prices(
        &mut self,
        indexes: &indexes::Vecs,
        price_cents: &cents::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let source_version = self.phase_v2_histogram.version();
        self.phase_v2_peak_price_cents
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = self.phase_v2_histogram.len();

        let start_height = self
            .phase_v2_peak_price_cents
            .len()
            .min(starting_indexes.height.to_usize());

        self.phase_v2_peak_price_cents
            .truncate_if_needed_at(start_height)?;

        if start_height >= total_heights {
            return Ok(());
        }

        info!(
            "Computing phase V2 peak prices from height {} to {}",
            start_height, total_heights
        );

        let mut histogram_iter = self.phase_v2_histogram.iter()?;
        let mut height_to_dateindex_iter = indexes.height.dateindex.iter();

        // For weekly OHLC anchors
        let mut price_ohlc_iter = price_cents.ohlc.dateindex.iter()?;
        let mut dateindex_to_weekindex_iter = indexes.dateindex.weekindex.iter();
        let mut weekindex_to_first_dateindex_iter = indexes.weekindex.first_dateindex.iter();
        let mut weekindex_dateindex_count_iter = indexes.weekindex.dateindex_count.iter();

        let mut last_progress = (start_height * 100 / total_heights.max(1)) as u8;

        // Track previous price for fallback
        let mut prev_price_cents = if start_height > 0 {
            self.phase_v2_peak_price_cents
                .iter()?
                .get(Height::from(start_height - 1))
                .unwrap_or(Cents::from(10_000_000i64))
        } else {
            Cents::from(10_000_000i64)
        };

        for height in start_height..total_heights {
            let height_idx = Height::from(height);
            let histogram: OracleBinsV2 = histogram_iter.get_unwrap(height_idx);

            // Get weekly anchor for decade selection
            let dateindex = height_to_dateindex_iter.get(height_idx);
            let anchor_price: Option<f64> = dateindex.and_then(|di| {
                let wi = dateindex_to_weekindex_iter.get(di)?;
                let first_di = weekindex_to_first_dateindex_iter.get(wi)?;
                let count = weekindex_dateindex_count_iter
                    .get(wi)
                    .map(|c| *c as usize)?;

                let mut sum = 0i64;
                let mut cnt = 0;
                for i in 0..count {
                    let di = DateIndex::from(first_di.to_usize() + i);
                    if let Some(ohlc) = price_ohlc_iter.get(di) {
                        sum += i64::from(*ohlc.close);
                        cnt += 1;
                    }
                }

                if cnt > 0 {
                    Some(sum as f64 / cnt as f64 / 100.0)
                } else {
                    None
                }
            });

            // Use anchor or previous price for decade selection
            let anchor = anchor_price.unwrap_or(i64::from(prev_price_cents) as f64 / 100.0);

            // Find peak bin directly (like V1) using 100 bins (downsample from 200)
            let price_cents = if histogram.total_count() >= 10 {
                // Downsample 200 bins to 100 bins
                let mut bins100 = [0u32; 100];
                for i in 0..100 {
                    bins100[i] = histogram.bins[i * 2] as u32 + histogram.bins[i * 2 + 1] as u32;
                }

                // Find peak bin, skipping bin 0 (round BTC amounts cluster there)
                let peak_bin = bins100
                    .iter()
                    .enumerate()
                    .filter(|(bin, _)| *bin != 0)
                    .max_by_key(|(_, count)| *count)
                    .map(|(bin, _)| bin)
                    .unwrap_or(0);

                // Convert bin to price using anchor for decade (100 bins)
                let phase = (peak_bin as f64 + 0.5) / 100.0;
                let base_price = 10.0_f64.powf(phase);

                // Find best decade
                let mut best_price = base_price;
                let mut best_dist = f64::MAX;
                for decade in -2..=6 {
                    let candidate = base_price * 10.0_f64.powi(decade);
                    let dist = (candidate - anchor).abs();
                    if dist < best_dist {
                        best_dist = dist;
                        best_price = candidate;
                    }
                }

                Cents::from((best_price.clamp(0.01, 10_000_000.0) * 100.0) as i64)
            } else {
                prev_price_cents
            };

            prev_price_cents = price_cents;
            self.phase_v2_peak_price_cents.push(price_cents);

            // Progress logging
            let progress = (height * 100 / total_heights.max(1)) as u8;
            if progress > last_progress {
                last_progress = progress;
                info!("Phase V2 peak price computation: {}%", progress);

                let _lock = exit.lock();
                self.phase_v2_peak_price_cents.write()?;
            }
        }

        // Final write
        {
            let _lock = exit.lock();
            self.phase_v2_peak_price_cents.write()?;
        }

        info!(
            "Phase V2 peak prices complete: {} blocks",
            self.phase_v2_peak_price_cents.len()
        );

        Ok(())
    }

    /// Compute Phase Oracle V2 - Daily distributions from per-block prices
    fn compute_phase_v2_daily(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        info!("Computing phase V2 daily distributions");

        // Cross-correlation based
        self.phase_v2_daily_cents.compute(
            starting_indexes.dateindex,
            &self.phase_v2_price_cents,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )?;

        // Peak-based
        self.phase_v2_peak_daily_cents.compute(
            starting_indexes.dateindex,
            &self.phase_v2_peak_price_cents,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )?;

        info!(
            "Phase V2 daily distributions complete: {} days",
            self.phase_v2_daily_cents.len()
        );

        Ok(())
    }

    /// Compute Phase Oracle V3 - Step 1: Per-block histograms with uniqueVal filtering
    ///
    /// Filters: >= 1000 sats, only outputs with unique values within their transaction.
    /// This reduces spurious peaks from exchange batched payouts and inscription spam.
    fn compute_phase_v3_histograms(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let source_version = indexer.vecs.outputs.value.version();
        self.phase_v3_histogram
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = indexer.vecs.blocks.timestamp.len();

        let start_height = self
            .phase_v3_histogram
            .len()
            .min(starting_indexes.height.to_usize());

        self.phase_v3_histogram
            .truncate_if_needed_at(start_height)?;

        if start_height >= total_heights {
            return Ok(());
        }

        info!(
            "Computing phase V3 histograms from height {} to {}",
            start_height, total_heights
        );

        let mut height_to_first_txindex_iter = indexer.vecs.transactions.first_txindex.into_iter();
        let mut txindex_to_first_txoutindex_iter =
            indexer.vecs.transactions.first_txoutindex.into_iter();
        let mut txindex_to_output_count_iter = indexes.txindex.output_count.iter();
        let mut txoutindex_to_value_iter = indexer.vecs.outputs.value.into_iter();

        let total_txs = indexer.vecs.transactions.height.len();
        let mut last_progress = (start_height * 100 / total_heights.max(1)) as u8;

        // Reusable buffer for collecting output values per transaction
        let mut tx_values: Vec<Sats> = Vec::with_capacity(16);

        for height in start_height..total_heights {
            // Get transaction range for this block
            let first_txindex = height_to_first_txindex_iter.get_at_unwrap(height);
            let next_first_txindex = height_to_first_txindex_iter
                .get_at(height + 1)
                .unwrap_or(TxIndex::from(total_txs));

            // Build phase histogram with uniqueVal filtering
            let mut histogram = OracleBinsV2::ZERO;

            // Skip coinbase (first tx in block)
            for txindex in (first_txindex.to_usize() + 1)..next_first_txindex.to_usize() {
                // Get output count and first output for this transaction
                let first_txoutindex = txindex_to_first_txoutindex_iter.get_at_unwrap(txindex);
                let output_count: StoredU64 =
                    txindex_to_output_count_iter.get_unwrap(TxIndex::from(txindex));

                // Collect all output values for this transaction
                tx_values.clear();
                for i in 0..*output_count as usize {
                    let txoutindex = first_txoutindex.to_usize() + i;
                    let sats: Sats = txoutindex_to_value_iter.get_at_unwrap(txoutindex);
                    tx_values.push(sats);
                }

                // Count occurrences of each value to determine uniqueness
                // For small output counts, simple nested loop is faster than HashMap
                for (i, &sats) in tx_values.iter().enumerate() {
                    // Skip if below minimum (BASE filter: >= 1000 sats)
                    if sats < Sats::_1K {
                        continue;
                    }

                    // Check if this value is unique within the transaction
                    let mut is_unique = true;
                    for (j, &other_sats) in tx_values.iter().enumerate() {
                        if i != j && sats == other_sats {
                            is_unique = false;
                            break;
                        }
                    }

                    // Only add unique values to histogram
                    if is_unique {
                        histogram.add(sats);
                    }
                }
            }

            self.phase_v3_histogram.push(histogram);

            // Progress logging
            let progress = (height * 100 / total_heights.max(1)) as u8;
            if progress > last_progress {
                last_progress = progress;
                info!("Phase V3 histogram computation: {}%", progress);

                let _lock = exit.lock();
                self.phase_v3_histogram.write()?;
            }
        }

        // Final write
        {
            let _lock = exit.lock();
            self.phase_v3_histogram.write()?;
        }

        info!(
            "Phase V3 histograms complete: {} blocks",
            self.phase_v3_histogram.len()
        );

        Ok(())
    }

    /// Compute Phase Oracle V3 - Step 2: Per-block prices using cross-correlation
    fn compute_phase_v3_prices(
        &mut self,
        indexes: &indexes::Vecs,
        price_cents: &cents::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let source_version = self.phase_v3_histogram.version();
        self.phase_v3_price_cents
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = self.phase_v3_histogram.len();

        let start_height = self
            .phase_v3_price_cents
            .len()
            .min(starting_indexes.height.to_usize());

        self.phase_v3_price_cents
            .truncate_if_needed_at(start_height)?;

        if start_height >= total_heights {
            return Ok(());
        }

        info!(
            "Computing phase V3 prices from height {} to {}",
            start_height, total_heights
        );

        let mut histogram_iter = self.phase_v3_histogram.iter()?;
        let mut height_to_dateindex_iter = indexes.height.dateindex.iter();

        // For weekly OHLC anchors
        let mut price_ohlc_iter = price_cents.ohlc.dateindex.iter()?;
        let mut dateindex_to_weekindex_iter = indexes.dateindex.weekindex.iter();
        let mut weekindex_to_first_dateindex_iter = indexes.weekindex.first_dateindex.iter();
        let mut weekindex_dateindex_count_iter = indexes.weekindex.dateindex_count.iter();

        let mut last_progress = (start_height * 100 / total_heights.max(1)) as u8;

        // Track previous price for fallback
        let mut prev_price_cents = if start_height > 0 {
            self.phase_v3_price_cents
                .iter()?
                .get(Height::from(start_height - 1))
                .unwrap_or(Cents::from(10_000_000i64))
        } else {
            Cents::from(10_000_000i64) // Default ~$100k
        };

        for height in start_height..total_heights {
            let height_idx = Height::from(height);
            let histogram: OracleBinsV2 = histogram_iter.get_unwrap(height_idx);

            // Get weekly anchor for this block's date
            let dateindex = height_to_dateindex_iter.get(height_idx);
            let weekly_bounds: Option<(f64, f64)> = dateindex.and_then(|di| {
                let wi = dateindex_to_weekindex_iter.get(di)?;
                let first_di = weekindex_to_first_dateindex_iter.get(wi)?;
                let count = weekindex_dateindex_count_iter
                    .get(wi)
                    .map(|c| *c as usize)?;

                let mut low = Cents::from(i64::MAX);
                let mut high = Cents::from(0i64);

                for i in 0..count {
                    let di = DateIndex::from(first_di.to_usize() + i);
                    if let Some(ohlc) = price_ohlc_iter.get(di) {
                        if *ohlc.low < low {
                            low = *ohlc.low;
                        }
                        if *ohlc.high > high {
                            high = *ohlc.high;
                        }
                    }
                }

                if i64::from(low) > 0 && i64::from(high) > 0 {
                    Some((
                        i64::from(low) as f64 / 100.0,
                        i64::from(high) as f64 / 100.0,
                    ))
                } else {
                    None
                }
            });

            // Compute price using cross-correlation
            let price_cents = if histogram.total_count() >= 10 {
                // Convert OracleBinsV2 to PhaseHistogramV2
                let mut phase_hist = PhaseHistogramV2::new();
                for (i, &count) in histogram.bins.iter().enumerate() {
                    if count > 0 {
                        let phase = (i as f64 + 0.5) / 200.0;
                        let log_sats = 6.0 + phase;
                        let sats = 10.0_f64.powf(log_sats);
                        for _ in 0..count {
                            phase_hist.add(Sats::from(sats as u64));
                        }
                    }
                }

                if let Some((low, high)) = weekly_bounds {
                    // Have weekly anchor - constrained search
                    let (phase_min, phase_max) = phase_range_from_anchor(low, high, 0.05);
                    let (best_phase, _corr) =
                        find_best_phase(&phase_hist, 2, Some(phase_min), Some(phase_max));
                    let price = phase_to_price(best_phase, low, high);
                    Cents::from((price * 100.0) as i64)
                } else {
                    // No anchor - use previous price as reference
                    let anchor_low = (i64::from(prev_price_cents) as f64 / 100.0) * 0.5;
                    let anchor_high = (i64::from(prev_price_cents) as f64 / 100.0) * 2.0;
                    let (best_phase, _corr) = find_best_phase(&phase_hist, 2, None, None);
                    let price = phase_to_price(best_phase, anchor_low, anchor_high);
                    Cents::from((price * 100.0) as i64)
                }
            } else {
                // Too few outputs - use previous price
                prev_price_cents
            };

            prev_price_cents = price_cents;
            self.phase_v3_price_cents.push(price_cents);

            // Progress logging
            let progress = (height * 100 / total_heights.max(1)) as u8;
            if progress > last_progress {
                last_progress = progress;
                info!("Phase V3 price computation: {}%", progress);

                let _lock = exit.lock();
                self.phase_v3_price_cents.write()?;
            }
        }

        // Final write
        {
            let _lock = exit.lock();
            self.phase_v3_price_cents.write()?;
        }

        info!(
            "Phase V3 prices complete: {} blocks",
            self.phase_v3_price_cents.len()
        );

        Ok(())
    }

    /// Compute Phase Oracle V3 - Peak prices using direct peak finding (like V1)
    fn compute_phase_v3_peak_prices(
        &mut self,
        indexes: &indexes::Vecs,
        price_cents: &cents::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let source_version = self.phase_v3_histogram.version();
        self.phase_v3_peak_price_cents
            .validate_computed_version_or_reset(source_version)?;

        let total_heights = self.phase_v3_histogram.len();

        let start_height = self
            .phase_v3_peak_price_cents
            .len()
            .min(starting_indexes.height.to_usize());

        self.phase_v3_peak_price_cents
            .truncate_if_needed_at(start_height)?;

        if start_height >= total_heights {
            return Ok(());
        }

        info!(
            "Computing phase V3 peak prices from height {} to {}",
            start_height, total_heights
        );

        let mut histogram_iter = self.phase_v3_histogram.iter()?;
        let mut height_to_dateindex_iter = indexes.height.dateindex.iter();

        // For weekly OHLC anchors
        let mut price_ohlc_iter = price_cents.ohlc.dateindex.iter()?;
        let mut dateindex_to_weekindex_iter = indexes.dateindex.weekindex.iter();
        let mut weekindex_to_first_dateindex_iter = indexes.weekindex.first_dateindex.iter();
        let mut weekindex_dateindex_count_iter = indexes.weekindex.dateindex_count.iter();

        let mut last_progress = (start_height * 100 / total_heights.max(1)) as u8;

        // Track previous price for fallback
        let mut prev_price_cents = if start_height > 0 {
            self.phase_v3_peak_price_cents
                .iter()?
                .get(Height::from(start_height - 1))
                .unwrap_or(Cents::from(10_000_000i64))
        } else {
            Cents::from(10_000_000i64)
        };

        for height in start_height..total_heights {
            let height_idx = Height::from(height);
            let histogram: OracleBinsV2 = histogram_iter.get_unwrap(height_idx);

            // Get weekly anchor for decade selection
            let dateindex = height_to_dateindex_iter.get(height_idx);
            let anchor_price: Option<f64> = dateindex.and_then(|di| {
                let wi = dateindex_to_weekindex_iter.get(di)?;
                let first_di = weekindex_to_first_dateindex_iter.get(wi)?;
                let count = weekindex_dateindex_count_iter
                    .get(wi)
                    .map(|c| *c as usize)?;

                let mut sum = 0i64;
                let mut cnt = 0;
                for i in 0..count {
                    let di = DateIndex::from(first_di.to_usize() + i);
                    if let Some(ohlc) = price_ohlc_iter.get(di) {
                        sum += i64::from(*ohlc.close);
                        cnt += 1;
                    }
                }

                if cnt > 0 {
                    Some(sum as f64 / cnt as f64 / 100.0)
                } else {
                    None
                }
            });

            // Use anchor or previous price for decade selection
            let anchor = anchor_price.unwrap_or(i64::from(prev_price_cents) as f64 / 100.0);

            // Find peak bin directly (like V1) using 100 bins (downsample from 200)
            let price_cents = if histogram.total_count() >= 10 {
                // Downsample 200 bins to 100 bins
                let mut bins100 = [0u32; 100];
                (0..100).for_each(|i| {
                    bins100[i] = histogram.bins[i * 2] as u32 + histogram.bins[i * 2 + 1] as u32;
                });

                // Find peak bin, skipping bin 0 (round BTC amounts cluster there)
                let peak_bin = bins100
                    .iter()
                    .enumerate()
                    .filter(|(bin, _)| *bin != 0)
                    .max_by_key(|(_, count)| *count)
                    .map(|(bin, _)| bin)
                    .unwrap_or(0);

                // Convert bin to price using anchor for decade (100 bins)
                let phase = (peak_bin as f64 + 0.5) / 100.0;
                let base_price = 10.0_f64.powf(phase);

                // Find best decade
                let mut best_price = base_price;
                let mut best_dist = f64::MAX;
                for decade in -2..=6 {
                    let candidate = base_price * 10.0_f64.powi(decade);
                    let dist = (candidate - anchor).abs();
                    if dist < best_dist {
                        best_dist = dist;
                        best_price = candidate;
                    }
                }

                Cents::from((best_price.clamp(0.01, 10_000_000.0) * 100.0) as i64)
            } else {
                prev_price_cents
            };

            prev_price_cents = price_cents;
            self.phase_v3_peak_price_cents.push(price_cents);

            // Progress logging
            let progress = (height * 100 / total_heights.max(1)) as u8;
            if progress > last_progress {
                last_progress = progress;
                info!("Phase V3 peak price computation: {}%", progress);

                let _lock = exit.lock();
                self.phase_v3_peak_price_cents.write()?;
            }
        }

        // Final write
        {
            let _lock = exit.lock();
            self.phase_v3_peak_price_cents.write()?;
        }

        info!(
            "Phase V3 peak prices complete: {} blocks",
            self.phase_v3_peak_price_cents.len()
        );

        Ok(())
    }

    /// Compute Phase Oracle V3 - Daily distributions from per-block prices
    fn compute_phase_v3_daily(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        info!("Computing phase V3 daily distributions");

        // Cross-correlation based
        self.phase_v3_daily_cents.compute(
            starting_indexes.dateindex,
            &self.phase_v3_price_cents,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )?;

        // Peak-based
        self.phase_v3_peak_daily_cents.compute(
            starting_indexes.dateindex,
            &self.phase_v3_peak_price_cents,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )?;

        info!(
            "Phase V3 daily distributions complete: {} days",
            self.phase_v3_daily_cents.len()
        );

        Ok(())
    }
}
