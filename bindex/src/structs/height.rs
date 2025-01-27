use std::{
    fmt, fs, io,
    ops::{Add, AddAssign, Rem, Sub},
    path::Path,
};

use biter::rpc::{self, RpcApi};
use derive_deref::{Deref, DerefMut};
use snkrj::{direct_repr, Storable, UnsizedStorable};
use storable_vec::UnsafeSizedSerDe;

#[derive(Debug, Clone, Copy, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Height(u32);
direct_repr!(Height);

impl Height {
    pub fn write(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, self.unsafe_as_slice())
    }
}

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

impl Rem<usize> for Height {
    type Output = Height;
    fn rem(self, rhs: usize) -> Self::Output {
        Self(self.abs_diff(Height::from(rhs).0))
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", **self)
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

impl TryFrom<&Path> for Height {
    type Error = color_eyre::Report;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Ok(Self::unsafe_try_from_slice(fs::read(value)?.as_slice())?.to_owned())
    }
}

impl TryFrom<&rpc::Client> for Height {
    type Error = rpc::Error;
    fn try_from(value: &rpc::Client) -> Result<Self, Self::Error> {
        Ok((value.get_blockchain_info()?.blocks as usize - 1).into())
    }
}

impl TryFrom<fjall::Slice> for Height {
    type Error = storable_vec::Error;
    fn try_from(value: fjall::Slice) -> Result<Self, Self::Error> {
        Ok(*Self::unsafe_try_from_slice(&value)?)
    }
}
impl From<Height> for fjall::Slice {
    fn from(value: Height) -> Self {
        Self::new(value.unsafe_as_slice())
    }
}
