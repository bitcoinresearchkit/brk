use std::ops::{Add, AddAssign};

#[cfg(feature = "storage")]
use bitview_traversable::Traversable;
use brk_types::OutputType;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{CohortName, Filter};

pub const SPENDABLE_TYPE_COUNT: usize = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SpendableTypeId {
    P2PK65,
    P2PK33,
    P2PKH,
    P2MS,
    P2SH,
    P2WPKH,
    P2WSH,
    P2TR,
    P2A,
    Unknown,
    Empty,
}

pub const SPENDABLE_TYPE_IDS: [SpendableTypeId; SPENDABLE_TYPE_COUNT] = [
    SpendableTypeId::P2PK65,
    SpendableTypeId::P2PK33,
    SpendableTypeId::P2PKH,
    SpendableTypeId::P2MS,
    SpendableTypeId::P2SH,
    SpendableTypeId::P2WPKH,
    SpendableTypeId::P2WSH,
    SpendableTypeId::P2TR,
    SpendableTypeId::P2A,
    SpendableTypeId::Unknown,
    SpendableTypeId::Empty,
];

impl SpendableTypeId {
    #[inline]
    pub fn select<T>(self, values: &SpendableType<T>) -> &T {
        match self {
            Self::P2PK65 => &values.p2pk65,
            Self::P2PK33 => &values.p2pk33,
            Self::P2PKH => &values.p2pkh,
            Self::P2MS => &values.p2ms,
            Self::P2SH => &values.p2sh,
            Self::P2WPKH => &values.p2wpkh,
            Self::P2WSH => &values.p2wsh,
            Self::P2TR => &values.p2tr,
            Self::P2A => &values.p2a,
            Self::Unknown => &values.unknown,
            Self::Empty => &values.empty,
        }
    }

    #[inline]
    pub fn select_mut<T>(self, values: &mut SpendableType<T>) -> &mut T {
        match self {
            Self::P2PK65 => &mut values.p2pk65,
            Self::P2PK33 => &mut values.p2pk33,
            Self::P2PKH => &mut values.p2pkh,
            Self::P2MS => &mut values.p2ms,
            Self::P2SH => &mut values.p2sh,
            Self::P2WPKH => &mut values.p2wpkh,
            Self::P2WSH => &mut values.p2wsh,
            Self::P2TR => &mut values.p2tr,
            Self::P2A => &mut values.p2a,
            Self::Unknown => &mut values.unknown,
            Self::Empty => &mut values.empty,
        }
    }

    pub const fn from_output_type(value: OutputType) -> Option<Self> {
        match value {
            OutputType::P2PK65 => Some(Self::P2PK65),
            OutputType::P2PK33 => Some(Self::P2PK33),
            OutputType::P2PKH => Some(Self::P2PKH),
            OutputType::P2MS => Some(Self::P2MS),
            OutputType::P2SH => Some(Self::P2SH),
            OutputType::P2WPKH => Some(Self::P2WPKH),
            OutputType::P2WSH => Some(Self::P2WSH),
            OutputType::P2TR => Some(Self::P2TR),
            OutputType::P2A => Some(Self::P2A),
            OutputType::Unknown => Some(Self::Unknown),
            OutputType::Empty => Some(Self::Empty),
            OutputType::OpReturn => None,
        }
    }

    pub const fn output_type(self) -> OutputType {
        match self {
            Self::P2PK65 => OutputType::P2PK65,
            Self::P2PK33 => OutputType::P2PK33,
            Self::P2PKH => OutputType::P2PKH,
            Self::P2MS => OutputType::P2MS,
            Self::P2SH => OutputType::P2SH,
            Self::P2WPKH => OutputType::P2WPKH,
            Self::P2WSH => OutputType::P2WSH,
            Self::P2TR => OutputType::P2TR,
            Self::P2A => OutputType::P2A,
            Self::Unknown => OutputType::Unknown,
            Self::Empty => OutputType::Empty,
        }
    }
}

impl SpendableTypeId {
    pub const ALL: &'static [Self] = &SPENDABLE_TYPE_IDS;

    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }
}

