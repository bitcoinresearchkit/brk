use super::OutputFilter;

#[derive(Default, Clone)]
pub struct OutputsByTerm<T> {
    pub short: T,
    pub long: T,
}

impl<T> OutputsByTerm<T> {
    pub fn as_mut_vec(&mut self) -> [&mut T; 2] {
        [&mut self.short, &mut self.long]
    }
}

impl<T> OutputsByTerm<(OutputFilter, T)> {
    pub fn vecs(&self) -> [&T; 2] {
        [&self.short.1, &self.long.1]
    }
}

impl<T> From<OutputsByTerm<T>> for OutputsByTerm<(OutputFilter, T)> {
    fn from(value: OutputsByTerm<T>) -> Self {
        Self {
            long: (OutputFilter::From(150), value.long),
            short: (OutputFilter::To(150), value.short),
        }
    }
}
