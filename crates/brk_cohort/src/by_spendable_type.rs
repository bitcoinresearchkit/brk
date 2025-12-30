use std::ops::{Add, AddAssign};

use brk_traversable::Traversable;
use brk_types::OutputType;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Serialize;

use super::{CohortName, Filter};

/// Spendable type values
pub const SPENDABLE_TYPE_VALUES: BySpendableType<OutputType> = BySpendableType {
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
pub const SPENDABLE_TYPE_FILTERS: BySpendableType<Filter> = BySpendableType {
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
pub const SPENDABLE_TYPE_NAMES: BySpendableType<CohortName> = BySpendableType {
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

#[derive(Default, Clone, Debug, Traversable, Serialize)]
pub struct BySpendableType<T> {
    pub p2pk65: T,
    pub p2pk33: T,
    pub p2pkh: T,
    pub p2ms: T,
    pub p2sh: T,
    pub p2wpkh: T,
    pub p2wsh: T,
    pub p2tr: T,
    pub p2a: T,
    pub unknown: T,
    pub empty: T,
}

impl BySpendableType<CohortName> {
    pub const fn names() -> &'static Self {
        &SPENDABLE_TYPE_NAMES
    }
}

impl<T> BySpendableType<T> {
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

impl<T> Add for BySpendableType<T>
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

impl<T> AddAssign for BySpendableType<T>
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
