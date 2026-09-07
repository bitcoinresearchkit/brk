#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

#[derive(Debug, Default)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct ByAnyAddr<T> {
    pub funded: T,
    pub empty: T,
}

impl<T> ByAnyAddr<Option<T>> {
    pub fn take(&mut self) {
        self.funded.take();
        self.empty.take();
    }
}
