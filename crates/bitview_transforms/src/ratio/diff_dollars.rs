use std::marker::PhantomData;

use brk_types::Dollars;
use vecdb::BinaryTransform;

pub struct RatioDiffDollars<P>(PhantomData<P>);

impl<P: From<f64> + Default> BinaryTransform<Dollars, Dollars, P> for RatioDiffDollars<P> {
    #[inline(always)]
    fn apply(close: Dollars, base: Dollars) -> P {
        let base = f64::from(base);
        if base == 0.0 {
            P::default()
        } else {
            P::from(f64::from(close) / base - 1.0)
        }
    }
}
