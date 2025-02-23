use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{Cents, Close, High, Low, Open};

// #[derive(Debug, Default, Clone, Copy, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize)]
// #[repr(C)]
// pub struct OHLCCents(OHLC<Cents>);

#[derive(Debug, Default, Clone, Copy, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize)]
#[repr(C)]
pub struct OHLCCents(Open<Cents>, High<Cents>, Low<Cents>, Close<Cents>);

impl OHLCCents {
    pub fn open(&self) -> Open<Cents> {
        self.0
    }

    pub fn high(&self) -> High<Cents> {
        self.1
    }

    pub fn low(&self) -> Low<Cents> {
        self.2
    }

    pub fn close(&self) -> Close<Cents> {
        self.3
    }
}

impl From<(Open<Cents>, High<Cents>, Low<Cents>, Close<Cents>)> for OHLCCents {
    fn from(value: (Open<Cents>, High<Cents>, Low<Cents>, Close<Cents>)) -> Self {
        Self(value.0, value.1, value.2, value.3)
    }
}
