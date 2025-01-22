use derive_deref::Deref;

#[derive(Debug, Deref, Clone)]
pub struct Timestamp(jiff::Timestamp);

impl TryFrom<u32> for Timestamp {
    type Error = jiff::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self(jiff::Timestamp::from_second(value as i64)?))
    }
}
