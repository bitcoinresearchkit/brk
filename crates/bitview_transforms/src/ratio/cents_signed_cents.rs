use std::marker::PhantomData;

use brk_types::{Cents, CentsSigned};
use vecdb::{BinaryTransform, unlikely};

pub struct RatioCentsSignedCents<P>(PhantomData<P>);

impl<P: From<f64> + Default> BinaryTransform<CentsSigned, Cents, P> for RatioCentsSignedCents<P> {
    #[inline(always)]
    fn apply(numerator: CentsSigned, denominator: Cents) -> P {
        let denominator = f64::from(denominator);
        if unlikely(denominator == 0.0) {
            P::default()
        } else {
            P::from(numerator.inner() as f64 / denominator)
        }
    }
}
