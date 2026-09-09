use std::result::Result;

use crate::Windows;

#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

const WINDOW_FROM_1W_COUNT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WindowFrom1wId {
    Week1,
    Month1,
    Year1,
}

const WINDOW_FROM_1W_IDS: [WindowFrom1wId; WINDOW_FROM_1W_COUNT] = [
    WindowFrom1wId::Week1,
    WindowFrom1wId::Month1,
    WindowFrom1wId::Year1,
];

impl WindowFrom1wId {
    pub const fn index(self) -> usize {
        self as usize
    }
    pub const ALL: &'static [Self] = &WINDOW_FROM_1W_IDS;
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Week1 => "1w",
            Self::Month1 => "1m",
            Self::Year1 => "1y",
        }
    }

    pub fn series<T>(mut create: impl FnMut(Self) -> T) -> WindowsFrom1w<T> {
        WindowsFrom1w {
            _1w: create(Self::Week1),
            _1m: create(Self::Month1),
            _1y: create(Self::Year1),
        }
    }

    pub fn select<T>(self, windows: &WindowsFrom1w<T>) -> &T {
        match self {
            Self::Week1 => &windows._1w,
            Self::Month1 => &windows._1m,
            Self::Year1 => &windows._1y,
        }
    }

    pub fn select_full<T>(self, windows: &Windows<T>) -> &T {
        match self {
            Self::Week1 => &windows._1w,
            Self::Month1 => &windows._1m,
            Self::Year1 => &windows._1y,
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct WindowsFrom1w<A> {
    /// Uses a trailing 7-day window.
    pub _1w: A,
    /// Uses a trailing 30-day window.
    pub _1m: A,
    /// Uses a trailing 365-day window.
    pub _1y: A,
}

impl<A> WindowsFrom1w<A> {
    pub const SUFFIXES: [&'static str; 3] = ["1w", "1m", "1y"];

    pub fn try_from_fn<E>(mut f: impl FnMut(&str) -> Result<A, E>) -> Result<Self, E> {
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

#[cfg(all(test, feature = "storage"))]
mod tests {

    use super::{WINDOW_FROM_1W_IDS, WindowFrom1wId};

    #[test]
    fn window_from_1w_ids_match_named_fields() {
        assert_eq!(WindowFrom1wId::ALL, WINDOW_FROM_1W_IDS);

        let windows = WindowFrom1wId::series(|window| window);
        assert_eq!(windows._1w, WindowFrom1wId::Week1);
        assert_eq!(windows._1m, WindowFrom1wId::Month1);
        assert_eq!(windows._1y, WindowFrom1wId::Year1);
    }
}
