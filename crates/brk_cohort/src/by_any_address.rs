use brk_traversable::Traversable;

#[derive(Debug, Default, Traversable)]
pub struct ByAnyAddress<T> {
    pub funded: T,
    pub empty: T,
}

impl<T> ByAnyAddress<Option<T>> {
    pub fn take(&mut self) {
        self.funded.take();
        self.empty.take();
    }
}
