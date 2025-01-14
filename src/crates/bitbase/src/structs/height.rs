use std::{
    fmt,
    ops::{Add, AddAssign, Sub},
};

use biter::bitcoincore_rpc::{self, RpcApi};
use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use super::SliceExtended;

#[derive(Debug, Clone, Copy, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Height(u32);

impl PartialEq<u64> for Height {
    fn eq(&self, other: &u64) -> bool {
        **self == *other as u32
    }
}

impl Add<u32> for Height {
    type Output = Height;

    fn add(self, rhs: u32) -> Self::Output {
        Self::from(*self + rhs)
    }
}

impl Add<usize> for Height {
    type Output = Height;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(*self + rhs as u32)
    }
}

impl Sub<Height> for Height {
    type Output = Height;

    fn sub(self, rhs: Height) -> Self::Output {
        Self::from(*self - *rhs)
    }
}

impl Sub<i32> for Height {
    type Output = Height;
    fn sub(self, rhs: i32) -> Self::Output {
        Self::from(*self - rhs as u32)
    }
}

impl Sub<u32> for Height {
    type Output = Height;
    fn sub(self, rhs: u32) -> Self::Output {
        Self::from(*self - rhs)
    }
}

impl Sub<usize> for Height {
    type Output = Height;
    fn sub(self, rhs: usize) -> Self::Output {
        Self::from(*self - rhs as u32)
    }
}

impl AddAssign<usize> for Height {
    fn add_assign(&mut self, rhs: usize) {
        *self = self.add(rhs);
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", **self)
    }
}

impl TryFrom<Slice> for Height {
    type Error = color_eyre::Report;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Ok(Self::from((&value[..]).read_be_u32()?))
    }
}
impl From<Height> for Slice {
    fn from(value: Height) -> Self {
        value.to_be_bytes().into()
    }
}

impl From<u32> for Height {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<usize> for Height {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<Height> for usize {
    fn from(value: Height) -> Self {
        value.0 as usize
    }
}

impl TryFrom<&bitcoincore_rpc::Client> for Height {
    type Error = bitcoincore_rpc::Error;
    fn try_from(value: &bitcoincore_rpc::Client) -> Result<Self, Self::Error> {
        Ok((value.get_blockchain_info()?.blocks as usize - 1).into())
    }
}

// impl Bytes for Height {
//     const SIZE: usize = size_of::<Self>();

//     type ByteArray = [u8; Self::SIZE];

//     // fn try_from_bytes(bytes: &[u8]) -> color_eyre::Result<Self> {
//     //     Ok(Self(Self::read_u32(bytes)))
//     // }

//     fn to_bytes(&self) -> Self::ByteArray {
//         self.to_ne_bytes()
//     }
// }
