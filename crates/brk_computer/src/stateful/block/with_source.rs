use brk_types::{EmptyAddressData, EmptyAddressIndex, LoadedAddressData, LoadedAddressIndex, TxIndex};
use smallvec::SmallVec;

/// Loaded address data with source tracking for flush operations.
pub type LoadedAddressDataWithSource = WithAddressDataSource<LoadedAddressData>;

/// Empty address data with source tracking for flush operations.
pub type EmptyAddressDataWithSource = WithAddressDataSource<EmptyAddressData>;

/// SmallVec for transaction indexes - most addresses have few transactions per block.
pub type TxIndexVec = SmallVec<[TxIndex; 4]>;

/// Address data wrapped with its source location for flush operations.
///
/// This enum tracks where the data came from so it can be correctly
/// updated or created during the flush phase.
#[derive(Debug, Clone)]
pub enum WithAddressDataSource<T> {
    /// Brand new address (never seen before)
    New(T),
    /// Loaded from loaded address storage (with original index)
    FromLoaded(LoadedAddressIndex, T),
    /// Loaded from empty address storage (with original index)
    FromEmpty(EmptyAddressIndex, T),
}

impl<T> std::ops::Deref for WithAddressDataSource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::New(v) | Self::FromLoaded(_, v) | Self::FromEmpty(_, v) => v,
        }
    }
}

impl<T> std::ops::DerefMut for WithAddressDataSource<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::New(v) | Self::FromLoaded(_, v) | Self::FromEmpty(_, v) => v,
        }
    }
}

impl From<WithAddressDataSource<EmptyAddressData>> for WithAddressDataSource<LoadedAddressData> {
    #[inline]
    fn from(value: WithAddressDataSource<EmptyAddressData>) -> Self {
        match value {
            WithAddressDataSource::New(v) => Self::New(v.into()),
            WithAddressDataSource::FromLoaded(i, v) => Self::FromLoaded(i, v.into()),
            WithAddressDataSource::FromEmpty(i, v) => Self::FromEmpty(i, v.into()),
        }
    }
}

impl From<WithAddressDataSource<LoadedAddressData>> for WithAddressDataSource<EmptyAddressData> {
    #[inline]
    fn from(value: WithAddressDataSource<LoadedAddressData>) -> Self {
        match value {
            WithAddressDataSource::New(v) => Self::New(v.into()),
            WithAddressDataSource::FromLoaded(i, v) => Self::FromLoaded(i, v.into()),
            WithAddressDataSource::FromEmpty(i, v) => Self::FromEmpty(i, v.into()),
        }
    }
}
