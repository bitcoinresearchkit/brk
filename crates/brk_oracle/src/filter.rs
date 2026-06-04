use brk_types::{OutputType, Sats};

use crate::scale::{sats_to_bin, HistogramRaw};

/// Dust floor: outputs below this many sats are too small to be payments.
const MIN_SATS: u64 = 1000;

/// Output types skipped entirely (protocol-dominated, too noisy to carry the
/// round-dollar signal).
const EXCLUDED_OUTPUT_TYPES: &[OutputType] = &[OutputType::P2TR];

/// Bitmask form of [`EXCLUDED_OUTPUT_TYPES`], folded at compile time so
/// [`eligible_bin`] checks membership with a single AND.
const EXCLUDED_MASK: u16 = {
    let mut mask = 0u16;
    let mut i = 0;
    while i < EXCLUDED_OUTPUT_TYPES.len() {
        mask |= 1u16 << EXCLUDED_OUTPUT_TYPES[i] as u8;
        i += 1;
    }
    mask
};

/// Pre-modern transaction-output fan-out cap. Above this, the transaction is a
/// batch payout (exchange sweep, mixer fan-out), not a round-dollar payment.
pub const PRE_MODERN_TX_OUTPUT_FANOUT_CAP: usize = 100;

/// Modern-chain transaction-output fan-out cap. Dense post-630k blocks can
/// carry more genuine payment outputs, but very large fan-outs can still
/// dominate one EMA slot and create a false round-dollar ladder.
pub const MODERN_TX_OUTPUT_FANOUT_CAP: usize = 250;

/// Height where [`PRE_MODERN_TX_OUTPUT_FANOUT_CAP`] relaxes to
/// [`MODERN_TX_OUTPUT_FANOUT_CAP`].
pub const MODERN_TX_OUTPUT_FANOUT_CAP_START_HEIGHT: usize = 630_000;

#[inline(always)]
fn tx_output_fanout_cap(height: usize) -> usize {
    if height < MODERN_TX_OUTPUT_FANOUT_CAP_START_HEIGHT {
        PRE_MODERN_TX_OUTPUT_FANOUT_CAP
    } else {
        MODERN_TX_OUTPUT_FANOUT_CAP
    }
}

/// Bin index for `(sats, output_type)`, or `None` for an excluded type (P2TR),
/// dust, a round-BTC value, or an out-of-range bin. The per-output half of the
/// round-dollar payment filter.
#[inline(always)]
pub fn eligible_bin(sats: Sats, output_type: OutputType) -> Option<u16> {
    if EXCLUDED_MASK & (1u16 << output_type as u8) != 0 {
        return None;
    }
    if *sats < MIN_SATS || sats.is_common_round_value() {
        return None;
    }
    sats_to_bin(sats).map(|b| b as u16)
}

/// The on-chain round-dollar payment filter, shared by the indexer warm-up,
/// per-request reconstruction, and the mempool's live histogram so every path
/// bins identically. Calls `emit(bin)` for each eligible output, in order.
///
/// A whole transaction is dropped when it carries any OP_RETURN output (data
/// carriers, not payments) or when it has more than the height-specific fan-out
/// cap: [`PRE_MODERN_TX_OUTPUT_FANOUT_CAP`] below
/// [`MODERN_TX_OUTPUT_FANOUT_CAP_START_HEIGHT`],
/// [`MODERN_TX_OUTPUT_FANOUT_CAP`] at and above it. `height` is the block these
/// outputs belong to. Live or otherwise guaranteed-modern callers should use
/// [`for_each_modern_round_dollar_bin`] instead.
#[inline]
pub fn for_each_round_dollar_bin(
    height: usize,
    outputs: impl ExactSizeIterator<Item = (Sats, OutputType)> + Clone,
    mut emit: impl FnMut(u16),
) {
    if outputs.len() > tx_output_fanout_cap(height) {
        return;
    }
    if outputs.clone().any(|(_, ty)| ty == OutputType::OpReturn) {
        return;
    }
    for (sats, ty) in outputs {
        if let Some(bin) = eligible_bin(sats, ty) {
            emit(bin);
        }
    }
}

