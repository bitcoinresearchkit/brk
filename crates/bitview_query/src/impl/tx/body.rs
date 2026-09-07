use crate::RepresentationId;

use super::ResolvedConfirmedTx;

/// Frozen live bytes or a confirmed transaction to revalidate before loading.
/// Public response wrappers keep raw, JSON, and CPFP bodies distinct.
pub(crate) enum ResolvedTxBody {
    Memory { bytes: Vec<u8>, hash: u64 },
    Chain(ResolvedConfirmedTx),
}

impl ResolvedTxBody {
    pub fn memory(bytes: Vec<u8>) -> Self {
        let hash = RepresentationId::content_hash(&bytes);
        Self::Memory { bytes, hash }
    }

    pub fn identity(&self) -> RepresentationId {
        match self {
            Self::Memory { hash, .. } => RepresentationId::Content(*hash),
            Self::Chain(transaction) => transaction.identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_body_owns_the_original_buffer_and_hashes_its_exact_bytes() {
        for bytes in [
            vec![],
            b"[]".to_vec(),
            vec![0, 255, 1],
            "é".as_bytes().to_vec(),
        ] {
            let hash = RepresentationId::content_hash(&bytes);
            let pointer = bytes.as_ptr();
            let body = ResolvedTxBody::memory(bytes);
            assert!(matches!(body.identity(), RepresentationId::Content(actual) if actual == hash));
            let ResolvedTxBody::Memory { bytes, .. } = body else {
                unreachable!()
            };
            assert_eq!(bytes.as_ptr(), pointer);
        }
    }
}
