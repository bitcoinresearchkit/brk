use brk_types::{OutputType, Sats};

use crate::scale::sats_to_bin;

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

/// A transaction with more than this many outputs is a batch payout (exchange
/// sweep, mixer fan-out), not a round-dollar payment, so it is dropped below
/// [`MAX_OUTPUTS_UNTIL_HEIGHT`].
pub const MAX_OUTPUTS: usize = 100;

/// Height below which the [`MAX_OUTPUTS`] cap applies. The thin 2018-2020
/// signal needs batch payouts removed to stay locked onto the round-dollar
/// pattern. Above this height on-chain volume is dense enough that the cap
/// removes more genuine signal than noise, so it is lifted.
pub const MAX_OUTPUTS_UNTIL_HEIGHT: usize = 630_000;

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
/// carriers, not payments) or, below [`MAX_OUTPUTS_UNTIL_HEIGHT`], when it has
/// more than [`MAX_OUTPUTS`] outputs (batch payouts). `height` is the block these
/// outputs belong to. The mempool, always past the cap window, passes
/// `usize::MAX`.
#[inline]
pub fn for_each_round_dollar_bin(
    height: usize,
    outputs: impl ExactSizeIterator<Item = (Sats, OutputType)> + Clone,
    mut emit: impl FnMut(u16),
) {
    if height < MAX_OUTPUTS_UNTIL_HEIGHT && outputs.len() > MAX_OUTPUTS {
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
