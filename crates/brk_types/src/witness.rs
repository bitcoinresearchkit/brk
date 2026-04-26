use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Transaction witness: a stack of byte arrays, one per witness item.
///
/// Wraps `bitcoin::Witness` (single-buffer layout with offsets, much
/// more compact than `Vec<Vec<u8>>`). Serializes as a JSON array of
/// hex strings - the format used by Bitcoin Core REST and mempool.space
/// and matching brk's `script_sig: ScriptBuf` (bytes internally, hex
/// on the wire).
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
#[schemars(with = "Vec<String>")]
pub struct Witness(bitcoin::Witness);

impl Witness {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn last(&self) -> Option<&[u8]> {
        self.0.last()
    }

    #[inline]
    pub fn second_to_last(&self) -> Option<&[u8]> {
        self.0.second_to_last()
    }

    #[inline]
    pub fn iter(&self) -> bitcoin::blockdata::witness::Iter<'_> {
        self.0.iter()
    }
}

impl From<bitcoin::Witness> for Witness {
    #[inline]
    fn from(w: bitcoin::Witness) -> Self {
        Self(w)
    }
}

impl From<Witness> for bitcoin::Witness {
    #[inline]
    fn from(w: Witness) -> Self {
        w.0
    }
}

impl From<&Witness> for bitcoin::Witness {
    #[inline]
    fn from(w: &Witness) -> Self {
        w.0.clone()
    }
}
