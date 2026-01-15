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
        // v32: Revert to simple anchor-based decade selection (no prev_price tracking)
        let phase_version = version + Version::new(25);
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
        })
    }
}