/// Spendable type values
pub const SPENDABLE_TYPE_VALUES: SpendableType<OutputType> = SpendableType {
    p2pk65: OutputType::P2PK65,
    p2pk33: OutputType::P2PK33,
    p2pkh: OutputType::P2PKH,
    p2ms: OutputType::P2MS,
    p2sh: OutputType::P2SH,
    p2wpkh: OutputType::P2WPKH,
    p2wsh: OutputType::P2WSH,
    p2tr: OutputType::P2TR,
    p2a: OutputType::P2A,
    unknown: OutputType::Unknown,
    empty: OutputType::Empty,
};

/// Spendable type filters
pub const SPENDABLE_TYPE_FILTERS: SpendableType<Filter> = SpendableType {
    p2pk65: Filter::Type(SPENDABLE_TYPE_VALUES.p2pk65),
    p2pk33: Filter::Type(SPENDABLE_TYPE_VALUES.p2pk33),
    p2pkh: Filter::Type(SPENDABLE_TYPE_VALUES.p2pkh),
    p2ms: Filter::Type(SPENDABLE_TYPE_VALUES.p2ms),
    p2sh: Filter::Type(SPENDABLE_TYPE_VALUES.p2sh),
    p2wpkh: Filter::Type(SPENDABLE_TYPE_VALUES.p2wpkh),
    p2wsh: Filter::Type(SPENDABLE_TYPE_VALUES.p2wsh),
    p2tr: Filter::Type(SPENDABLE_TYPE_VALUES.p2tr),
    p2a: Filter::Type(SPENDABLE_TYPE_VALUES.p2a),
    unknown: Filter::Type(SPENDABLE_TYPE_VALUES.unknown),
    empty: Filter::Type(SPENDABLE_TYPE_VALUES.empty),
};

/// Spendable type names
pub const SPENDABLE_TYPE_NAMES: SpendableType<CohortName> = SpendableType {
    p2pk65: CohortName::new("p2pk65", "P2PK65", "Pay to Public Key (65 bytes)"),
    p2pk33: CohortName::new("p2pk33", "P2PK33", "Pay to Public Key (33 bytes)"),
    p2pkh: CohortName::new("p2pkh", "P2PKH", "Pay to Public Key Hash"),
    p2ms: CohortName::new("p2ms", "P2MS", "Pay to Multisig"),
    p2sh: CohortName::new("p2sh", "P2SH", "Pay to Script Hash"),
    p2wpkh: CohortName::new("p2wpkh", "P2WPKH", "Pay to Witness Public Key Hash"),
    p2wsh: CohortName::new("p2wsh", "P2WSH", "Pay to Witness Script Hash"),
    p2tr: CohortName::new("p2tr", "P2TR", "Pay to Taproot"),
    p2a: CohortName::new("p2a", "P2A", "Pay to Anchor"),
    unknown: CohortName::new("unknown_outputs", "Unknown", "Unknown Output Type"),
    empty: CohortName::new("empty_outputs", "Empty", "Empty Output"),
};

#[derive(Default, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct SpendableType<T> {
    /// Uses pay-to-public-key outputs with a 65-byte key field.
    pub p2pk65: T,
    /// Uses pay-to-public-key outputs with a 33-byte key field.
    pub p2pk33: T,
    /// Uses pay-to-public-key-hash outputs.
    pub p2pkh: T,
    /// Uses bare pay-to-multisig outputs.
    pub p2ms: T,
    /// Uses pay-to-script-hash outputs.
    pub p2sh: T,
    /// Uses version-0 pay-to-witness-public-key-hash outputs.
    pub p2wpkh: T,
    /// Uses version-0 pay-to-witness-script-hash outputs.
    pub p2wsh: T,
    /// Uses pay-to-Taproot outputs.
    pub p2tr: T,
    /// Uses pay-to-Anchor outputs.
    pub p2a: T,
    /// Uses outputs that do not match another recognized locking-script type.
    pub unknown: T,
    /// Uses outputs with an empty locking script.
    pub empty: T,
}

impl<T> SpendableType<T> {
    pub fn from_fn(mut f: impl FnMut(SpendableTypeId) -> T) -> Self {
        Self {
            p2pk65: f(SpendableTypeId::P2PK65),
            p2pk33: f(SpendableTypeId::P2PK33),
            p2pkh: f(SpendableTypeId::P2PKH),
            p2ms: f(SpendableTypeId::P2MS),
            p2sh: f(SpendableTypeId::P2SH),
            p2wpkh: f(SpendableTypeId::P2WPKH),
            p2wsh: f(SpendableTypeId::P2WSH),
            p2tr: f(SpendableTypeId::P2TR),
            p2a: f(SpendableTypeId::P2A),
            unknown: f(SpendableTypeId::Unknown),
            empty: f(SpendableTypeId::Empty),
        }
    }
}

