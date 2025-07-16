#[derive(Debug, Default)]
pub struct ByAnyAddress<T> {
    pub loaded: T,
    pub empty: T,
}
