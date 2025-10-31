use std::ops::{Add, AddAssign};

use brk_traversable::Traversable;
use brk_types::OutputType;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::{Filter, Filtered};

#[derive(Default, Clone, Debug, Traversable)]
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

impl<T> BySpendableType<T> {
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
}

impl<T> BySpendableType<Filtered<T>> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [
            &self.p2pk65.1,
            &self.p2pk33.1,
            &self.p2pkh.1,
            &self.p2ms.1,
            &self.p2sh.1,
            &self.p2wpkh.1,
            &self.p2wsh.1,
            &self.p2tr.1,
            &self.p2a.1,
            &self.unknown.1,
            &self.empty.1,
        ]
        .into_iter()
    }
}

impl<T> From<BySpendableType<T>> for BySpendableType<Filtered<T>> {
    #[inline]
    fn from(value: BySpendableType<T>) -> Self {
        Self {
            p2pk65: (Filter::Type(OutputType::P2PK65), value.p2pk65).into(),
            p2pk33: (Filter::Type(OutputType::P2PK33), value.p2pk33).into(),
            p2pkh: (Filter::Type(OutputType::P2PKH), value.p2pkh).into(),
            p2ms: (Filter::Type(OutputType::P2MS), value.p2ms).into(),
            p2sh: (Filter::Type(OutputType::P2SH), value.p2sh).into(),
            p2wpkh: (Filter::Type(OutputType::P2WPKH), value.p2wpkh).into(),
            p2wsh: (Filter::Type(OutputType::P2WSH), value.p2wsh).into(),
            p2tr: (Filter::Type(OutputType::P2TR), value.p2tr).into(),
            p2a: (Filter::Type(OutputType::P2A), value.p2a).into(),
            unknown: (Filter::Type(OutputType::Unknown), value.unknown).into(),
            empty: (Filter::Type(OutputType::Empty), value.empty).into(),
        }
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
