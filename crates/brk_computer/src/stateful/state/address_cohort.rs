use std::path::Path;

use brk_error::Result;
use brk_types::{Dollars, Height, LoadedAddressData, Sats, SupplyState};
use vecdb::unlikely;

use crate::stateful::state::RealizedState;

use super::CohortState;

#[derive(Clone)]
pub struct AddressCohortState {
    pub addr_count: u64,
    pub inner: CohortState,
}

impl AddressCohortState {
    pub fn new(path: &Path, name: &str, compute_dollars: bool) -> Self {
        Self {
            addr_count: 0,
            inner: CohortState::new(path, name, compute_dollars),
        }
    }

    /// Reset state for fresh start.
    pub fn reset(&mut self) {
        self.addr_count = 0;
        self.inner.supply = SupplyState::default();
        self.inner.sent = Sats::ZERO;
        self.inner.satblocks_destroyed = Sats::ZERO;
        self.inner.satdays_destroyed = Sats::ZERO;
        if let Some(realized) = self.inner.realized.as_mut() {
            *realized = RealizedState::NAN;
        }
    }

    pub fn reset_price_to_amount_if_needed(&mut self) -> Result<()> {
        self.inner.reset_price_to_amount_if_needed()
    }

    pub fn reset_single_iteration_values(&mut self) {
        self.inner.reset_single_iteration_values();
    }

    #[allow(clippy::too_many_arguments)]
    pub fn send(
        &mut self,
        addressdata: &mut LoadedAddressData,
        value: Sats,
        current_price: Option<Dollars>,
        prev_price: Option<Dollars>,
        blocks_old: usize,
        days_old: f64,
        older_than_hour: bool,
    ) -> Result<()> {
        let compute_price = current_price.is_some();

        let prev_realized_price = compute_price.then(|| addressdata.realized_price());
        let prev_supply_state = SupplyState {
            utxo_count: addressdata.utxo_count() as u64,
            value: addressdata.balance(),
        };

        addressdata.send(value, prev_price)?;

        let supply_state = SupplyState {
            utxo_count: addressdata.utxo_count() as u64,
            value: addressdata.balance(),
        };

        self.inner.send_(
            &SupplyState {
                utxo_count: 1,
                value,
            },
            current_price,
            prev_price,
            blocks_old,
            days_old,
            older_than_hour,
            compute_price.then(|| (addressdata.realized_price(), &supply_state)),
            prev_realized_price.map(|prev_price| (prev_price, &prev_supply_state)),
        );

        Ok(())
    }

    pub fn receive(
        &mut self,
        address_data: &mut LoadedAddressData,
        value: Sats,
        price: Option<Dollars>,
    ) {
        self.receive_outputs(address_data, value, price, 1);
    }

    pub fn receive_outputs(
        &mut self,
        address_data: &mut LoadedAddressData,
        value: Sats,
        price: Option<Dollars>,
        output_count: u32,
    ) {
        let compute_price = price.is_some();

        let prev_realized_price = compute_price.then(|| address_data.realized_price());
        let prev_supply_state = SupplyState {
            utxo_count: address_data.utxo_count() as u64,
            value: address_data.balance(),
        };

        address_data.receive_outputs(value, price, output_count);

        let supply_state = SupplyState {
            utxo_count: address_data.utxo_count() as u64,
            value: address_data.balance(),
        };

        self.inner.receive_(
            &SupplyState {
                utxo_count: output_count as u64,
                value,
            },
            price,
            compute_price.then(|| (address_data.realized_price(), &supply_state)),
            prev_realized_price.map(|prev_price| (prev_price, &prev_supply_state)),
        );
    }

    pub fn add(&mut self, addressdata: &LoadedAddressData) {
        self.addr_count += 1;
        self.inner.increment_(
            &addressdata.into(),
            addressdata.realized_cap,
            addressdata.realized_price(),
        );
    }

    pub fn subtract(&mut self, addressdata: &LoadedAddressData) {
        let addr_supply: SupplyState = addressdata.into();
        let realized_price = addressdata.realized_price();

        // Check for potential underflow before it happens
        if unlikely(self.inner.supply.utxo_count < addr_supply.utxo_count) {
            panic!(
                "AddressCohortState::subtract underflow!\n\
                Cohort state: addr_count={}, supply={}\n\
                Address being subtracted: {}\n\
                Address supply: {}\n\
                Realized price: {}\n\
                This means the address is not properly tracked in this cohort.",
                self.addr_count, self.inner.supply, addressdata, addr_supply, realized_price
            );
        }
        if unlikely(self.inner.supply.value < addr_supply.value) {
            panic!(
                "AddressCohortState::subtract value underflow!\n\
                Cohort state: addr_count={}, supply={}\n\
                Address being subtracted: {}\n\
                Address supply: {}\n\
                Realized price: {}\n\
                This means the address is not properly tracked in this cohort.",
                self.addr_count, self.inner.supply, addressdata, addr_supply, realized_price
            );
        }

        self.addr_count = self.addr_count.checked_sub(1).unwrap_or_else(|| {
            panic!(
                "AddressCohortState::subtract addr_count underflow! addr_count=0\n\
                Address being subtracted: {}\n\
                Realized price: {}",
                addressdata, realized_price
            )
        });

        self.inner
            .decrement_(&addr_supply, addressdata.realized_cap, realized_price);
    }

    pub fn write(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.inner.write(height, cleanup)
    }
}
