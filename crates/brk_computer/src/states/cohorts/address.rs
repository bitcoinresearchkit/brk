use std::path::Path;

use brk_core::{AddressData, Dollars, Height, Result, Sats};

use crate::SupplyState;

use super::CohortState;

#[derive(Clone)]
pub struct AddressCohortState {
    pub address_count: usize,
    pub inner: CohortState,
}

impl AddressCohortState {
    pub fn default_and_import(path: &Path, name: &str, compute_dollars: bool) -> Result<Self> {
        Ok(Self {
            address_count: 0,
            inner: CohortState::default_and_import(path, name, compute_dollars)?,
        })
    }

    pub fn height(&self) -> Option<Height> {
        self.inner.height()
    }

    pub fn reset_price_to_amount(&mut self) -> Result<()> {
        self.inner.reset_price_to_amount()
    }

    pub fn reset_single_iteration_values(&mut self) {
        self.inner.reset_single_iteration_values();
    }

    #[allow(clippy::too_many_arguments)]
    pub fn send(
        &mut self,
        addressdata: &mut AddressData,
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
            utxos: addressdata.outputs_len as usize,
            value: addressdata.amount(),
        };

        addressdata.send(value, prev_price)?;

        let supply_state = SupplyState {
            utxos: addressdata.outputs_len as usize,
            value: addressdata.amount(),
        };

        self.inner.send_(
            &SupplyState { utxos: 1, value },
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

    pub fn receive(&mut self, address_data: &mut AddressData, value: Sats, price: Option<Dollars>) {
        let compute_price = price.is_some();

        let prev_realized_price = compute_price.then(|| address_data.realized_price());
        let prev_supply_state = SupplyState {
            utxos: address_data.outputs_len as usize,
            value: address_data.amount(),
        };

        address_data.receive(value, price);

        let supply_state = SupplyState {
            utxos: address_data.outputs_len as usize,
            value: address_data.amount(),
        };

        self.inner.receive_(
            &SupplyState { utxos: 1, value },
            price,
            compute_price.then(|| (address_data.realized_price(), &supply_state)),
            prev_realized_price.map(|prev_price| (prev_price, &prev_supply_state)),
        );
    }

    pub fn add(&mut self, addressdata: &AddressData) {
        self.address_count += 1;
        self.inner.increment_(
            &addressdata.into(),
            addressdata.realized_cap,
            addressdata.realized_price(),
        );
    }

    pub fn subtract(&mut self, addressdata: &AddressData) {
        self.address_count = self.address_count.checked_sub(1).unwrap();
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
