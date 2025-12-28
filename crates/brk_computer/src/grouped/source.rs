use vecdb::IterableBoxedVec;

#[derive(Clone)]
pub enum Source<I, T> {
    Compute,
    Vec(IterableBoxedVec<I, T>),
}

impl<I, T> Source<I, T> {
    pub fn is_compute(&self) -> bool {
        matches!(self, Self::Compute)
    }

    pub fn is_vec(&self) -> bool {
        matches!(self, Self::Vec(_))
    }

    pub fn vec(self) -> Option<IterableBoxedVec<I, T>> {
        match self {
            Self::Vec(v) => Some(v),
            _ => None,
        }
    }
}

impl<I, T> From<IterableBoxedVec<I, T>> for Source<I, T> {
    #[inline]
    fn from(value: IterableBoxedVec<I, T>) -> Self {
        Self::Vec(value)
    }
}
