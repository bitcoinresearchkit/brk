use brk_core::Sats;

use super::OutputFilter;

#[derive(Default, Clone)]
pub struct OutputsByFromSize<T> {
    pub _1_000sats: T,
    pub _1btc: T,
    pub _10btc: T,
    pub _100btc: T,
}

impl<T> OutputsByFromSize<T> {
    pub fn as_mut_vec(&mut self) -> [&mut T; 4] {
        [
            &mut self._1_000sats,
            &mut self._1btc,
            &mut self._10btc,
            &mut self._100btc,
        ]
    }
}

impl<T> OutputsByFromSize<(OutputFilter, T)> {
    pub fn vecs(&self) -> [&T; 4] {
        [
            &self._1_000sats.1,
            &self._1btc.1,
            &self._10btc.1,
            &self._100btc.1,
        ]
    }
}

impl<T> From<OutputsByFromSize<T>> for OutputsByFromSize<(OutputFilter, T)> {
    fn from(value: OutputsByFromSize<T>) -> Self {
        Self {
            _1_000sats: (OutputFilter::From(1_000), value._1_000sats),
            _1btc: (OutputFilter::From(usize::from(Sats::ONE_BTC)), value._1btc),
            _10btc: (
                OutputFilter::From(usize::from(10 * Sats::ONE_BTC)),
                value._10btc,
            ),
            _100btc: (
                OutputFilter::From(usize::from(100 * Sats::ONE_BTC)),
                value._100btc,
            ),
        }
    }
}
