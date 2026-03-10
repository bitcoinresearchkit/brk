use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
pub struct WindowsFrom1w<A> {
    pub _1w: A,
    pub _1m: A,
    pub _1y: A,
}

impl<A> WindowsFrom1w<A> {
    pub const SUFFIXES: [&'static str; 3] = ["1w", "1m", "1y"];

    pub fn try_from_fn<E>(
        mut f: impl FnMut(&str) -> std::result::Result<A, E>,
    ) -> std::result::Result<Self, E> {
        Ok(Self {
            _1w: f(Self::SUFFIXES[0])?,
            _1m: f(Self::SUFFIXES[1])?,
            _1y: f(Self::SUFFIXES[2])?,
        })
    }

    pub fn as_array(&self) -> [&A; 3] {
        [&self._1w, &self._1m, &self._1y]
    }

    pub fn as_mut_array(&mut self) -> [&mut A; 3] {
        [&mut self._1w, &mut self._1m, &mut self._1y]
    }
}
