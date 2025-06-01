use std::{cmp::Ordering, path::Path};

use brk_core::{CheckedSub, Dollars, Height, Result, Sats, Version};
use brk_store::Store;
use fjall::TransactionalKeyspace;

use crate::UnrealizedState;

use super::{RealizedState, SupplyState};

#[derive(Clone)]
pub struct CohortState {
    pub supply: SupplyState,
    pub realized: Option<RealizedState>,
    pub price_to_amount: Store<Dollars, Sats>,
}

impl CohortState {
    pub fn default_and_import(
        keyspace: &TransactionalKeyspace,
        path: &Path,
        name: &str,
        version: Version,
        compute_dollars: bool,
    ) -> Result<Self> {
        Ok(Self {
            supply: SupplyState::default(),
            realized: compute_dollars.then_some(RealizedState::NAN),
            price_to_amount: Store::import(
                keyspace,
                path,
                &format!("{name}_price_to_amount"),
                version + Version::new(3),
                Some(None),
            )?,
        })
    }

    pub fn reset_single_iteration_values(&mut self) {
        if let Some(realized) = self.realized.as_mut() {
            realized.reset_single_iteration_values();
        }
    }

    pub fn increment(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.supply += supply_state;
        if let Some(realized) = self.realized.as_mut() {
            let price = price.unwrap();
            realized.increment(supply_state, price);
            *self.price_to_amount.puts_entry_or_default(&price) += supply_state.value;
        }
    }

    pub fn decrement(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.supply -= supply_state;
        if let Some(realized) = self.realized.as_mut() {
            let price = price.unwrap();
            realized.decrement(supply_state, price);
            *self.price_to_amount.puts_entry_or_default(&price) -= supply_state.value;
        }
    }

    pub fn receive(&mut self, supply_state: &SupplyState, price: Option<Dollars>) {
        self.supply += supply_state;
        if let Some(realized) = self.realized.as_mut() {
            let price = price.unwrap();
            realized.receive(supply_state, price);
            *self.price_to_amount.puts_entry_or_default(&price) += supply_state.value;
        }
    }

    pub fn send(
        &mut self,
        supply_state: &SupplyState,
        current_price: Option<Dollars>,
        prev_price: Option<Dollars>,
        older_than_hour: bool,
    ) {
        self.supply -= supply_state;
        if let Some(realized) = self.realized.as_mut() {
            let current_price = current_price.unwrap();
            let prev_price = prev_price.unwrap();
            realized.send(supply_state, current_price, prev_price, older_than_hour);
            *self.price_to_amount.puts_entry_or_default(&prev_price) -= supply_state.value;
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

        self.price_to_amount
            .puts_iter()
            .for_each(|(&price, &sats)| {
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
        self.price_to_amount
            .retain_or_del(|_, sats| *sats > Sats::ZERO);
        let price_to_amount_puts = self.price_to_amount.clone_puts();
        self.price_to_amount.commit(height)?;
        self.price_to_amount.append_puts(price_to_amount_puts);
        Ok(())
    }
}
