use brk_core::Dollars;

#[derive(Debug, Default)]
pub struct RealizedState {
    realized_profit: Dollars,
    realized_loss: Dollars,
    value_created: Dollars,
    adjusted_value_created: Dollars,
    value_destroyed: Dollars,
    adjusted_value_destroyed: Dollars,
}
