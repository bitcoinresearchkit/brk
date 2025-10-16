use derive_deref::Deref;

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vin(u16);

impl Vin {
    pub const ZERO: Self = Vin(0);
    pub const ONE: Self = Vin(1);

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

const U16_MAX_AS_U32: u32 = u16::MAX as u32;
impl From<u32> for Vin {
    fn from(value: u32) -> Self {
        if value > U16_MAX_AS_U32 {
            panic!()
        }
        Self(value as u16)
    }
}

const U16_MAX_AS_USIZE: usize = u16::MAX as usize;
impl From<usize> for Vin {
    fn from(value: usize) -> Self {
        if value > U16_MAX_AS_USIZE {
            panic!()
        }
        Self(value as u16)
    }
}

impl From<Vin> for u64 {
    fn from(value: Vin) -> Self {
        value.0 as u64
    }
}
