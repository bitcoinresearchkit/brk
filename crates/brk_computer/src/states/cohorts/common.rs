use std::{cmp::Ordering, path::Path};

use brk_core::{CheckedSub, Dollars, Height, Result, Sats};

use crate::{PriceToAmount, RealizedState, SupplyState, UnrealizedState};

#[derive(Clone)]
pub struct CohortState {
    pub supply: SupplyState,
    pub realized: Option<RealizedState>,
    pub satblocks_destroyed: Sats,
    pub satdays_destroyed: Sats,

    price_to_amount: PriceToAmount,
}

impl CohortState {
    pub fn default_and_import(path: &Path, name: &str, compute_dollars: bool) -> Result<Self> {
        Ok(Self {
            supply: SupplyState::default(),
            realized: compute_dollars.then_some(RealizedState::NAN),
            satblocks_destroyed: Sats::ZERO,
            satdays_destroyed: Sats::ZERO,
            price_to_amount: PriceToAmount::forced_import(path, name),
        })
    }

    pub fn height(&self) -> Option<Height> {
        self.price_to_amount.height()
    }

    pub fn reset_price_to_amount(&mut self) -> Result<()> {
        self.price_to_amount.reset()
    }

    pub fn price_to_amount_first_key_value(&self) -> Option<(&Dollars, &Sats)> {
        self.price_to_amount.first_key_value()
    }

    pub fn price_to_amount_last_key_value(&self) -> Option<(&Dollars, &Sats)> {
        self.price_to_amount.last_key_value()
    }

    pub fn reset_single_iteration_values(&mut self) {
        self.satdays_destroyed = Sats::ZERO;
        self.satblocks_destroyed = Sats::ZERO;
        if let Some(realized) = self.realized.as_mut() {
            realized.reset_single_iteration_values();
        }
    }

    pub fn increment(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.supply += supply_state;

        if supply_state.value > Sats::ZERO {
            if let Some(realized) = self.realized.as_mut() {
                let price = price.unwrap();
                realized.increment(supply_state, price);
                self.price_to_amount.increment(price, supply_state);
            }
        }
    }

    pub fn increment_(
        &mut self,
        supply_state: &SupplyState,
        realized_cap: Dollars,
        realized_price: Dollars,
    ) {
        self.supply += supply_state;

        if supply_state.value > Sats::ZERO {
            if let Some(realized) = self.realized.as_mut() {
                realized.increment_(realized_cap);
                self.price_to_amount.increment(realized_price, supply_state);
            }
        }
    }

    pub fn decrement(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.supply -= supply_state;

        if supply_state.value > Sats::ZERO {
            if let Some(realized) = self.realized.as_mut() {
                let price = price.unwrap();
                realized.decrement(supply_state, price);
                self.price_to_amount.decrement(price, supply_state);
            }
        }
    }

    pub fn decrement_(
        &mut self,
        supply_state: &SupplyState,
        realized_cap: Dollars,
        realized_price: Dollars,
    ) {
        self.supply -= supply_state;

        if supply_state.value > Sats::ZERO {
            if let Some(realized) = self.realized.as_mut() {
                realized.decrement_(realized_cap);
                self.price_to_amount.decrement(realized_price, supply_state);
            }
        }
    }

    pub fn receive(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.receive_(
            supply_state,
            price,
            price.map(|price| (price, supply_state)),
            None,
        );
    }

    pub fn receive_(
        &mut self,
        supply_state: &SupplyState,
        price: Option<Dollars>,
        price_to_amount_increment: Option<(Dollars, &SupplyState)>,
        price_to_amount_decrement: Option<(Dollars, &SupplyState)>,
    ) {
        self.supply += supply_state;

        if supply_state.value > Sats::ZERO {
            if let Some(realized) = self.realized.as_mut() {
                let price = price.unwrap();
                realized.receive(supply_state, price);

                if let Some((price, supply)) = price_to_amount_increment
                    && supply.value.is_not_zero()
                {
                    self.price_to_amount.increment(price, supply);
                }
                if let Some((price, supply)) = price_to_amount_decrement
                    && supply.value.is_not_zero()
                {
                    self.price_to_amount.decrement(price, supply);
                }
            }
        }
    }

