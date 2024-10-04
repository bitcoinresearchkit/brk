use crate::structs::Price;

#[derive(Debug, Default)]
pub struct RealizedState {
    realized_profit: Price,
    realized_loss: Price,
    value_created: Price,
    adjusted_value_created: Price,
    value_destroyed: Price,
    adjusted_value_destroyed: Price,
}

impl RealizedState {
    pub fn realized_profit(&self) -> Price {
        self.realized_profit
    }

    pub fn realized_loss(&self) -> Price {
        self.realized_loss
    }

    pub fn value_created(&self) -> Price {
        self.value_created
    }

    pub fn adjusted_value_created(&self) -> Price {
        self.adjusted_value_created
    }

    pub fn value_destroyed(&self) -> Price {
        self.value_destroyed
    }

    pub fn adjusted_value_destroyed(&self) -> Price {
        self.adjusted_value_destroyed
    }

    pub fn iterate(
        &mut self,
        realized_profit: Price,
        realized_loss: Price,
        value_created: Price,
        adjusted_value_created: Price,
        value_destroyed: Price,
        adjusted_value_destroyed: Price,
    ) {
        self.realized_profit += realized_profit;
        self.realized_loss += realized_loss;
        self.value_created += value_created;
        self.adjusted_value_created += adjusted_value_created;
        self.value_destroyed += value_destroyed;
        self.adjusted_value_destroyed += adjusted_value_destroyed;
    }
}