/// Heightless form of [`for_each_round_dollar_bin`] for live or otherwise
/// guaranteed-modern transaction streams. Applies
/// [`MODERN_TX_OUTPUT_FANOUT_CAP`].
#[inline]
pub fn for_each_modern_round_dollar_bin(
    outputs: impl ExactSizeIterator<Item = (Sats, OutputType)> + Clone,
    emit: impl FnMut(u16),
) {
    for_each_round_dollar_bin(MODERN_TX_OUTPUT_FANOUT_CAP_START_HEIGHT, outputs, emit);
}

/// Build a fresh eligible round-dollar payment histogram for one block's
/// non-coinbase transaction outputs.
#[inline]
pub fn payment_histogram<Outputs>(
    height: usize,
    txs: impl IntoIterator<Item = Outputs>,
) -> HistogramRaw
where
    Outputs: ExactSizeIterator<Item = (Sats, OutputType)> + Clone,
{
    let mut hist = HistogramRaw::zeros();
    for outputs in txs {
        for_each_round_dollar_bin(height, outputs, |bin| hist.increment(bin as usize));
    }
    hist
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payment_outputs(len: usize) -> impl ExactSizeIterator<Item = (Sats, OutputType)> + Clone {
        std::iter::repeat_n((Sats::new(12_345), OutputType::P2WPKH), len)
    }

    fn emitted_count(height: usize, len: usize) -> usize {
        let mut count = 0;
        for_each_round_dollar_bin(height, payment_outputs(len), |_| count += 1);
        count
    }

    #[test]
    fn early_fanout_cap_is_strict() {
        assert_eq!(
            emitted_count(
                MODERN_TX_OUTPUT_FANOUT_CAP_START_HEIGHT - 1,
                PRE_MODERN_TX_OUTPUT_FANOUT_CAP,
            ),
            PRE_MODERN_TX_OUTPUT_FANOUT_CAP
        );
        assert_eq!(
            emitted_count(
                MODERN_TX_OUTPUT_FANOUT_CAP_START_HEIGHT - 1,
                PRE_MODERN_TX_OUTPUT_FANOUT_CAP + 1,
            ),
            0
        );
    }

    #[test]
    fn modern_fanout_cap_is_relaxed_but_not_lifted() {
        assert_eq!(
            emitted_count(
                MODERN_TX_OUTPUT_FANOUT_CAP_START_HEIGHT,
                MODERN_TX_OUTPUT_FANOUT_CAP,
            ),
            MODERN_TX_OUTPUT_FANOUT_CAP
        );
        assert_eq!(
            emitted_count(
                MODERN_TX_OUTPUT_FANOUT_CAP_START_HEIGHT,
                MODERN_TX_OUTPUT_FANOUT_CAP + 1,
            ),
            0
        );
    }

    fn emitted_count_modern(len: usize) -> usize {
        let mut count = 0;
        for_each_modern_round_dollar_bin(payment_outputs(len), |_| count += 1);
        count
    }

    #[test]
    fn modern_helper_uses_modern_fanout_cap() {
        assert_eq!(
            emitted_count_modern(MODERN_TX_OUTPUT_FANOUT_CAP),
            MODERN_TX_OUTPUT_FANOUT_CAP
        );
        assert_eq!(emitted_count_modern(MODERN_TX_OUTPUT_FANOUT_CAP + 1), 0);
    }

    #[test]
    fn payment_histogram_drops_op_return_transaction() {
        let sats = Sats::new(12_345);
        let txs = vec![
            vec![(sats, OutputType::P2WPKH), (sats, OutputType::P2PKH)],
            vec![
                (Sats::new(54_321), OutputType::OpReturn),
                (sats, OutputType::P2WPKH),
            ],
        ];
        let hist = payment_histogram(
            MODERN_TX_OUTPUT_FANOUT_CAP_START_HEIGHT,
            txs.into_iter().map(|tx| tx.into_iter()),
        );

        let bin = eligible_bin(sats, OutputType::P2WPKH).unwrap() as usize;
        assert_eq!(hist[bin], 2);
    }

    #[test]
    fn builds_fresh_payment_histogram() {
        let sats = Sats::new(12_345);
        let txs = vec![vec![
            (sats, OutputType::P2WPKH),
            (Sats::new(100_000_000), OutputType::P2WPKH),
        ]];

        let hist = payment_histogram(
            MODERN_TX_OUTPUT_FANOUT_CAP_START_HEIGHT,
            txs.into_iter().map(|tx| tx.into_iter()),
        );

        let bin = eligible_bin(sats, OutputType::P2WPKH).unwrap() as usize;
        assert_eq!(hist[bin], 1);
    }
}
