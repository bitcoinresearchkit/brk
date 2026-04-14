use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Indexes, OutputType, StoredF32, StoredU32, StoredU64, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, Rw, StorageMode, WritableVec};

use crate::{
    indexes, inputs,
    internal::{
        PerBlockCumulativeRolling, PerBlockRollingAverage, PercentCumulativeRolling,
        WindowStartVec, Windows, WithAddrTypes,
    },
    outputs,
};

use super::state::AddrTypeToReusedAddrEventCount;

/// Per-block reused-address event metrics. Holds three families of
/// signals: output-level (use), input-level (spend), and address-level
/// (active in block).
///
/// `output_to_reused_addr_count`: every output landing on an address that had
/// already received at least one prior output anywhere in its lifetime,
/// i.e. an output-level reuse event. Outputs are not deduplicated per
/// address within a block: an address receiving N outputs in one block
/// that had `before` lifetime outputs contributes
/// `max(0, N - max(0, 1 - before))` events. Only the very first output
/// an address ever sees is excluded. Every subsequent output counts,
/// matching the standard "% of outputs to previously-used addresses"
/// reuse ratio reported by external sources. `output_to_reused_addr_share`
/// uses `outputs::ByTypeVecs::output_count` (all 12 output types) as
/// denominator. `spendable_output_to_reused_addr_share` uses the
/// op_return-excluded 11-type aggregate (`spendable_output_count`).
///
/// `input_from_reused_addr_count`: every input spending from an address
/// whose lifetime `funded_txo_count > 1` at the time of the spend (i.e.
/// the address is in the same reused set tracked by
/// `reused_addr_count`). Every input is checked independently. If a
/// single address has multiple inputs in one block each one counts.
/// This is a *stable-predicate* signal about the sending address, not
/// an output-level repeat event: the first spend from a reused address
/// counts just as much as the tenth. Denominator
/// (`input_from_reused_addr_share`): `inputs::ByTypeVecs::input_count` (11
/// spendable types, where `p2ms`, `unknown`, `empty` count as true
/// negatives).
///
/// `active_reused_addr_count` / `active_reused_addr_share`: block-level
/// *address* signals (single aggregate, not per-type).
/// `active_reused_addr_count` is the count of distinct addresses
/// involved in this block (sent ∪ received) that satisfy `is_reused()`
/// after the block's events, populated inline in `process_received`
/// (each receiver, post-receive) and in `process_sent` (each
/// first-encounter sender, deduped against `received_addrs` so
/// addresses that did both aren't double-counted).
/// `active_reused_addr_share` is the per-block ratio
/// `reused / active * 100` as a percentage in `[0, 100]` (or `0.0` for
/// empty blocks). The denominator (distinct active addrs per block)
/// lives on `ActivityCountVecs::active` (`addrs.activity.all.active`),
/// derived from `sending + receiving - bidirectional`. Both fields
/// use `PerBlockRollingAverage` so their lazy 24h/1w/1m/1y series are
/// rolling *averages* of the per-block values. Sums and cumulatives of
/// distinct-address counts would be misleading because the same
/// address can appear in multiple blocks.
#[derive(Traversable)]
pub struct ReusedAddrEventsVecs<M: StorageMode = Rw> {
    pub output_to_reused_addr_count:
        WithAddrTypes<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
    pub output_to_reused_addr_share: WithAddrTypes<PercentCumulativeRolling<BasisPoints16, M>>,
    pub spendable_output_to_reused_addr_share: PercentCumulativeRolling<BasisPoints16, M>,
    pub input_from_reused_addr_count:
        WithAddrTypes<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
    pub input_from_reused_addr_share: WithAddrTypes<PercentCumulativeRolling<BasisPoints16, M>>,
    pub active_reused_addr_count: PerBlockRollingAverage<StoredU32, StoredU64, M>,
    pub active_reused_addr_share: PerBlockRollingAverage<StoredF32, StoredF32, M>,
}

