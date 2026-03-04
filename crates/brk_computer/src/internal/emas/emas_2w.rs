//! Generic 1-slot container for 2w EMA.

use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
pub struct Emas2w<A> {
    #[traversable(rename = "2w")]
    pub _2w: A,
}

impl<A> Emas2w<A> {
    pub const SUFFIXES: [&'static str; 1] = ["ema_2w"];

    pub fn try_from_fn<E>(
        mut f: impl FnMut(&str) -> std::result::Result<A, E>,
    ) -> std::result::Result<Self, E> {
        Ok(Self {
            _2w: f(Self::SUFFIXES[0])?,
        })
    }

    pub fn as_array(&self) -> [&A; 1] {
        [&self._2w]
    }

    pub fn as_mut_array(&mut self) -> [&mut A; 1] {
        [&mut self._2w]
    }
}
