use brk_types::{EmptyAddressData, EmptyAddressIndex, FundedAddressData, FundedAddressIndex};

/// Address data wrapped with its source location for flush operations.
///
/// This enum tracks where the data came from so it can be correctly
/// updated or created during the flush phase.
#[derive(Debug, Clone)]
pub enum WithAddressDataSource<T> {
    /// Brand new address (never seen before)
    New(T),
    /// Funded from funded address storage (with original index)
    FromFunded(FundedAddressIndex, T),
    /// Funded from empty address storage (with original index)
    FromEmpty(EmptyAddressIndex, T),
}

impl<T> std::ops::Deref for WithAddressDataSource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::New(v) | Self::FromFunded(_, v) | Self::FromEmpty(_, v) => v,
        }
    }
}

impl<T> std::ops::DerefMut for WithAddressDataSource<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::New(v) | Self::FromFunded(_, v) | Self::FromEmpty(_, v) => v,
        }
    }
}

impl From<WithAddressDataSource<EmptyAddressData>> for WithAddressDataSource<FundedAddressData> {
    #[inline]
    fn from(value: WithAddressDataSource<EmptyAddressData>) -> Self {
        match value {
            WithAddressDataSource::New(v) => Self::New(v.into()),
            WithAddressDataSource::FromFunded(i, v) => Self::FromFunded(i, v.into()),
            WithAddressDataSource::FromEmpty(i, v) => Self::FromEmpty(i, v.into()),
        }
    }
}

impl From<WithAddressDataSource<FundedAddressData>> for WithAddressDataSource<EmptyAddressData> {
    #[inline]
    fn from(value: WithAddressDataSource<FundedAddressData>) -> Self {
        match value {
            WithAddressDataSource::New(v) => Self::New(v.into()),
            WithAddressDataSource::FromFunded(i, v) => Self::FromFunded(i, v.into()),
            WithAddressDataSource::FromEmpty(i, v) => Self::FromEmpty(i, v.into()),
        }
    }
}
