use std::marker::PhantomData;

use brk_types::Cents;
use vecdb::{BinaryTransform, unlikely};

pub struct RatioCents<P>(PhantomData<P>);

impl<P: From<f64> + Default> BinaryTransform<Cents, Cents, P> for RatioCents<P> {
    #[inline(always)]
    fn apply(numerator: Cents, denominator: Cents) -> P {
        let denominator = f64::from(denominator);
        if unlikely(denominator == 0.0) {
            P::default()
        } else {
            P::from(f64::from(numerator) / denominator)
        }
    }
}
