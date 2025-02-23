use super::{PricePaidState, UnrealizedState};

#[derive(Default)]
pub struct OneShotStates {
    pub price_paid_state: PricePaidState,

    pub unrealized_block_state: UnrealizedState,
    pub unrealized_date_state: Option<UnrealizedState>,
}
