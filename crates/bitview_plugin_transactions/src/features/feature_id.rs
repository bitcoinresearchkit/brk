use std::array;

use brk_types::Version;
use vecdb::{ColumnId, VecValue};

const FEATURE_COUNT: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeatureId {
    Inscription,
    Annex,
    SighashAll,
    SighashNone,
    SighashSingle,
    SighashDefault,
    SighashAnyoneCanPay,
    DustOutput,
}

const FEATURE_IDS: [FeatureId; FEATURE_COUNT] = [
    FeatureId::Inscription,
    FeatureId::Annex,
    FeatureId::SighashAll,
    FeatureId::SighashNone,
    FeatureId::SighashSingle,
    FeatureId::SighashDefault,
    FeatureId::SighashAnyoneCanPay,
    FeatureId::DustOutput,
];

impl ColumnId for FeatureId {
    type Row<T>
        = [T; FEATURE_COUNT]
    where
        T: VecValue;

    const VERSION: Version = Version::ONE;
    const ALL: &'static [Self] = &FEATURE_IDS;

    #[inline]
    fn index(self) -> usize {
        self as usize
    }

    #[inline]
    fn get<T: VecValue>(self, row: &Self::Row<T>) -> &T {
        &row[self.index()]
    }

    #[inline]
    fn get_mut<T: VecValue>(self, row: &mut Self::Row<T>) -> &mut T {
        &mut row[self.index()]
    }

    #[inline]
    fn from_fn<T, F>(mut create: F) -> Self::Row<T>
    where
        T: VecValue,
        F: FnMut(Self) -> T,
    {
        array::from_fn(|index| create(FEATURE_IDS[index]))
    }

    #[inline]
    fn map<T, U, F>(row: Self::Row<T>, create: F) -> Self::Row<U>
    where
        T: VecValue,
        U: VecValue,
        F: FnMut(T) -> U,
    {
        row.map(create)
    }
}

#[cfg(test)]
mod tests {
    use vecdb::ColumnId;

    use super::{FEATURE_IDS, FeatureId};

    #[test]
    fn feature_columns_match_public_field_order() {
        assert_eq!(FeatureId::ALL, FEATURE_IDS);
        assert_eq!(
            FeatureId::from_fn(|feature| feature),
            [
                FeatureId::Inscription,
                FeatureId::Annex,
                FeatureId::SighashAll,
                FeatureId::SighashNone,
                FeatureId::SighashSingle,
                FeatureId::SighashDefault,
                FeatureId::SighashAnyoneCanPay,
                FeatureId::DustOutput,
            ]
        );
    }
}
