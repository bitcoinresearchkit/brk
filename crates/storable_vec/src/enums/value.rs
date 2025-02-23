use std::{fmt::Debug, ops::Deref};

#[derive(Debug, Clone)]
pub enum Value<'a, T> {
    Ref(&'a T),
    Owned(T),
}

impl<T> Value<'_, T>
where
    T: Sized + Debug + Clone,
{
    pub fn into_inner(self) -> T {
        match self {
            Self::Ref(t) => t.to_owned(),
            Self::Owned(t) => t,
        }
    }
}
impl<T> Deref for Value<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(t) => t,
            Self::Owned(t) => t,
        }
    }
}
impl<T> AsRef<T> for Value<'_, T>
where
    T: Sized + Debug + Clone,
{
    fn as_ref(&self) -> &T {
        match self {
            Self::Ref(t) => t,
            Self::Owned(t) => t,
        }
    }
}
