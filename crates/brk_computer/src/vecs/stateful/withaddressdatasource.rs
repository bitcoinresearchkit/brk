use brk_core::{AddressData, EmptyAddressData};

#[derive(Debug)]
pub enum WithAddressDataSource<T> {
    New(T),
    FromAddressDataStore(T),
    FromEmptyAddressDataStore(T),
}

impl<T> WithAddressDataSource<T> {
    pub fn is_new(&self) -> bool {
        matches!(self, Self::New(_))
    }

    pub fn deref(&self) -> &T {
        match self {
            Self::New(v) => v,
            Self::FromAddressDataStore(v) => v,
            Self::FromEmptyAddressDataStore(v) => v,
        }
    }

    pub fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::New(v) => v,
            Self::FromAddressDataStore(v) => v,
            Self::FromEmptyAddressDataStore(v) => v,
        }
    }
}

impl From<WithAddressDataSource<EmptyAddressData>> for WithAddressDataSource<AddressData> {
    fn from(value: WithAddressDataSource<EmptyAddressData>) -> Self {
        match value {
            WithAddressDataSource::New(v) => Self::New(v.into()),
            WithAddressDataSource::FromAddressDataStore(v) => Self::FromAddressDataStore(v.into()),
            WithAddressDataSource::FromEmptyAddressDataStore(v) => {
                Self::FromEmptyAddressDataStore(v.into())
            }
        }
    }
}

impl From<WithAddressDataSource<AddressData>> for WithAddressDataSource<EmptyAddressData> {
    fn from(value: WithAddressDataSource<AddressData>) -> Self {
        match value {
            WithAddressDataSource::New(v) => Self::New(v.into()),
            WithAddressDataSource::FromAddressDataStore(v) => Self::FromAddressDataStore(v.into()),
            WithAddressDataSource::FromEmptyAddressDataStore(v) => {
                Self::FromEmptyAddressDataStore(v.into())
            }
        }
    }
}
