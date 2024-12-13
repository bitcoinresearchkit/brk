use super::{AddressData, Amount, Price, Timestamp};

#[derive(Debug)]
pub struct AddressRealizedData {
    pub initial_address_data: AddressData,
    pub received: Amount,
    pub sent: Amount,
    pub profit: Price,
    pub loss: Price,
    pub value_created: Price,
    pub adjusted_value_created: Price,
    pub value_destroyed: Price,
    pub adjusted_value_destroyed: Price,
    pub utxos_created: u32,
    pub utxos_destroyed: u32,
}

impl AddressRealizedData {
    pub fn default(initial_address_data: &AddressData) -> Self {
        Self {
            received: Amount::ZERO,
            sent: Amount::ZERO,
            profit: Price::ZERO,
            loss: Price::ZERO,
            utxos_created: 0,
            utxos_destroyed: 0,
            value_created: Price::ZERO,
            adjusted_value_created: Price::ZERO,
            value_destroyed: Price::ZERO,
            adjusted_value_destroyed: Price::ZERO,
            initial_address_data: *initial_address_data,
        }
    }

    pub fn receive(&mut self, amount: Amount) {
        self.received += amount;
        self.utxos_created += 1;
    }

    pub fn send(
        &mut self,
        amount: Amount,
        current_price: Price,
        previous_price: Price,
        current_timestamp: Timestamp,
        previous_timestamp: Timestamp,
    ) {
        self.sent += amount;

        self.utxos_destroyed += 1;

        let current_value = current_price * amount;
        let previous_value = previous_price * amount;

        self.value_created += current_value;
        self.value_destroyed += previous_value;

        if previous_timestamp.older_by_1h_plus_than(current_timestamp) {
            self.adjusted_value_created += current_value;
            self.adjusted_value_destroyed += previous_value;
        }

        if current_value >= previous_value {
            self.profit += current_value - previous_value;
        } else {
            self.loss += previous_value - current_value;
        }
    }
}
