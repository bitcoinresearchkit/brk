use crate::{Dollars, Sats};

#[derive(Debug, Default)]
pub struct AddressData {
    pub sent: Sats,
    pub received: Sats,
    pub realized_cap: Dollars,
    pub outputs_len: u32,
}

impl AddressData {
    pub fn amount(&self) -> Sats {
        (u64::from(self.received) - u64::from(self.sent)).into()
    }
}
