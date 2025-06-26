use std::path::Path;

use brk_core::{Dollars, Height, Result};

use crate::{SupplyState, UnrealizedState};

pub trait CohortStateTrait: Sized {
    fn default_and_import(path: &Path, name: &str, compute_dollars: bool) -> Result<Self>;
    fn reset_single_iteration_values(&mut self);
    fn increment(&mut self, supply_state: &SupplyState, price: Option<Dollars>);
    fn decrement(&mut self, supply_state: &SupplyState, price: Option<Dollars>);
    fn decrement_price_to_amount(&mut self, supply_state: &SupplyState, price: Dollars);
    fn receive(&mut self, supply_state: &SupplyState, price: Option<Dollars>);
    fn send(
        &mut self,
        supply_state: &SupplyState,
        current_price: Option<Dollars>,
        prev_price: Option<Dollars>,
        blocks_old: usize,
        days_old: f64,
        older_than_hour: bool,
    );
    fn compute_unrealized_states(
        &self,
        height_price: Dollars,
        date_price: Option<Dollars>,
    ) -> (UnrealizedState, Option<UnrealizedState>);
    fn commit(&mut self, height: Height) -> Result<()>;
}
