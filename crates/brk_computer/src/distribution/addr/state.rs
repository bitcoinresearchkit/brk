use brk_types::{FundedAddrData, Height, OutputType, Sats};

use crate::distribution::{block::TrackingStatus, vecs::AddrMetricsVecs};

use super::{
    AddrTypeToActivityCounts, AddrTypeToAddrCount, ExposedAddrState, ReusedAddrState,
};

/// Bundle of per-block runtime state for the full address-metrics pipeline.
/// Feeds `process_received` / `process_sent` and is pushed to [`AddrMetricsVecs`]
/// once per block.
///
/// Recovery: [`From<(&AddrMetricsVecs, Height)>`] reads the prior block from
/// disk to seed all persistent running totals. Per-block counters (activity,
/// and event counts inside each [`ReusedAddrState`]) default to zero and are
/// cleared at the top of each block via [`Self::reset_per_block`].
#[derive(Debug, Default)]
pub struct AddrMetricsState {
    pub funded: AddrTypeToAddrCount,
    pub empty: AddrTypeToAddrCount,
    pub activity: AddrTypeToActivityCounts,
    pub reused: ReusedAddrState,
    pub respent: ReusedAddrState,
    pub exposed: ExposedAddrState,
}

/// Snapshot of [`FundedAddrData`] taken BEFORE a receive mutates it.
/// Feeds delta-based updates in [`AddrMetricsState::on_receive_applied`].
#[derive(Debug)]
pub struct AddrReceivePreState {
    pub was_funded: bool,
    pub was_reused: bool,
    pub was_respent: bool,
    pub was_pubkey_exposed: bool,
    pub prev_funded_txo_count: u32,
    pub exposed_contribution: Sats,
    pub reused_contribution: Sats,
    pub respent_contribution: Sats,
}

impl AddrReceivePreState {
    #[inline]
    pub fn capture(addr_data: &FundedAddrData, output_type: OutputType) -> Self {
        Self {
            was_funded: addr_data.is_funded(),
            was_reused: addr_data.is_reused(),
            was_respent: addr_data.is_respent(),
            was_pubkey_exposed: addr_data.is_pubkey_exposed(output_type),
            prev_funded_txo_count: addr_data.funded_txo_count,
            exposed_contribution: addr_data.exposed_supply_contribution(output_type),
            reused_contribution: addr_data.reused_supply_contribution(),
            respent_contribution: addr_data.respent_supply_contribution(),
        }
    }
}

/// Snapshot of [`FundedAddrData`] taken BEFORE a spend mutates it.
/// Feeds delta-based updates in [`AddrMetricsState::on_send_applied`].
#[derive(Debug)]
pub struct AddrSendPreState {
    pub was_reused: bool,
    pub was_respent: bool,
    pub was_pubkey_exposed: bool,
    pub exposed_contribution: Sats,
    pub reused_contribution: Sats,
    pub respent_contribution: Sats,
}

impl AddrSendPreState {
    #[inline]
    pub fn capture(addr_data: &FundedAddrData, output_type: OutputType) -> Self {
        Self {
            was_reused: addr_data.is_reused(),
            was_respent: addr_data.is_respent(),
            was_pubkey_exposed: addr_data.is_pubkey_exposed(output_type),
            exposed_contribution: addr_data.exposed_supply_contribution(output_type),
            reused_contribution: addr_data.reused_supply_contribution(),
            respent_contribution: addr_data.respent_supply_contribution(),
        }
    }
}

impl AddrMetricsState {
    #[inline]
    pub(crate) fn reset_per_block(&mut self) {
        self.activity.reset();
        self.reused.reset_per_block();
        self.respent.reset_per_block();
    }

    /// Apply all state updates for a received output, AFTER the cohort and
    /// `addr_data` have been mutated. `pre` is the snapshot captured before
    /// the mutation, `addr_data` is the post-receive view.
    #[inline]
    pub(crate) fn on_receive_applied(
        &mut self,
        output_type: OutputType,
        status: TrackingStatus,
        addr_data: &FundedAddrData,
        pre: &AddrReceivePreState,
        output_count: u32,
    ) {
        let activity = self.activity.get_mut_unwrap(output_type);
        activity.receiving += 1;
        match status {
            TrackingStatus::New => {
                *self.funded.get_mut_unwrap(output_type) += 1;
            }
            TrackingStatus::WasEmpty => {
                activity.reactivated += 1;
                *self.funded.get_mut_unwrap(output_type) += 1;
                *self.empty.get_mut_unwrap(output_type) -= 1;
            }
            TrackingStatus::Tracked => {}
        }
        self.reused
            .on_receive_as_reused(output_type, addr_data, pre, output_count);
        self.respent
            .on_receive_as_respent(output_type, addr_data, pre, output_count);
        self.exposed.on_receive(output_type, addr_data, pre, status);
    }

    /// Apply all state updates for a spent UTXO, AFTER the cohort and
    /// `addr_data` have been mutated. `pre` is the snapshot captured before
    /// the mutation. `is_first_encounter` / `also_received` come from the
    /// caller's per-block seen/received tracking. `will_be_empty` is from
    /// the pre-mutation `addr_data.has_1_utxos()`.
    #[inline]
    pub(crate) fn on_send_applied(
        &mut self,
        output_type: OutputType,
        addr_data: &FundedAddrData,
        pre: &AddrSendPreState,
        is_first_encounter: bool,
        also_received: bool,
        will_be_empty: bool,
    ) {
        if is_first_encounter {
            let activity = self.activity.get_mut_unwrap(output_type);
            activity.sending += 1;
            if also_received {
                activity.bidirectional += 1;
            }
        }
        if will_be_empty {
            *self.funded.get_mut_unwrap(output_type) -= 1;
            *self.empty.get_mut_unwrap(output_type) += 1;
        }
        self.reused.on_send_as_reused(
            output_type,
            addr_data,
            pre,
            is_first_encounter,
            also_received,
            will_be_empty,
        );
        self.respent.on_send_as_respent(
            output_type,
            addr_data,
            pre,
            is_first_encounter,
            also_received,
            will_be_empty,
        );
        self.exposed.on_send(output_type, addr_data, pre, will_be_empty);
    }
}

impl From<(&AddrMetricsVecs, Height)> for AddrMetricsState {
    #[inline]
    fn from((vecs, starting_height): (&AddrMetricsVecs, Height)) -> Self {
        Self {
            funded: AddrTypeToAddrCount::from((&vecs.funded, starting_height)),
            empty: AddrTypeToAddrCount::from((&vecs.empty, starting_height)),
            activity: AddrTypeToActivityCounts::default(),
            reused: ReusedAddrState::from((&vecs.reused, starting_height)),
            respent: ReusedAddrState::from((&vecs.respent, starting_height)),
            exposed: ExposedAddrState::from((&vecs.exposed, starting_height)),
        }
    }
}