    pub fn send(
        &mut self,
        supply_state: &SupplyState,
        current_price: Option<Dollars>,
        prev_price: Option<Dollars>,
        blocks_old: usize,
        days_old: f64,
        older_than_hour: bool,
    ) {
        self.send_(
            supply_state,
            current_price,
            prev_price,
            blocks_old,
            days_old,
            older_than_hour,
            None,
            prev_price.map(|prev_price| (prev_price, supply_state)),
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn send_(
        &mut self,
        supply_state: &SupplyState,
        current_price: Option<Dollars>,
        prev_price: Option<Dollars>,
        blocks_old: usize,
        days_old: f64,
        older_than_hour: bool,
        price_to_amount_increment: Option<(Dollars, &SupplyState)>,
        price_to_amount_decrement: Option<(Dollars, &SupplyState)>,
    ) {
        if supply_state.utxos == 0 {
            return;
        }

        self.supply -= supply_state;

        if supply_state.value > Sats::ZERO {
            self.satblocks_destroyed += supply_state.value * blocks_old;

            self.satdays_destroyed +=
                Sats::from((u64::from(supply_state.value) as f64 * days_old).floor() as u64);

            if let Some(realized) = self.realized.as_mut() {
                let current_price = current_price.unwrap();
                let prev_price = prev_price.unwrap();
                realized.send(supply_state, current_price, prev_price, older_than_hour);
                if let Some((price, supply)) = price_to_amount_increment
                    && supply.value.is_not_zero()
                {
                    self.price_to_amount.increment(price, supply);
                }
                if let Some((price, supply)) = price_to_amount_decrement
                    && supply.value.is_not_zero()
                {
                    self.price_to_amount.decrement(price, supply);
                }
            }
        }
    }

    pub fn compute_unrealized_states(
        &self,
        height_price: Dollars,
        date_price: Option<Dollars>,
    ) -> (UnrealizedState, Option<UnrealizedState>) {
        if self.price_to_amount.is_empty() {
            return (
                UnrealizedState::NAN,
                date_price.map(|_| UnrealizedState::NAN),
            );
        }

        let mut height_unrealized_state = UnrealizedState::ZERO;
        let mut date_unrealized_state = date_price.map(|_| UnrealizedState::ZERO);

        let update_state =
            |price: Dollars, current_price: Dollars, sats: Sats, state: &mut UnrealizedState| {
                match price.cmp(&current_price) {
                    Ordering::Equal => {
                        state.supply_even += sats;
                    }
                    Ordering::Less => {
                        state.supply_in_profit += sats;
                        if price > Dollars::ZERO && current_price > Dollars::ZERO {
                            let diff = current_price.checked_sub(price).unwrap();
                            if diff <= Dollars::ZERO {
                                dbg!(price, current_price, diff, sats);
                                panic!();
                            }
                            state.unrealized_profit += diff * sats;
                        }
                    }
                    Ordering::Greater => {
                        state.supply_in_loss += sats;
                        if price > Dollars::ZERO && current_price > Dollars::ZERO {
                            let diff = price.checked_sub(current_price).unwrap();
                            if diff <= Dollars::ZERO {
                                dbg!(price, current_price, diff, sats);
                                panic!();
                            }
                            state.unrealized_loss += diff * sats;
                        }
                    }
                }
            };

        self.price_to_amount.iter().for_each(|(&price, &sats)| {
            update_state(price, height_price, sats, &mut height_unrealized_state);

            if let Some(date_price) = date_price {
                update_state(
                    price,
                    date_price,
                    sats,
                    date_unrealized_state.as_mut().unwrap(),
                )
            }
        });

        (height_unrealized_state, date_unrealized_state)
    }

    pub fn commit(&mut self, height: Height) -> Result<()> {
        self.price_to_amount.flush(height)
    }
}
