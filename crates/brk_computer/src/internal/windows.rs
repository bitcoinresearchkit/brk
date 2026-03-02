//! Base generic struct with 4 type parameters — one per rolling window duration.
//!
//! Foundation for all rolling window types (24h, 7d, 30d, 1y).

use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
pub struct Windows<A, B = A, C = A, D = A> {
    #[traversable(rename = "24h")]
    pub _24h: A,
    #[traversable(rename = "7d")]
    pub _7d: B,
    #[traversable(rename = "30d")]
    pub _30d: C,
    #[traversable(rename = "1y")]
    pub _1y: D,
}

impl<A> Windows<A> {
    pub const SUFFIXES: [&'static str; 4] = ["24h", "7d", "30d", "1y"];

    pub fn try_from_fn<E>(
        mut f: impl FnMut(&str) -> std::result::Result<A, E>,
    ) -> std::result::Result<Self, E> {
        Ok(Self {
            _24h: f(Self::SUFFIXES[0])?,
            _7d: f(Self::SUFFIXES[1])?,
            _30d: f(Self::SUFFIXES[2])?,
            _1y: f(Self::SUFFIXES[3])?,
        })
    }

    pub fn as_array(&self) -> [&A; 4] {
        [&self._24h, &self._7d, &self._30d, &self._1y]
    }

    pub fn as_mut_array(&mut self) -> [&mut A; 4] {
        [&mut self._24h, &mut self._7d, &mut self._30d, &mut self._1y]
    }
}