impl ReusedAddrEventsVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let import_count = |name: &str| {
            WithAddrTypes::<PerBlockCumulativeRolling<StoredU64, StoredU64>>::forced_import(
                db,
                name,
                version,
                indexes,
                cached_starts,
            )
        };
        let import_percent = |name: &str| -> Result<WithAddrTypes<
            PercentCumulativeRolling<BasisPoints16>,
        >> {
            Ok(WithAddrTypes {
                all: PercentCumulativeRolling::forced_import(db, name, version, indexes)?,
                by_addr_type: ByAddrType::new_with_name(|type_name| {
                    PercentCumulativeRolling::forced_import(
                        db,
                        &format!("{type_name}_{name}"),
                        version,
                        indexes,
                    )
                })?,
            })
        };

        let output_to_reused_addr_count = import_count("output_to_reused_addr_count")?;
        let output_to_reused_addr_share = import_percent("output_to_reused_addr_share")?;
        let spendable_output_to_reused_addr_share = PercentCumulativeRolling::forced_import(
            db,
            "spendable_output_to_reused_addr_share",
            version,
            indexes,
        )?;
        let input_from_reused_addr_count = import_count("input_from_reused_addr_count")?;
        let input_from_reused_addr_share = import_percent("input_from_reused_addr_share")?;

        let active_reused_addr_count = PerBlockRollingAverage::forced_import(
            db,
            "active_reused_addr_count",
            version,
            indexes,
            cached_starts,
        )?;
        let active_reused_addr_share = PerBlockRollingAverage::forced_import(
            db,
            "active_reused_addr_share",
            version,
            indexes,
            cached_starts,
        )?;

        Ok(Self {
            output_to_reused_addr_count,
            output_to_reused_addr_share,
            spendable_output_to_reused_addr_share,
            input_from_reused_addr_count,
            input_from_reused_addr_share,
            active_reused_addr_count,
            active_reused_addr_share,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.output_to_reused_addr_count
            .min_stateful_len()
            .min(self.input_from_reused_addr_count.min_stateful_len())
            .min(self.active_reused_addr_count.block.len())
            .min(self.active_reused_addr_share.block.len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.output_to_reused_addr_count
            .par_iter_height_mut()
            .chain(self.input_from_reused_addr_count.par_iter_height_mut())
            .chain([
                &mut self.active_reused_addr_count.block as &mut dyn AnyStoredVec,
                &mut self.active_reused_addr_share.block as &mut dyn AnyStoredVec,
            ])
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.output_to_reused_addr_count.reset_height()?;
        self.input_from_reused_addr_count.reset_height()?;
        self.active_reused_addr_count.block.reset()?;
        self.active_reused_addr_share.block.reset()?;
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn push_height(
        &mut self,
        uses: &AddrTypeToReusedAddrEventCount,
        spends: &AddrTypeToReusedAddrEventCount,
        active_addr_count: u32,
        active_reused_addr_count: u32,
    ) {
        self.output_to_reused_addr_count
            .push_height(uses.sum(), uses.values().copied());
        self.input_from_reused_addr_count
            .push_height(spends.sum(), spends.values().copied());
        self.active_reused_addr_count
            .block
            .push(StoredU32::from(active_reused_addr_count));
        // Stored as a percentage in [0, 100] to match the rest of the
        // codebase (Unit.percentage on the website expects 0..100). The
        // `active_addr_count` denominator lives on `ActivityCountVecs`
        // (`addrs.activity.all.active`), passed in here so we can
        // compute the per-block ratio inline.
        let share = if active_addr_count > 0 {
            100.0 * (active_reused_addr_count as f32 / active_addr_count as f32)
        } else {
            0.0
        };
        self.active_reused_addr_share
            .block
            .push(StoredF32::from(share));
    }

    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        outputs_by_type: &outputs::ByTypeVecs,
        inputs_by_type: &inputs::ByTypeVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.output_to_reused_addr_count
            .compute_rest(starting_indexes.height, exit)?;
        self.input_from_reused_addr_count
            .compute_rest(starting_indexes.height, exit)?;
        self.active_reused_addr_count
            .compute_rest(starting_indexes.height, exit)?;
        self.active_reused_addr_share
            .compute_rest(starting_indexes.height, exit)?;

        self.output_to_reused_addr_share.all.compute_count_ratio(
            &self.output_to_reused_addr_count.all,
            &outputs_by_type.output_count.all,
            starting_indexes.height,
            exit,
        )?;
        self.spendable_output_to_reused_addr_share.compute_count_ratio(
            &self.output_to_reused_addr_count.all,
            &outputs_by_type.spendable_output_count,
            starting_indexes.height,
            exit,
        )?;
        self.input_from_reused_addr_share.all.compute_count_ratio(
            &self.input_from_reused_addr_count.all,
            &inputs_by_type.input_count.all,
            starting_indexes.height,
            exit,
        )?;
        for otype in OutputType::ADDR_TYPES {
            self.output_to_reused_addr_share
                .by_addr_type
                .get_mut_unwrap(otype)
                .compute_count_ratio(
                    self.output_to_reused_addr_count.by_addr_type.get_unwrap(otype),
                    outputs_by_type.output_count.by_type.get(otype),
                    starting_indexes.height,
                    exit,
                )?;
            self.input_from_reused_addr_share
                .by_addr_type
                .get_mut_unwrap(otype)
                .compute_count_ratio(
                    self.input_from_reused_addr_count.by_addr_type.get_unwrap(otype),
                    inputs_by_type.input_count.by_type.get(otype),
                    starting_indexes.height,
                    exit,
                )?;
        }
        Ok(())
    }
}
