use std::str::FromStr;

use brk_error::{Error, Result};
use brk_types::{Addr, AddrBytes, AddrHash, OutputType, TypeIndex};

use crate::Query;

impl Query {
    pub(crate) fn missing_addr(&self) -> Error {
        if self.indexer().publication().try_read().is_none() {
            Error::StateUpdating
        } else {
            Error::UnknownAddr
        }
    }

    pub fn resolve_addr(&self, addr: &Addr) -> Result<(OutputType, TypeIndex)> {
        let bytes = AddrBytes::from_str(addr)?;
        self.resolve_addr_bytes(&bytes)
    }
    pub fn resolve_addr_bytes(&self, bytes: &AddrBytes) -> Result<(OutputType, TypeIndex)> {
        self.find_addr_bytes(bytes)?
            .ok_or_else(|| self.missing_addr())
    }
    pub fn find_addr_bytes(&self, bytes: &AddrBytes) -> Result<Option<(OutputType, TypeIndex)>> {
        let output_type = OutputType::from(bytes);
        let hash = AddrHash::from(bytes);
        Ok(self
            .indexer()
            .stores()
            .addr_index(output_type, &hash)?
            .filter(|type_index| *type_index < self.safe_lengths().to_type_index(output_type))
            .map(|type_index| (output_type, type_index)))
    }
    /// Lookup the per-type index of an address by `(output_type, hash)`.
    /// Returns `UnknownAddr` if the hash is absent from the type's index.
    pub fn type_index_for(&self, output_type: OutputType, hash: &AddrHash) -> Result<TypeIndex> {
        self.indexer()
            .stores()
            .addr_index(output_type, hash)?
            .ok_or(Error::UnknownAddr)
    }
}
