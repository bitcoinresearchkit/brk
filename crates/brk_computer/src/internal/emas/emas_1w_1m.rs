//! Generic 2-slot container for 1w + 1m EMA pairs.

use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
pub struct Emas1w1m<A, B = A> {
    #[traversable(rename = "1w")]
    pub _1w: A,
    #[traversable(rename = "1m")]
    pub _1m: B,
}

impl<A> Emas1w1m<A> {
    pub const SUFFIXES: [&'static str; 2] = ["ema_1w", "ema_1m"];

    pub fn try_from_fn<E>(
        mut f: impl FnMut(&str) -> std::result::Result<A, E>,
    ) -> std::result::Result<Self, E> {
        Ok(Self {
            _1w: f(Self::SUFFIXES[0])?,
            _1m: f(Self::SUFFIXES[1])?,
        })
    }

    pub fn as_array(&self) -> [&A; 2] {
        [&self._1w, &self._1m]
    }

    pub fn as_mut_array(&mut self) -> [&mut A; 2] {
        [&mut self._1w, &mut self._1m]
    }
}
