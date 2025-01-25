use std::fmt::Debug;

use sanakirja::Storable;

pub trait AnyDatabase {
    fn export(self) -> Result<(), sanakirja::Error>;
    // fn destroy(self) -> io::Result<()>;
}

pub trait DatabaseKey
where
    Self: Ord + Clone + Debug + Storable + Send + Sync,
{
    const SIZE: usize = size_of::<Self>();
    const SIZE_SMALLER_THAN_TWO: bool = Self::SIZE < 2;

    fn as_ne_byte(&self) -> u8 {
        let data: *const Self = self;
        let data: *const u8 = data as *const u8;
        let slice = unsafe { std::slice::from_raw_parts(data, Self::SIZE) };

        *(if cfg!(target_endian = "big") {
            slice.last()
        } else {
            slice.first()
        })
        .unwrap()
    }

    fn as_ne_six_bits(&self) -> u8 {
        self.as_ne_byte() >> 2
    }

    fn as_ne_two_bytes(&self) -> [u8; 2] {
        let data: *const Self = self;
        let data: *const u8 = data as *const u8;
        let slice = unsafe { std::slice::from_raw_parts(data, Self::SIZE) };

        if Self::SIZE_SMALLER_THAN_TWO {
            panic!("Doesn't make sense")
        }

        if cfg!(target_endian = "big") {
            let mut iter = slice.iter().rev();
            [*iter.next().unwrap(), *iter.next().unwrap()]
        } else {
            let mut iter = slice.iter();
            [*iter.next().unwrap(), *iter.next().unwrap()]
        }
    }
}
impl<T> DatabaseKey for T where T: Ord + Clone + Debug + Storable + Send + Sync {}

pub trait DatabaseValue
where
    Self: Clone + Storable + PartialEq + Send + Sync,
{
}
impl<T> DatabaseValue for T where T: Clone + Storable + PartialEq + Send + Sync {}
