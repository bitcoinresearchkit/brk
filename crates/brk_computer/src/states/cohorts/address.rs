use std::path::Path;

use brk_error::Result;
use brk_types::{Dollars, Height, LoadedAddressData, Sats};

use crate::SupplyState;

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
        let compute_price = price.is_some();

        let prev_realized_price = compute_price.then(|| address_data.realized_price());
        let prev_supply_state = SupplyState {
            utxo_count: address_data.utxo_count() as u64,
            value: address_data.balance(),
        };

        address_data.receive(value, price);

        let supply_state = SupplyState {
            utxo_count: address_data.utxo_count() as u64,
            value: address_data.balance(),
        };

        self.inner.receive_(
            &SupplyState {
                utxo_count: 1,
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
        self.addr_count = self.addr_count.checked_sub(1).unwrap();
        self.inner.decrement_(
            &addressdata.into(),
            addressdata.realized_cap,
            addressdata.realized_price(),
        );
    }

    pub fn commit(&mut self, height: Height) -> Result<()> {
        self.inner.commit(height)
    }
}
