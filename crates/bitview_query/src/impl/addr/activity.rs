use brk_error::{Error, Result};
use brk_types::{Addr, Height, OutputType, Txid, TypeIndex};

use crate::Query;

impl Query {
    /// Height of the last on-chain activity for an address (last tx_index to height).
    /// With `before_txid`, returns the newest activity strictly older than that
    /// cursor. Used by paginated chain etags so a new tx above the cursor
    /// doesn't invalidate deeper pages.
    pub fn addr_last_activity_height(
        &self,
        addr: &Addr,
        before_txid: Option<&Txid>,
    ) -> Result<Height> {
        let _guard = self.read_plugin(self.indexer())?;
        let (output_type, type_index) = self.resolve_addr(addr)?;
        self.addr_last_activity_height_for(output_type, type_index, before_txid)
    }

    pub fn addr_last_activity_height_for(
        &self,
        output_type: OutputType,
        type_index: TypeIndex,
        before_txid: Option<&Txid>,
    ) -> Result<Height> {
        let stores = self.indexer().stores();
        let tx_index_len = self.safe_lengths().tx_index;
        let before = before_txid
            .map(|txid| self.resolve_tx_index(txid))
            .transpose()?
            .unwrap_or(tx_index_len)
            .min(tx_index_len);
        let last_tx_index = stores
            .addr_tx_indexes_before(output_type, type_index, before)?
            .next_back()
            .ok_or(Error::UnknownAddr)?;
        self.confirmed_status_height(last_tx_index)
    }
}
