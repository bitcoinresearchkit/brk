use std::marker::PhantomData;

use brk_types::Dollars;
use vecdb::BinaryTransform;

pub struct RatioDollars<P>(PhantomData<P>);

impl<P: From<f64> + Default> BinaryTransform<Dollars, Dollars, P> for RatioDollars<P> {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> P {
        let ratio = f64::from(numerator) / f64::from(denominator);
        if ratio.is_finite() {
            P::from(ratio)
        } else {
            P::default()
        }
    }
}
