use std::path::Path;

use brk_error::Result;
use brk_types::{Age, Cents, FundedAddrData, Sats, SupplyState};
use vecdb::unlikely;

use super::super::cost_basis::{CostBasisRaw, RealizedOps};
use super::base::CohortState;

/// Significant digits for address cost basis prices (after rounding to dollars).
const COST_BASIS_PRICE_DIGITS: i32 = 4;

pub struct AddrCohortState<R: RealizedOps> {
    pub addr_count: u64,
    pub inner: CohortState<R, CostBasisRaw>,
}

impl<R: RealizedOps> AddrCohortState<R> {
    pub(crate) fn new(path: &Path, name: &str) -> Self {
        Self {
            addr_count: 0,
            inner: CohortState::new(path, name).with_price_rounding(COST_BASIS_PRICE_DIGITS),
        }
    }

    /// Reset state for fresh start.
    pub(crate) fn reset(&mut self) {
        self.addr_count = 0;
        self.inner.supply = SupplyState::default();
        self.inner.sent = Sats::ZERO;
        self.inner.satdays_destroyed = Sats::ZERO;
        self.inner.realized = R::default();
    }

    pub(crate) fn send(
        &mut self,
        addr_data: &mut FundedAddrData,
        value: Sats,
        current_price: Cents,
        prev_price: Cents,
        ath: Cents,
        age: Age,
    ) -> Result<()> {
        let prev = addr_data.cost_basis_snapshot();
        addr_data.send(value, prev_price)?;
        let current = addr_data.cost_basis_snapshot();

        self.inner.send_addr(
            &SupplyState {
                utxo_count: 1,
                value,
            },
            current_price,
            prev_price,
            ath,
            age,
            &current,
            &prev,
        );

        Ok(())
    }

    pub(crate) fn receive_outputs(
        &mut self,
        addr_data: &mut FundedAddrData,
        value: Sats,
        price: Cents,
        output_count: u32,
    ) {
        let prev = addr_data.cost_basis_snapshot();
        addr_data.receive_outputs(value, price, output_count);
        let current = addr_data.cost_basis_snapshot();

        self.inner.receive_addr(
            &SupplyState {
                utxo_count: output_count as u64,
                value,
            },
            price,
            &current,
            &prev,
        );
    }

    pub(crate) fn add(&mut self, addr_data: &FundedAddrData) {
        self.addr_count += 1;
        self.inner
            .increment_snapshot(&addr_data.cost_basis_snapshot());
    }

    pub(crate) fn subtract(&mut self, addr_data: &FundedAddrData) {
        let snapshot = addr_data.cost_basis_snapshot();

        // Check for potential underflow before it happens
        if unlikely(self.inner.supply.utxo_count < snapshot.supply_state.utxo_count) {
            panic!(
                "AddrCohortState::subtract underflow!\n\
                Cohort state: addr_count={}, supply={}\n\
                Addr being subtracted: {}\n\
                Addr supply: {}\n\
                Realized price: {}\n\
                This means the addr is not properly tracked in this cohort.",
                self.addr_count,
                self.inner.supply,
                addr_data,
                snapshot.supply_state,
                snapshot.realized_price
            );
        }
        if unlikely(self.inner.supply.value < snapshot.supply_state.value) {
            panic!(
                "AddrCohortState::subtract value underflow!\n\
                Cohort state: addr_count={}, supply={}\n\
                Addr being subtracted: {}\n\
                Addr supply: {}\n\
                Realized price: {}\n\
                This means the addr is not properly tracked in this cohort.",
                self.addr_count,
                self.inner.supply,
                addr_data,
                snapshot.supply_state,
                snapshot.realized_price
            );
        }

        self.addr_count = self.addr_count.checked_sub(1).unwrap_or_else(|| {
            panic!(
                "AddrCohortState::subtract addr_count underflow! addr_count=0\n\
                Addr being subtracted: {}\n\
                Realized price: {}",
                addr_data, snapshot.realized_price
            )
        });

        self.inner.decrement_snapshot(&snapshot);
    }
}
