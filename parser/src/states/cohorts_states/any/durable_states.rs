use std::ops::AddAssign;

use allocative::Allocative;

use crate::structs::{Amount, Price};

use super::{CapitalizationState, SupplyState, UTXOState};

#[derive(Default, Debug, Allocative)]
pub struct DurableStates {
    pub capitalization_state: CapitalizationState,
    pub supply_state: SupplyState,
    pub utxo_state: UTXOState,
}

impl DurableStates {
    pub fn increment(
        &mut self,
        amount: Amount,
        utxo_count: f64,
        realized_cap: Price,
    ) -> color_eyre::Result<()> {
        self.utxo_state.increment(utxo_count);
        self.capitalization_state.increment(realized_cap);
        self.supply_state.increment(amount);

        Ok(())
    }

    pub fn decrement(
        &mut self,
        amount: Amount,
        utxo_count: f64,
        realized_cap: Price,
    ) -> color_eyre::Result<()> {
        self.utxo_state.decrement(utxo_count)?;
        self.capitalization_state.decrement(realized_cap);
        self.supply_state.decrement(amount)?;

        Ok(())
    }
}

impl AddAssign for DurableStates {
    fn add_assign(&mut self, rhs: Self) {
        self.capitalization_state += rhs.capitalization_state;
        self.supply_state += rhs.supply_state;
        self.utxo_state += rhs.utxo_state;
    }
}
