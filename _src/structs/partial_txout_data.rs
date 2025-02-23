use super::{Address, Amount};

#[derive(Debug)]
pub struct PartialTxoutData {
    pub amount: Amount,
    pub address: Option<Address>,
    pub address_index_opt: Option<u32>,
}

impl PartialTxoutData {
    pub fn new(address: Option<Address>, amount: Amount, address_index_opt: Option<u32>) -> Self {
        Self {
            address,
            amount,
            address_index_opt,
        }
    }
}
