use brk_core::{EmptyAddressData, LoadedAddressData};

#[derive(Debug)]
pub enum WithAddressDataSource<T> {
    New(T),
    FromLoadedAddressDataVec(T),
    FromEmptyAddressDataVec(T),
}

impl<T> WithAddressDataSource<T> {
    pub fn is_new(&self) -> bool {
        matches!(self, Self::New(_))
    }

    pub fn is_from_addressdata(&self) -> bool {
        matches!(self, Self::FromLoadedAddressDataVec(_))
    }

    pub fn is_from_emptyaddressdata(&self) -> bool {
        matches!(self, Self::FromEmptyAddressDataVec(_))
    }

    pub fn deref(&self) -> &T {
        match self {
            Self::New(v) => v,
            Self::FromLoadedAddressDataVec(v) => v,
            Self::FromEmptyAddressDataVec(v) => v,
        }
    }

    pub fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::New(v) => v,
            Self::FromLoadedAddressDataVec(v) => v,
            Self::FromEmptyAddressDataVec(v) => v,
        }
    }
}

impl From<WithAddressDataSource<EmptyAddressData>> for WithAddressDataSource<LoadedAddressData> {
    fn from(value: WithAddressDataSource<EmptyAddressData>) -> Self {
        match value {
            WithAddressDataSource::New(v) => Self::New(v.into()),
            WithAddressDataSource::FromLoadedAddressDataVec(v) => {
                Self::FromLoadedAddressDataVec(v.into())
            }
            WithAddressDataSource::FromEmptyAddressDataVec(v) => {
                Self::FromEmptyAddressDataVec(v.into())
            }
        }
    }
}

impl From<WithAddressDataSource<LoadedAddressData>> for WithAddressDataSource<EmptyAddressData> {
    fn from(value: WithAddressDataSource<LoadedAddressData>) -> Self {
        match value {
            WithAddressDataSource::New(v) => Self::New(v.into()),
            WithAddressDataSource::FromLoadedAddressDataVec(v) => {
                Self::FromLoadedAddressDataVec(v.into())
            }
            WithAddressDataSource::FromEmptyAddressDataVec(v) => {
                Self::FromEmptyAddressDataVec(v.into())
            }
        }
    }
}
