use allocative::Allocative;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::structs::Counter;

use super::AnyState;

#[derive(Default, Debug, Encode, Decode, Serialize, Deserialize, Allocative)]
pub struct Counters {
    pub multisig_addresses: Counter,
    pub op_return_addresses: Counter,
    pub push_only_addresses: Counter,
    pub unknown_addresses: Counter,
    pub empty_addresses: Counter,
}

impl Counters {}

impl AnyState for Counters {
    fn name<'a>() -> &'a str {
        "counters"
    }

    fn clear(&mut self) {
        self.multisig_addresses.reset();
        self.push_only_addresses.reset();
        self.unknown_addresses.reset();
        self.empty_addresses.reset();
    }
}
