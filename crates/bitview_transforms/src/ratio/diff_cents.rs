use std::marker::PhantomData;

use brk_types::Cents;
use vecdb::BinaryTransform;

pub struct RatioDiffCents<P>(PhantomData<P>);

impl<P: From<f64> + Default> BinaryTransform<Cents, Cents, P> for RatioDiffCents<P> {
    #[inline(always)]
    fn apply(close: Cents, base: Cents) -> P {
        let base = f64::from(base);
        if base == 0.0 {
            P::default()
        } else {
            P::from(f64::from(close) / base - 1.0)
        }
    }
}
