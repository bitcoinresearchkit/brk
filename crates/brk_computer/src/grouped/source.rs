use brk_vecs::BoxedAnyIterableVec;

#[derive(Clone)]
pub enum Source<I, T> {
    Compute,
    None,
    Vec(BoxedAnyIterableVec<I, T>),
}

impl<I, T> Source<I, T> {
    pub fn is_compute(&self) -> bool {
        matches!(self, Self::Compute)
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_vec(&self) -> bool {
        matches!(self, Self::Vec(_))
    }

    pub fn vec(self) -> Option<BoxedAnyIterableVec<I, T>> {
        match self {
            Self::Vec(v) => Some(v),
            _ => None,
        }
    }
}

impl<I, T> From<bool> for Source<I, T> {
    fn from(value: bool) -> Self {
        if value { Self::Compute } else { Self::None }
    }
}

impl<I, T> From<BoxedAnyIterableVec<I, T>> for Source<I, T> {
    fn from(value: BoxedAnyIterableVec<I, T>) -> Self {
        Self::Vec(value)
    }
}

impl<I, T> From<Option<BoxedAnyIterableVec<I, T>>> for Source<I, T> {
    fn from(value: Option<BoxedAnyIterableVec<I, T>>) -> Self {
        if let Some(v) = value {
            Self::Vec(v)
        } else {
            Self::None
        }
    }
}
