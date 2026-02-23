use std::path::Path;

use brk_error::Result;
use brk_types::{Age, Cents, FundedAddressData, Sats, SupplyState};
use vecdb::unlikely;

use super::{super::cost_basis::RealizedState, base::CohortState};

/// Significant digits for address cost basis prices (after rounding to dollars).
const COST_BASIS_PRICE_DIGITS: i32 = 4;

pub struct AddressCohortState {
    pub addr_count: u64,
    pub inner: CohortState,
}

impl AddressCohortState {
    pub(crate) fn new(path: &Path, name: &str) -> Self {
        Self {
            addr_count: 0,
            inner: CohortState::new(path, name)
                .with_price_rounding(COST_BASIS_PRICE_DIGITS),
        }
    }

    /// Reset state for fresh start.
    pub(crate) fn reset(&mut self) {
        self.addr_count = 0;
        self.inner.supply = SupplyState::default();
        self.inner.sent = Sats::ZERO;
        self.inner.satblocks_destroyed = Sats::ZERO;
        self.inner.satdays_destroyed = Sats::ZERO;
        self.inner.realized = RealizedState::default();
    }

    pub(crate) fn reset_cost_basis_data_if_needed(&mut self) -> Result<()> {
        self.inner.reset_cost_basis_data_if_needed()
    }

    pub(crate) fn send(
        &mut self,
        addressdata: &mut FundedAddressData,
        value: Sats,
        current_price: Cents,
        prev_price: Cents,
        ath: Cents,
        age: Age,
    ) -> Result<()> {
        let prev = addressdata.cost_basis_snapshot();
        addressdata.send(value, prev_price)?;
        let current = addressdata.cost_basis_snapshot();

        self.inner.send_address(
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
        address_data: &mut FundedAddressData,
        value: Sats,
        price: Cents,
        output_count: u32,
    ) {
        let prev = address_data.cost_basis_snapshot();
        address_data.receive_outputs(value, price, output_count);
        let current = address_data.cost_basis_snapshot();

        self.inner.receive_address(
            &SupplyState {
                utxo_count: output_count as u64,
                value,
            },
            price,
            &current,
            &prev,
        );
    }

    pub(crate) fn add(&mut self, addressdata: &FundedAddressData) {
        self.addr_count += 1;
        self.inner
            .increment_snapshot(&addressdata.cost_basis_snapshot());
    }

    pub(crate) fn subtract(&mut self, addressdata: &FundedAddressData) {
        let snapshot = addressdata.cost_basis_snapshot();

        // Check for potential underflow before it happens
        if unlikely(self.inner.supply.utxo_count < snapshot.supply_state.utxo_count) {
            panic!(
                "AddressCohortState::subtract underflow!\n\
                Cohort state: addr_count={}, supply={}\n\
                Address being subtracted: {}\n\
                Address supply: {}\n\
                Realized price: {}\n\
                This means the address is not properly tracked in this cohort.",
                self.addr_count,
                self.inner.supply,
                addressdata,
                snapshot.supply_state,
                snapshot.realized_price
            );
        }
        if unlikely(self.inner.supply.value < snapshot.supply_state.value) {
            panic!(
                "AddressCohortState::subtract value underflow!\n\
                Cohort state: addr_count={}, supply={}\n\
                Address being subtracted: {}\n\
                Address supply: {}\n\
                Realized price: {}\n\
                This means the address is not properly tracked in this cohort.",
                self.addr_count,
                self.inner.supply,
                addressdata,
                snapshot.supply_state,
                snapshot.realized_price
            );
        }

        self.addr_count = self.addr_count.checked_sub(1).unwrap_or_else(|| {
            panic!(
                "AddressCohortState::subtract addr_count underflow! addr_count=0\n\
                Address being subtracted: {}\n\
                Realized price: {}",
                addressdata, snapshot.realized_price
            )
        });

        self.inner.decrement_snapshot(&snapshot);
    }

}
