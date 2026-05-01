use crate::{Txid, TxidPrefix, Vout};

/// Compact `(TxidPrefix, Vout)` outpoint identifier. Prefix collisions
/// are possible and must be verified by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutpointPrefix(TxidPrefix, Vout);

impl OutpointPrefix {
    #[inline]
    pub fn new(txid_prefix: TxidPrefix, vout: Vout) -> Self {
        Self(txid_prefix, vout)
    }

    #[inline]
    pub fn txid_prefix(self) -> TxidPrefix {
        self.0
    }

    #[inline]
    pub fn vout(self) -> Vout {
        self.1
    }
}

impl From<(TxidPrefix, Vout)> for OutpointPrefix {
    #[inline]
    fn from((txid_prefix, vout): (TxidPrefix, Vout)) -> Self {
        Self(txid_prefix, vout)
    }
}

impl From<(&Txid, Vout)> for OutpointPrefix {
    #[inline]
    fn from((txid, vout): (&Txid, Vout)) -> Self {
        Self(TxidPrefix::from(txid), vout)
    }
}

impl From<(Txid, Vout)> for OutpointPrefix {
    #[inline]
    fn from((txid, vout): (Txid, Vout)) -> Self {
        Self(TxidPrefix::from(&txid), vout)
    }
}
