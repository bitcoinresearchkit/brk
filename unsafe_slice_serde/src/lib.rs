use std::{fmt, slice};

pub trait UnsafeSliceSerde
where
    Self: Sized,
{
    const SIZE: usize = size_of::<Self>();

    fn unsafe_try_from_slice(slice: &[u8]) -> Result<&Self> {
        let (prefix, shorts, suffix) = unsafe { slice.align_to::<Self>() };

        if !prefix.is_empty() || shorts.len() != 1 || !suffix.is_empty() {
            // dbg!(&slice, &prefix, &shorts, &suffix);
            return Err(Error::FailedToAlignToSelf);
        }

        Ok(&shorts[0])
    }

    fn unsafe_as_slice(&self) -> &[u8] {
        let data: *const Self = self;
        let data: *const u8 = data as *const u8;
        unsafe { slice::from_raw_parts(data, Self::SIZE) }
    }
}
impl<T> UnsafeSliceSerde for T {}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    FailedToAlignToSelf,
}
impl fmt::Display for Error {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FailedToAlignToSelf => write!(f, "Failed to align_to for T"),
        }
    }
}
impl std::error::Error for Error {}
