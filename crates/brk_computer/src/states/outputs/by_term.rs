#[derive(Default, Clone)]
pub struct OutputsByTerm<T> {
    pub short: T,
    pub long: T,
}

impl<T> OutputsByTerm<T> {
    pub fn mut_flatten(&mut self) -> Vec<&mut T> {
        vec![&mut self.short, &mut self.long]
    }
}
