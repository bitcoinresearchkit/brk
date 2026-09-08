#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

use crate::{Amount, CohortContext, Filter, UTXOGroups};

/// UTXO groups plus groups defined by the controlling address's balance.
///
/// These are independent views, not a cross-product. `A` can own the
/// address-balance storage when its UTXO counterpart is stored separately.
#[derive(Clone)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct UTXOAndAddrGroups<T: Clone, A = Amount<T>> {
    #[cfg_attr(feature = "storage", traversable(flatten))]
    pub utxo: UTXOGroups<T>,
    /// Groups addresses by their balance, not individual output value.
    pub addr_balance: A,
}

impl<T: Clone> UTXOAndAddrGroups<T> {
    pub fn map_named<U: Clone>(
        &self,
        mut map: impl FnMut(CohortContext, &Filter, &'static str, &T) -> U,
    ) -> UTXOAndAddrGroups<U> {
        UTXOAndAddrGroups {
            utxo: self
                .utxo
                .map_named(|filter, name, value| map(CohortContext::Utxo, filter, name, value)),
            addr_balance: self
                .addr_balance
                .map_named(|filter, name, value| map(CohortContext::Addr, filter, name, value)),
        }
    }
}
