//! Address data wrapper that tracks its source for flush operations.

use brk_types::{EmptyAddressData, EmptyAddressIndex, LoadedAddressData, LoadedAddressIndex};

/// Address data wrapped with its source location for flush operations.
///
/// This enum tracks where the data came from so it can be correctly
/// updated or created during the flush phase.
#[derive(Debug)]
pub enum WithAddressDataSource<T> {
    /// Brand new address (never seen before)
    New(T),
    /// Loaded from loaded address storage (with original index)
    FromLoaded(LoadedAddressIndex, T),
    /// Loaded from empty address storage (with original index)
    FromEmpty(EmptyAddressIndex, T),
}

impl<T> WithAddressDataSource<T> {
    pub fn is_new(&self) -> bool {
        matches!(self, Self::New(_))
    }

    pub fn is_from_emptyaddressdata(&self) -> bool {
        matches!(self, Self::FromEmpty(..))
    }
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
