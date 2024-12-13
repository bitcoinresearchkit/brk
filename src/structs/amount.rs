use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use allocative::{Allocative, Visitor};
use bincode::{
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};
use biter::bitcoin::Amount as BitcoinAmount;
use derive_deref::{Deref, DerefMut};
use sanakirja::{direct_repr, Storable, UnsizedStorable};
use serde::{Deserialize, Serialize};

use super::Height;

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Deref,
    DerefMut,
    Default,
    Serialize,
    Deserialize,
)]
pub struct Amount(BitcoinAmount);
direct_repr!(Amount);

impl Amount {
    pub const ZERO: Self = Self(BitcoinAmount::ZERO);
    pub const ONE_BTC_F32: f32 = 100_000_000.0;
    pub const ONE_BTC_F64: f64 = 100_000_000.0;

    #[inline(always)]
    pub fn wrap(amount: BitcoinAmount) -> Self {
        Self(amount)
    }

    #[inline(always)]
    pub fn from_sat(sats: u64) -> Self {
        Self(BitcoinAmount::from_sat(sats))
    }
}

impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Self::Output {
        Amount::from_sat(self.to_sat() + rhs.to_sat())
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        *self = Amount::from_sat(self.to_sat() + rhs.to_sat());
    }
}

impl Sub for Amount {
    type Output = Amount;

    fn sub(self, rhs: Amount) -> Self::Output {
        Amount::from_sat(self.to_sat() - rhs.to_sat())
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Amount::from_sat(self.to_sat() - rhs.to_sat());
    }
}

impl Mul<Amount> for Amount {
    type Output = Amount;

    fn mul(self, rhs: Amount) -> Self::Output {
        Amount::from_sat(self.to_sat() * rhs.to_sat())
    }
}

impl Mul<u64> for Amount {
    type Output = Amount;

    fn mul(self, rhs: u64) -> Self::Output {
        Amount::from_sat(self.to_sat() * rhs)
    }
}

impl Mul<Height> for Amount {
    type Output = Amount;

    fn mul(self, rhs: Height) -> Self::Output {
        Amount::from_sat(self.to_sat() * *rhs as u64)
    }
}

impl Sum for Amount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let sats = iter.map(|amt| amt.to_sat()).sum();
        Amount::from_sat(sats)
    }
}

impl Encode for Amount {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.to_sat(), encoder)
    }
}

impl Decode for Amount {
    fn decode<D: Decoder>(decoder: &mut D) -> core::result::Result<Self, DecodeError> {
        let sats: u64 = Decode::decode(decoder)?;

        Ok(Amount::from_sat(sats))
    }
}

impl<'de> BorrowDecode<'de> for Amount {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let sats: u64 = BorrowDecode::borrow_decode(decoder)?;

        Ok(Amount::from_sat(sats))
    }
}

impl Allocative for Amount {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        visitor.visit_simple_sized::<Self>();
    }
}
