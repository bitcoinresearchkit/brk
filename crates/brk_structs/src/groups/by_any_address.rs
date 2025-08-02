#[derive(Debug, Default)]
pub struct ByAnyAddress<T> {
    pub loaded: T,
    pub empty: T,
}

impl<T> ByAnyAddress<Option<T>> {
    pub fn take(&mut self) {
        self.loaded.take();
        self.empty.take();
    }
}
