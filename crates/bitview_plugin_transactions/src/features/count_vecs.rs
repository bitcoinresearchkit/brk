use bitview_traversable::Traversable;
use bitview_vecs::{ColumnarPerBlockCumulativeRolling, LazyColumnPerBlockCumulativeRolling};
use brk_types::StoredU64;
use derive_more::{Deref, DerefMut};
use vecdb::{Rw, StorageMode};

use super::FeatureId;

/// Transaction counts by detected feature.
///
/// Each metric counts transactions containing the feature at least once, not
/// individual occurrences. A transaction can contribute to multiple metrics.
#[derive(Deref, DerefMut, Traversable)]
pub struct CountVecs<M: StorageMode = Rw> {
    /// Counts transactions containing at least one Taproot script-path input
    /// whose tapscript contains the Ordinals envelope prefix
    /// `OP_0 OP_IF PUSH 'ord'`.
    pub inscription: LazyColumnPerBlockCumulativeRolling<StoredU64, FeatureId>,
    /// Counts transactions containing at least one Taproot input with more
    /// than one witness element whose final element begins with annex prefix
    /// byte `0x50`.
    pub annex: LazyColumnPerBlockCumulativeRolling<StoredU64, FeatureId>,
    /// Counts transactions containing at least one detected `SIGHASH_ALL`
    /// signature. This base hash type commits the signature to every output;
    /// the independently detected `SIGHASH_ANYONECANPAY` modifier can narrow its
    /// input commitment to the signing input.
    pub sighash_all: LazyColumnPerBlockCumulativeRolling<StoredU64, FeatureId>,
    /// Counts transactions containing at least one detected `SIGHASH_NONE`
    /// signature. This base hash type commits to no transaction outputs, so
    /// outputs may be changed after signing.
    pub sighash_none: LazyColumnPerBlockCumulativeRolling<StoredU64, FeatureId>,
    /// Counts transactions containing at least one detected `SIGHASH_SINGLE`
    /// signature. This base hash type normally commits only to the output at the
    /// same position as the signing input; legacy signatures without a
    /// corresponding output retain Bitcoin's historical `SIGHASH_SINGLE` bug.
    pub sighash_single: LazyColumnPerBlockCumulativeRolling<StoredU64, FeatureId>,
    /// Counts transactions containing at least one detected Taproot
    /// `SIGHASH_DEFAULT` signature. Taproot's omitted hash-type byte has the
    /// same commitments as `SIGHASH_ALL` without `SIGHASH_ANYONECANPAY`.
    pub sighash_default: LazyColumnPerBlockCumulativeRolling<StoredU64, FeatureId>,
    /// Counts transactions containing at least one detected signature with the
    /// `SIGHASH_ANYONECANPAY` modifier, which commits only to the signing input
    /// rather than every input. This is counted independently from
    /// `SIGHASH_ALL`, `SIGHASH_NONE`, and `SIGHASH_SINGLE`.
    pub sighash_anyone_can_pay: LazyColumnPerBlockCumulativeRolling<StoredU64, FeatureId>,
    /// Counts non-coinbase transactions containing at least one output below
    /// BRK's type-specific minimal non-dust value; `OP_RETURN` is excluded.
    pub dust_output: LazyColumnPerBlockCumulativeRolling<StoredU64, FeatureId>,
    #[deref]
    #[deref_mut]
    #[traversable(hidden)]
    pub source: ColumnarPerBlockCumulativeRolling<StoredU64, FeatureId, (), M>,
}
