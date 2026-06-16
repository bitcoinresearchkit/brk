use std::str::FromStr;

use brk_error::{Error, OptionData, Result};
use brk_types::{Addr, AddrBytes, AddrHash, OutputType, TypeIndex};

use crate::Query;

impl Query {
    pub(super) fn resolve_addr(&self, addr: &Addr) -> Result<(OutputType, TypeIndex)> {
        let bytes = AddrBytes::from_str(addr)?;
        let output_type = OutputType::from(&bytes);
        let hash = AddrHash::from(&bytes);
        let type_index = self.type_index_for(output_type, &hash)?;
        Ok((output_type, type_index))
    }

    /// Lookup the per-type index of an address by `(output_type, hash)`.
    /// Returns `UnknownAddr` if the hash is absent from the type's index.
    pub(super) fn type_index_for(
        &self,
        output_type: OutputType,
        hash: &AddrHash,
    ) -> Result<TypeIndex> {
        self.indexer()
            .stores
            .addr_type_to_addr_hash_to_addr_index
            .get(output_type)
            .data()?
            .get(hash)?
            .map(|cow| cow.into_owned())
            .ok_or(Error::UnknownAddr)
    }
}
