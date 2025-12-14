use schemars::JsonSchema;
use serde::Serialize;

use crate::{FeeRate, Sats, VSize};

/// Block info in a mempool.space like format for fee estimation.
#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MempoolBlock {
    /// Total block size in weight units
    #[schemars(example = 3993472)]
    pub block_size: u64,

    /// Total block virtual size in vbytes
    #[schemars(example = 998368.0)]
    pub block_v_size: f64,

    /// Number of transactions in the projected block
    #[schemars(example = 863)]
    pub n_tx: u32,

    /// Total fees in satoshis
    #[schemars(example = 8875608)]
    pub total_fees: Sats,

    /// Median fee rate in sat/vB
    #[schemars(example = 10.5)]
    pub median_fee: FeeRate,

    /// Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]
    #[schemars(example = example_fee_range())]
    pub fee_range: [FeeRate; 7],
}

fn example_fee_range() -> [FeeRate; 7] {
    [
        FeeRate::new(1.0),
        FeeRate::new(2.42),
        FeeRate::new(8.1),
        FeeRate::new(10.14),
        FeeRate::new(11.05),
        FeeRate::new(12.04),
        FeeRate::new(302.11),
    ]
}

impl MempoolBlock {
    pub fn new(
        tx_count: u32,
        total_vsize: VSize,
        total_fee: Sats,
        fee_range: [FeeRate; 7],
    ) -> Self {
        let vsize_f64 = *total_vsize as f64;
        Self {
            block_size: *total_vsize * 4, // weight = vsize * 4
            block_v_size: vsize_f64,
            n_tx: tx_count,
            total_fees: total_fee,
            median_fee: fee_range[3],
            fee_range,
        }
    }
}
