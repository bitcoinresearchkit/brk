use bitview_plugin_mappings::Vecs as MappingVecs;

use bitview_traversable::Traversable;
use bitview_vecs::LazySpotValuePerBlock;
use brk_error::Result;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::ReadableCloneableVec;

use crate::DCA_DOLLARS_PER_DAY;

#[derive(Clone, derive_more::Deref, derive_more::DerefMut, Traversable)]
#[traversable(transparent)]
pub struct DcaStack(pub LazySpotValuePerBlock);

impl DcaStack {
    const COST_BASIS_NUMERATOR: f64 = DCA_DOLLARS_PER_DAY * 100.0 * Sats::ONE_BTC_U64 as f64;

    pub fn from_source<V>(
        name: &str,
        version: Version,
        mappings: &MappingVecs,
        source: &V,
        spot_price: &impl ReadableCloneableVec<Height, Cents>,
    ) -> Result<Self>
    where
        V: ReadableCloneableVec<Height, Sats> + ?Sized,
    {
        Ok(Self(LazySpotValuePerBlock::from_sats_source(
            name, version, source, mappings, spot_price,
        )))
    }

    #[inline(always)]
    pub fn cost_basis_cents(days: usize, sats: Sats) -> Cents {
        if sats == Sats::ZERO {
            return Cents::NAN;
        }

        let cents = (Self::COST_BASIS_NUMERATOR * days as f64 / f64::from(sats)).round();
        if cents >= u64::MAX as f64 {
            Cents::MAX_FINITE
        } else {
            Cents::from(cents as u64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DCA_AMOUNT;
    use brk_types::Bitcoin;

    #[test]
    fn cost_basis_cents_matches_typed_formula() {
        assert_eq!(DcaStack::cost_basis_cents(1, Sats::ZERO), Cents::NAN);
        assert_eq!(
            DcaStack::cost_basis_cents(1, Sats::ONE_BTC),
            Cents::from(10_000_u64),
        );
        assert_eq!(
            DcaStack::cost_basis_cents(usize::MAX, Sats::_1),
            Cents::MAX_FINITE,
        );

        for index in 0..1_200_000_u64 {
            let days = index as usize % 6_000 + 1;
            let sats =
                Sats::from(index.wrapping_mul(6_364_136_223_846_793_005) % 2_000_000_000 + 1);
            assert_eq!(
                DcaStack::cost_basis_cents(days, sats),
                Cents::from(DCA_AMOUNT * days / Bitcoin::from(sats)),
            );
        }
    }
}
