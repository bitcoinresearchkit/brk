use brk_core::Sats;

use super::OutputFilter;

#[derive(Default, Clone)]
pub struct OutputsByUpToSize<T> {
    pub _1_000sats: T,
    pub _10_000sats: T,
    pub _1btc: T,
    pub _10btc: T,
    pub _100btc: T,
}

impl<T> OutputsByUpToSize<T> {
    pub fn as_mut_vec(&mut self) -> [&mut T; 5] {
        [
            &mut self._1_000sats,
            &mut self._10_000sats,
            &mut self._1btc,
            &mut self._10btc,
            &mut self._100btc,
        ]
    }
}

impl<T> OutputsByUpToSize<(OutputFilter, T)> {
    pub fn vecs(&self) -> [&T; 5] {
        [
            &self._1_000sats.1,
            &self._10_000sats.1,
            &self._1btc.1,
            &self._10btc.1,
            &self._100btc.1,
        ]
    }
}

impl<T> From<OutputsByUpToSize<T>> for OutputsByUpToSize<(OutputFilter, T)> {
    fn from(value: OutputsByUpToSize<T>) -> Self {
        Self {
            _1_000sats: (OutputFilter::To(1_000), value._1_000sats),
            _10_000sats: (OutputFilter::To(10_000), value._10_000sats),
            _1btc: (OutputFilter::To(usize::from(Sats::ONE_BTC)), value._1btc),
            _10btc: (
                OutputFilter::To(usize::from(10 * Sats::ONE_BTC)),
                value._10btc,
            ),
            _100btc: (
                OutputFilter::To(usize::from(100 * Sats::ONE_BTC)),
                value._100btc,
            ),
        }
    }
}