impl_collection_formattable!(SpendableType {
    p2pk65,
    p2pk33,
    p2pkh,
    p2ms,
    p2sh,
    p2wpkh,
    p2wsh,
    p2tr,
    p2a,
    unknown,
    empty,
});

impl SpendableType<CohortName> {
    pub const fn names() -> &'static Self {
        &SPENDABLE_TYPE_NAMES
    }
}

impl<T> SpendableType<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        let f = SPENDABLE_TYPE_FILTERS;
        let n = SPENDABLE_TYPE_NAMES;
        Self {
            p2pk65: create(f.p2pk65, n.p2pk65.id),
            p2pk33: create(f.p2pk33, n.p2pk33.id),
            p2pkh: create(f.p2pkh, n.p2pkh.id),
            p2ms: create(f.p2ms, n.p2ms.id),
            p2sh: create(f.p2sh, n.p2sh.id),
            p2wpkh: create(f.p2wpkh, n.p2wpkh.id),
            p2wsh: create(f.p2wsh, n.p2wsh.id),
            p2tr: create(f.p2tr, n.p2tr.id),
            p2a: create(f.p2a, n.p2a.id),
            unknown: create(f.unknown, n.unknown.id),
            empty: create(f.empty, n.empty.id),
        }
    }

    pub fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(Filter, &'static str) -> Result<T, E>,
    {
        let f = SPENDABLE_TYPE_FILTERS;
        let n = SPENDABLE_TYPE_NAMES;
        Ok(Self {
            p2pk65: create(f.p2pk65, n.p2pk65.id)?,
            p2pk33: create(f.p2pk33, n.p2pk33.id)?,
            p2pkh: create(f.p2pkh, n.p2pkh.id)?,
            p2ms: create(f.p2ms, n.p2ms.id)?,
            p2sh: create(f.p2sh, n.p2sh.id)?,
            p2wpkh: create(f.p2wpkh, n.p2wpkh.id)?,
            p2wsh: create(f.p2wsh, n.p2wsh.id)?,
            p2tr: create(f.p2tr, n.p2tr.id)?,
            p2a: create(f.p2a, n.p2a.id)?,
            unknown: create(f.unknown, n.unknown.id)?,
            empty: create(f.empty, n.empty.id)?,
        })
    }

    pub fn get(&self, output_type: OutputType) -> &T {
        match output_type {
            OutputType::P2PK65 => &self.p2pk65,
            OutputType::P2PK33 => &self.p2pk33,
            OutputType::P2PKH => &self.p2pkh,
            OutputType::P2MS => &self.p2ms,
            OutputType::P2SH => &self.p2sh,
            OutputType::P2WPKH => &self.p2wpkh,
            OutputType::P2WSH => &self.p2wsh,
            OutputType::P2TR => &self.p2tr,
            OutputType::P2A => &self.p2a,
            OutputType::Unknown => &self.unknown,
            OutputType::Empty => &self.empty,
            _ => unreachable!(),
        }
    }

    pub fn get_mut(&mut self, output_type: OutputType) -> &mut T {
        match output_type {
            OutputType::P2PK65 => &mut self.p2pk65,
            OutputType::P2PK33 => &mut self.p2pk33,
            OutputType::P2PKH => &mut self.p2pkh,
            OutputType::P2MS => &mut self.p2ms,
            OutputType::P2SH => &mut self.p2sh,
            OutputType::P2WPKH => &mut self.p2wpkh,
            OutputType::P2WSH => &mut self.p2wsh,
            OutputType::P2TR => &mut self.p2tr,
            OutputType::P2A => &mut self.p2a,
            OutputType::Unknown => &mut self.unknown,
            OutputType::Empty => &mut self.empty,
            _ => unreachable!(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [
            &self.p2pk65,
            &self.p2pk33,
            &self.p2pkh,
            &self.p2ms,
            &self.p2sh,
            &self.p2wpkh,
            &self.p2wsh,
            &self.p2tr,
            &self.p2a,
            &self.unknown,
            &self.empty,
        ]
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self.p2pk65,
            &mut self.p2pk33,
            &mut self.p2pkh,
            &mut self.p2ms,
            &mut self.p2sh,
            &mut self.p2wpkh,
            &mut self.p2wsh,
            &mut self.p2tr,
            &mut self.p2a,
            &mut self.unknown,
            &mut self.empty,
        ]
        .into_iter()
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [
            &mut self.p2pk65,
            &mut self.p2pk33,
            &mut self.p2pkh,
            &mut self.p2ms,
            &mut self.p2sh,
            &mut self.p2wpkh,
            &mut self.p2wsh,
            &mut self.p2tr,
            &mut self.p2a,
            &mut self.unknown,
            &mut self.empty,
        ]
        .into_par_iter()
    }

    pub fn iter_typed(&self) -> impl Iterator<Item = (OutputType, &T)> {
        [
            (OutputType::P2PK65, &self.p2pk65),
            (OutputType::P2PK33, &self.p2pk33),
            (OutputType::P2PKH, &self.p2pkh),
            (OutputType::P2MS, &self.p2ms),
            (OutputType::P2SH, &self.p2sh),
            (OutputType::P2WPKH, &self.p2wpkh),
            (OutputType::P2WSH, &self.p2wsh),
            (OutputType::P2TR, &self.p2tr),
            (OutputType::P2A, &self.p2a),
            (OutputType::Unknown, &self.unknown),
            (OutputType::Empty, &self.empty),
        ]
        .into_iter()
    }

    pub fn iter_typed_mut(&mut self) -> impl Iterator<Item = (OutputType, &mut T)> {
        [
            (OutputType::P2PK65, &mut self.p2pk65),
            (OutputType::P2PK33, &mut self.p2pk33),
            (OutputType::P2PKH, &mut self.p2pkh),
            (OutputType::P2MS, &mut self.p2ms),
            (OutputType::P2SH, &mut self.p2sh),
            (OutputType::P2WPKH, &mut self.p2wpkh),
            (OutputType::P2WSH, &mut self.p2wsh),
            (OutputType::P2TR, &mut self.p2tr),
            (OutputType::P2A, &mut self.p2a),
            (OutputType::Unknown, &mut self.unknown),
            (OutputType::Empty, &mut self.empty),
        ]
        .into_iter()
    }
}

impl<T> Add for SpendableType<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            p2pk65: self.p2pk65 + rhs.p2pk65,
            p2pk33: self.p2pk33 + rhs.p2pk33,
            p2pkh: self.p2pkh + rhs.p2pkh,
            p2ms: self.p2ms + rhs.p2ms,
            p2sh: self.p2sh + rhs.p2sh,
            p2wpkh: self.p2wpkh + rhs.p2wpkh,
            p2wsh: self.p2wsh + rhs.p2wsh,
            p2tr: self.p2tr + rhs.p2tr,
            p2a: self.p2a + rhs.p2a,
            unknown: self.unknown + rhs.unknown,
            empty: self.empty + rhs.empty,
        }
    }
}

impl<T> AddAssign for SpendableType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.p2pk65 += rhs.p2pk65;
        self.p2pk33 += rhs.p2pk33;
        self.p2pkh += rhs.p2pkh;
        self.p2ms += rhs.p2ms;
        self.p2sh += rhs.p2sh;
        self.p2wpkh += rhs.p2wpkh;
        self.p2wsh += rhs.p2wsh;
        self.p2tr += rhs.p2tr;
        self.p2a += rhs.p2a;
        self.unknown += rhs.unknown;
        self.empty += rhs.empty;
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "storage")]
    use super::*;

    #[cfg(feature = "storage")]
    #[test]
    fn cohort_ids_match_spendable_type_order() {
        let output_types: Vec<_> = SPENDABLE_TYPE_VALUES.iter().copied().collect();
        let selected_output_types: Vec<_> = SpendableTypeId::ALL
            .iter()
            .map(|id| id.output_type())
            .collect();

        assert_eq!(selected_output_types, output_types);
        assert_eq!(
            SpendableTypeId::from_output_type(OutputType::OpReturn),
            None
        );

        let values = SpendableType::from_fn(|id| id.index());
        for id in SpendableTypeId::ALL {
            assert_eq!(*id.select(&values), id.index());
        }
    }
}
