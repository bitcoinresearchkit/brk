use std::ops::AddAssign;

use allocative::Allocative;

use crate::{
    states::{DurableStates, IsZero, OneShotStates, PriceToValue, UnrealizedState},
    structs::{Amount, Price},
};

#[derive(Default, Debug, Allocative)]
pub struct AddressCohortDurableStates {
    pub address_count: f64,
    pub durable_states: DurableStates,
    pub price_to_amount: PriceToValue<Amount>,
}

impl AddressCohortDurableStates {
    #[allow(clippy::too_many_arguments)]
    pub fn increment(
        &mut self,
        address_count: f64,
        amount: Amount,
        utxo_count: f64,
        realized_cap: Price,
        mean_price_paid: Price,
    ) -> color_eyre::Result<()> {
        self.address_count += address_count;

        self._crement(amount, utxo_count, realized_cap, mean_price_paid, true)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn decrement(
        &mut self,
        address_count: f64,
        amount: Amount,
        utxo_count: f64,
        realized_cap: Price,
        mean_price_paid: Price,
    ) -> color_eyre::Result<()> {
        self.address_count -= address_count;

        self._crement(amount, utxo_count, realized_cap, mean_price_paid, false)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn _crement(
        &mut self,
        amount: Amount,
        utxo_count: f64,
        realized_cap: Price,
        mean_price_paid: Price,
        increment: bool,
    ) -> color_eyre::Result<()> {
        if increment {
            self.durable_states
                .increment(amount, utxo_count, realized_cap)
        } else {
            self.durable_states
                .decrement(amount, utxo_count, realized_cap)
        }
        .inspect_err(|report| {
            dbg!(report);
        })?;

        if !amount.is_zero()? {
            if increment {
                self.price_to_amount.increment(mean_price_paid, amount);
            } else {
                self.price_to_amount
                    .decrement(mean_price_paid, amount)
                    .inspect_err(|report| {
                        dbg!(report, "cents_to_split_amount decrement",);
                    })?;
            }
        }

        Ok(())
    }

    pub fn compute_one_shot_states(
        &self,
        block_price: Price,
        date_price: Option<Price>,
    ) -> OneShotStates {
        let mut one_shot_states = OneShotStates::default();

        if date_price.is_some() {
            one_shot_states
                .unrealized_date_state
                .replace(UnrealizedState::default());
        }

        let one_shot_states_ref = &mut one_shot_states;

        let supply = self.durable_states.supply_state.supply;

        self.price_to_amount.iterate(supply, |price_paid, amount| {
            one_shot_states_ref
                .price_paid_state
                .iterate(price_paid, amount, supply);
            one_shot_states_ref
                .unrealized_block_state
                .iterate(price_paid, block_price, amount);
            if let Some(unrealized_date_state) = one_shot_states_ref.unrealized_date_state.as_mut()
            {
                unrealized_date_state.iterate(price_paid, date_price.unwrap(), amount);
            }
        });

        one_shot_states
    }
}

impl AddAssign for AddressCohortDurableStates {
    fn add_assign(&mut self, rhs: Self) {
        self.address_count += rhs.address_count;
        self.durable_states += rhs.durable_states;
        self.price_to_amount += rhs.price_to_amount;
    }
}
