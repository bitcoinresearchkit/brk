//! Field name position types for metric name reconstruction.

/// How a field modifies the accumulated metric name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldNamePosition {
    /// Field prepends a prefix: leaf.name() = prefix + accumulated
    Prepend(String),
    /// Field appends a suffix: leaf.name() = accumulated + suffix
    Append(String),
    /// Field IS the accumulated name (no modification)
    Identity,
    /// Field sets a new base name (used at pattern entry points)
    SetBase(String),
}
