use brk_types::{EmptyAddressData, EmptyAddressIndex, LoadedAddressData, LoadedAddressIndex};

#[derive(Debug)]
pub enum WithAddressDataSource<T> {
    New(T),
    FromLoadedAddressDataVec((LoadedAddressIndex, T)),
    FromEmptyAddressDataVec((EmptyAddressIndex, T)),
}

impl<T> WithAddressDataSource<T> {
    pub fn is_new(&self) -> bool {
        matches!(self, Self::New(_))
    }

    pub fn is_from_emptyaddressdata(&self) -> bool {
        matches!(self, Self::FromEmptyAddressDataVec(_))
    }

    pub fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::New(v) => v,
            Self::FromLoadedAddressDataVec((_, v)) => v,
            Self::FromEmptyAddressDataVec((_, v)) => v,
        }
    }
}

impl From<WithAddressDataSource<EmptyAddressData>> for WithAddressDataSource<LoadedAddressData> {
    #[inline]
    fn from(value: WithAddressDataSource<EmptyAddressData>) -> Self {
        match value {
            WithAddressDataSource::New(v) => Self::New(v.into()),
            WithAddressDataSource::FromLoadedAddressDataVec((i, v)) => {
                Self::FromLoadedAddressDataVec((i, v.into()))
            }
            WithAddressDataSource::FromEmptyAddressDataVec((i, v)) => {
                Self::FromEmptyAddressDataVec((i, v.into()))
            }
        }
    }
}

impl From<WithAddressDataSource<LoadedAddressData>> for WithAddressDataSource<EmptyAddressData> {
    #[inline]
    fn from(value: WithAddressDataSource<LoadedAddressData>) -> Self {
        match value {
            WithAddressDataSource::New(v) => Self::New(v.into()),
            WithAddressDataSource::FromLoadedAddressDataVec((i, v)) => {
                Self::FromLoadedAddressDataVec((i, v.into()))
            }
            WithAddressDataSource::FromEmptyAddressDataVec((i, v)) => {
                Self::FromEmptyAddressDataVec((i, v.into()))
            }
        }
    }
}
