use brk_error::Result;
use brk_types::{DateIndex, OHLCCents, OHLCDollars, Version};
use vecdb::{BytesVec, Database, ImportableVec, IterableCloneableVec, LazyVecFrom1, PcoVec};

use super::Vecs;
use crate::internal::{CentsToDollars, Distribution, LazyTransformDistribution};

impl Vecs {
    pub fn forced_import(db: &Database, parent_version: Version) -> Result<Self> {
        // v12: Add both-outputs-round filter
        let version = parent_version + Version::new(12);

        // Layer 1: Pair output index
        let pairoutputindex_to_txindex =
            PcoVec::forced_import(db, "pairoutputindex_to_txindex", version)?;
        let height_to_first_pairoutputindex =
            PcoVec::forced_import(db, "height_to_first_pairoutputindex", version)?;

        // Layer 3: Output values
        let output0_value = PcoVec::forced_import(db, "pair_output0_value", version)?;
        let output1_value = PcoVec::forced_import(db, "pair_output1_value", version)?;

        // Layer 4: Phase histograms (depends on Layer 1)
        let phase_histogram = BytesVec::forced_import(db, "phase_histogram", version)?;

        // Layer 5: Phase Oracle prices
        // v45: Back to decades (10x) + anchor only
        let phase_version = version + Version::new(38);
        let phase_price_cents = PcoVec::forced_import(db, "phase_price_cents", phase_version)?;
        let phase_daily_cents = Distribution::forced_import(db, "phase_daily", phase_version)?;
        let phase_daily_dollars = LazyTransformDistribution::from_distribution::<CentsToDollars>(
            "phase_daily_dollars",
            phase_version,
            &phase_daily_cents,
        );

        // UTXOracle (Python port)
        let price_cents = PcoVec::forced_import(db, "oracle_price_cents", version)?;
        let ohlc_cents = BytesVec::forced_import(db, "oracle_ohlc_cents", version)?;
        let tx_count = PcoVec::forced_import(db, "oracle_tx_count", version)?;

        let ohlc_dollars = LazyVecFrom1::init(
            "oracle_ohlc",
            version,
            ohlc_cents.boxed_clone(),
            |di: DateIndex, iter| iter.get(di).map(|o: OHLCCents| OHLCDollars::from(o)),
        );

        // Daily OHLC from height close only
        let close_ohlc_cents = BytesVec::forced_import(db, "close_ohlc_cents", version)?;
        let close_ohlc_dollars = LazyVecFrom1::init(
            "close_ohlc_dollars",
            version,
            close_ohlc_cents.boxed_clone(),
            |di: DateIndex, iter| iter.get(di).map(|o: OHLCCents| OHLCDollars::from(o)),
        );

        // Daily OHLC from height mid price ((open+close)/2)
        let mid_ohlc_cents = BytesVec::forced_import(db, "mid_ohlc_cents", version)?;
        let mid_ohlc_dollars = LazyVecFrom1::init(
            "mid_ohlc_dollars",
            version,
            mid_ohlc_cents.boxed_clone(),
            |di: DateIndex, iter| iter.get(di).map(|o: OHLCCents| OHLCDollars::from(o)),
        );

        // Phase Oracle V2 (round USD template matching)
        // v3: Peak prices use 100 bins (downsampled from 200)
        let phase_v2_version = version + Version::new(3);
        let phase_v2_histogram =
            BytesVec::forced_import(db, "phase_v2_histogram", phase_v2_version)?;
        let phase_v2_price_cents =
            PcoVec::forced_import(db, "phase_v2_price_cents", phase_v2_version)?;
        let phase_v2_peak_price_cents =
            PcoVec::forced_import(db, "phase_v2_peak_price_cents", phase_v2_version)?;
        let phase_v2_daily_cents =
            Distribution::forced_import(db, "phase_v2_daily", phase_v2_version)?;
        let phase_v2_daily_dollars =
            LazyTransformDistribution::from_distribution::<CentsToDollars>(
                "phase_v2_daily_dollars",
                phase_v2_version,
                &phase_v2_daily_cents,
            );
        let phase_v2_peak_daily_cents =
            Distribution::forced_import(db, "phase_v2_peak_daily", phase_v2_version)?;
        let phase_v2_peak_daily_dollars =
            LazyTransformDistribution::from_distribution::<CentsToDollars>(
                "phase_v2_peak_daily_dollars",
                phase_v2_version,
                &phase_v2_peak_daily_cents,
            );

        // Phase Oracle V3 (BASE + noP2TR + uniqueVal filter)
        // v5: Added noP2TR filter to reduce inscription spam
        let phase_v3_version = version + Version::new(5);
        let phase_v3_histogram =
            BytesVec::forced_import(db, "phase_v3_histogram", phase_v3_version)?;
        let phase_v3_price_cents =
            PcoVec::forced_import(db, "phase_v3_price_cents", phase_v3_version)?;
        let phase_v3_peak_price_cents =
            PcoVec::forced_import(db, "phase_v3_peak_price_cents", phase_v3_version)?;
        let phase_v3_daily_cents =
            Distribution::forced_import(db, "phase_v3_daily", phase_v3_version)?;
        let phase_v3_daily_dollars =
            LazyTransformDistribution::from_distribution::<CentsToDollars>(
                "phase_v3_daily_dollars",
                phase_v3_version,
                &phase_v3_daily_cents,
            );
        let phase_v3_peak_daily_cents =
            Distribution::forced_import(db, "phase_v3_peak_daily", phase_v3_version)?;
        let phase_v3_peak_daily_dollars =
            LazyTransformDistribution::from_distribution::<CentsToDollars>(
                "phase_v3_peak_daily_dollars",
                phase_v3_version,
                &phase_v3_peak_daily_cents,
            );

        Ok(Self {
            pairoutputindex_to_txindex,
            height_to_first_pairoutputindex,
            output0_value,
            output1_value,
            phase_histogram,
            phase_price_cents,
            phase_daily_cents,
            phase_daily_dollars,
            price_cents,
            ohlc_cents,
            ohlc_dollars,
            tx_count,
            close_ohlc_cents,
            close_ohlc_dollars,
            mid_ohlc_cents,
            mid_ohlc_dollars,
            phase_v2_histogram,
            phase_v2_price_cents,
            phase_v2_peak_price_cents,
            phase_v2_daily_cents,
            phase_v2_daily_dollars,
            phase_v2_peak_daily_cents,
            phase_v2_peak_daily_dollars,
            phase_v3_histogram,
            phase_v3_price_cents,
            phase_v3_peak_price_cents,
            phase_v3_daily_cents,
            phase_v3_daily_dollars,
            phase_v3_peak_daily_cents,
            phase_v3_peak_daily_dollars,
        })
    }
}
