use brk_types::Dollars;
use vecdb::BinaryTransform;

/// (Dollars, Dollars) -> Dollars: a² / b
pub struct DollarsSquaredDivide;

impl BinaryTransform<Dollars, Dollars, Dollars> for DollarsSquaredDivide {
    #[inline(always)]
    fn apply(a: Dollars, b: Dollars) -> Dollars {
        let af = f64::from(a);
        let bf = f64::from(b);
        if bf == 0.0 {
            Dollars::NAN
        } else {
            Dollars::from(af * af / bf)
        }
    }
}
