/// Writer-owned last row. A length change reloads the persisted checkpoint.
pub(crate) struct CumulativeState<R> {
    last: Option<(usize, R)>,
}

impl<R> Default for CumulativeState<R> {
    fn default() -> Self {
        Self { last: None }
    }
}

impl<R: Clone + Default> CumulativeState<R> {
    pub fn accumulate(
        &mut self,
        len: usize,
        load: impl FnOnce() -> Option<R>,
        add: impl FnOnce(&mut R),
    ) -> R {
        let mut row = match self.last.take() {
            Some((cached_len, row)) if cached_len == len => row,
            _ => load().unwrap_or_default(),
        };
        add(&mut row);
        self.last = Some((len + 1, row.clone()));
        row
    }
}
