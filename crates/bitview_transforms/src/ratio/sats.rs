use std::marker::PhantomData;

use brk_types::Sats;
use vecdb::BinaryTransform;

pub struct RatioSats<P>(PhantomData<P>);

impl<P: From<f64> + Default> BinaryTransform<Sats, Sats, P> for RatioSats<P> {
    #[inline(always)]
    fn apply(numerator: Sats, denominator: Sats) -> P {
        if *denominator > 0 {
            P::from(*numerator as f64 / *denominator as f64)
        } else {
            P::default()
        }
    }
}
