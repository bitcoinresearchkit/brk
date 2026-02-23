use std::fmt::{self, Debug};

use brk_error::Error;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::PrintableIndex;

use crate::PairOutputIndex;

use super::{
    Date, Day1, Day3, Year10, DifficultyEpoch, EmptyAddressIndex, EmptyOutputIndex, HalvingEpoch,
    Height, Hour1, Hour4, Hour12, FundedAddressIndex, Minute1, Minute5, Minute10, Minute30,
    Month1, OpReturnIndex, P2AAddressIndex, P2MSOutputIndex,
    P2PK33AddressIndex, P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex,
    P2WPKHAddressIndex, P2WSHAddressIndex, Month3, Month6, Timestamp, TxInIndex, TxIndex,
    TxOutIndex, UnknownOutputIndex, Week1, Year1,
    timestamp::INDEX_EPOCH,
    minute1::MINUTE1_INTERVAL, minute5::MINUTE5_INTERVAL, minute10::MINUTE10_INTERVAL,
    minute30::MINUTE30_INTERVAL, hour1::HOUR1_INTERVAL, hour4::HOUR4_INTERVAL,
    hour12::HOUR12_INTERVAL, day3::DAY3_INTERVAL,
};

/// Aggregation dimension for querying metrics. Includes time-based (date, week, month, year),
/// block-based (height, txindex), and address/output type indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[schemars(example = Index::Day1)]
pub enum Index {
    Minute1,
    Minute5,
    Minute10,
    Minute30,
    Hour1,
    Hour4,
    Hour12,
    Day1,
    Day3,
    Week1,
    Month1,
    Month3,
    Month6,
    Year1,
    Year10,
    HalvingEpoch,
    DifficultyEpoch,
    Height,
    TxIndex,
    TxInIndex,
    TxOutIndex,
    EmptyOutputIndex,
    OpReturnIndex,
    P2AAddressIndex,
    P2MSOutputIndex,
    P2PK33AddressIndex,
    P2PK65AddressIndex,
    P2PKHAddressIndex,
    P2SHAddressIndex,
    P2TRAddressIndex,
    P2WPKHAddressIndex,
    P2WSHAddressIndex,
    UnknownOutputIndex,
    FundedAddressIndex,
    EmptyAddressIndex,
    PairOutputIndex,
}

impl Index {
    pub const fn all() -> [Self; 36] {
        [
            Self::Minute1,
            Self::Minute5,
            Self::Minute10,
            Self::Minute30,
            Self::Hour1,
            Self::Hour4,
            Self::Hour12,
            Self::Day1,
            Self::Day3,
            Self::Week1,
            Self::Month1,
            Self::Month3,
            Self::Month6,
            Self::Year1,
            Self::Year10,
            Self::HalvingEpoch,
            Self::DifficultyEpoch,
            Self::Height,
            Self::TxIndex,
            Self::TxInIndex,
            Self::TxOutIndex,
            Self::EmptyOutputIndex,
            Self::OpReturnIndex,
            Self::P2AAddressIndex,
            Self::P2MSOutputIndex,
            Self::P2PK33AddressIndex,
            Self::P2PK65AddressIndex,
            Self::P2PKHAddressIndex,
            Self::P2SHAddressIndex,
            Self::P2TRAddressIndex,
            Self::P2WPKHAddressIndex,
            Self::P2WSHAddressIndex,
            Self::UnknownOutputIndex,
            Self::FundedAddressIndex,
            Self::EmptyAddressIndex,
            Self::PairOutputIndex,
        ]
    }

    pub fn possible_values(&self) -> &'static [&'static str] {
        match self {
            Self::Minute1 => Minute1::to_possible_strings(),
            Self::Minute5 => Minute5::to_possible_strings(),
            Self::Minute10 => Minute10::to_possible_strings(),
            Self::Minute30 => Minute30::to_possible_strings(),
            Self::Hour1 => Hour1::to_possible_strings(),
            Self::Hour4 => Hour4::to_possible_strings(),
            Self::Hour12 => Hour12::to_possible_strings(),
            Self::Day1 => Day1::to_possible_strings(),
            Self::Day3 => Day3::to_possible_strings(),
            Self::Week1 => Week1::to_possible_strings(),
            Self::Month1 => Month1::to_possible_strings(),
            Self::Month3 => Month3::to_possible_strings(),
            Self::Month6 => Month6::to_possible_strings(),
            Self::Year1 => Year1::to_possible_strings(),
            Self::Year10 => Year10::to_possible_strings(),
            Self::HalvingEpoch => HalvingEpoch::to_possible_strings(),
            Self::DifficultyEpoch => DifficultyEpoch::to_possible_strings(),
            Self::Height => Height::to_possible_strings(),
            Self::TxIndex => TxIndex::to_possible_strings(),
            Self::TxInIndex => TxInIndex::to_possible_strings(),
            Self::TxOutIndex => TxOutIndex::to_possible_strings(),
            Self::EmptyOutputIndex => EmptyOutputIndex::to_possible_strings(),
            Self::OpReturnIndex => OpReturnIndex::to_possible_strings(),
            Self::P2AAddressIndex => P2AAddressIndex::to_possible_strings(),
            Self::P2MSOutputIndex => P2MSOutputIndex::to_possible_strings(),
            Self::P2PK33AddressIndex => P2PK33AddressIndex::to_possible_strings(),
            Self::P2PK65AddressIndex => P2PK65AddressIndex::to_possible_strings(),
            Self::P2PKHAddressIndex => P2PKHAddressIndex::to_possible_strings(),
            Self::P2SHAddressIndex => P2SHAddressIndex::to_possible_strings(),
            Self::P2TRAddressIndex => P2TRAddressIndex::to_possible_strings(),
            Self::P2WPKHAddressIndex => P2WPKHAddressIndex::to_possible_strings(),
            Self::P2WSHAddressIndex => P2WSHAddressIndex::to_possible_strings(),
            Self::UnknownOutputIndex => UnknownOutputIndex::to_possible_strings(),
            Self::FundedAddressIndex => FundedAddressIndex::to_possible_strings(),
            Self::EmptyAddressIndex => EmptyAddressIndex::to_possible_strings(),
            Self::PairOutputIndex => PairOutputIndex::to_possible_strings(),
        }
    }

    pub fn all_possible_values() -> Vec<&'static str> {
        Self::all()
            .into_iter()
            .flat_map(|i| i.possible_values().iter().cloned())
            .collect::<Vec<_>>()
    }


    pub fn name(&self) -> &'static str {
        match self {
            Self::Minute1 => <Minute1 as PrintableIndex>::to_string(),
            Self::Minute5 => <Minute5 as PrintableIndex>::to_string(),
            Self::Minute10 => <Minute10 as PrintableIndex>::to_string(),
            Self::Minute30 => <Minute30 as PrintableIndex>::to_string(),
            Self::Hour1 => <Hour1 as PrintableIndex>::to_string(),
            Self::Hour4 => <Hour4 as PrintableIndex>::to_string(),
            Self::Hour12 => <Hour12 as PrintableIndex>::to_string(),
            Self::Day1 => <Day1 as PrintableIndex>::to_string(),
            Self::Day3 => <Day3 as PrintableIndex>::to_string(),
            Self::Week1 => <Week1 as PrintableIndex>::to_string(),
            Self::Month1 => <Month1 as PrintableIndex>::to_string(),
            Self::Month3 => <Month3 as PrintableIndex>::to_string(),
            Self::Month6 => <Month6 as PrintableIndex>::to_string(),
            Self::Year1 => <Year1 as PrintableIndex>::to_string(),
            Self::Year10 => <Year10 as PrintableIndex>::to_string(),
            Self::HalvingEpoch => <HalvingEpoch as PrintableIndex>::to_string(),
            Self::DifficultyEpoch => <DifficultyEpoch as PrintableIndex>::to_string(),
            Self::Height => <Height as PrintableIndex>::to_string(),
            Self::TxIndex => <TxIndex as PrintableIndex>::to_string(),
            Self::TxInIndex => <TxInIndex as PrintableIndex>::to_string(),
            Self::TxOutIndex => <TxOutIndex as PrintableIndex>::to_string(),
            Self::EmptyOutputIndex => <EmptyOutputIndex as PrintableIndex>::to_string(),
            Self::OpReturnIndex => <OpReturnIndex as PrintableIndex>::to_string(),
            Self::P2AAddressIndex => <P2AAddressIndex as PrintableIndex>::to_string(),
            Self::P2MSOutputIndex => <P2MSOutputIndex as PrintableIndex>::to_string(),
            Self::P2PK33AddressIndex => <P2PK33AddressIndex as PrintableIndex>::to_string(),
            Self::P2PK65AddressIndex => <P2PK65AddressIndex as PrintableIndex>::to_string(),
            Self::P2PKHAddressIndex => <P2PKHAddressIndex as PrintableIndex>::to_string(),
            Self::P2SHAddressIndex => <P2SHAddressIndex as PrintableIndex>::to_string(),
            Self::P2TRAddressIndex => <P2TRAddressIndex as PrintableIndex>::to_string(),
            Self::P2WPKHAddressIndex => <P2WPKHAddressIndex as PrintableIndex>::to_string(),
            Self::P2WSHAddressIndex => <P2WSHAddressIndex as PrintableIndex>::to_string(),
            Self::UnknownOutputIndex => <UnknownOutputIndex as PrintableIndex>::to_string(),
            Self::FundedAddressIndex => <FundedAddressIndex as PrintableIndex>::to_string(),
            Self::EmptyAddressIndex => <EmptyAddressIndex as PrintableIndex>::to_string(),
            Self::PairOutputIndex => <PairOutputIndex as PrintableIndex>::to_string(),
        }
    }

    /// Returns the query cost multiplier for this index type.
    /// Used for rate limiting to account for expensive lazy computations.
    pub const fn cost_multiplier(&self) -> usize {
        match self {
            Self::DifficultyEpoch => 60,
            _ => 1,
        }
    }

    /// Returns true if this index type is date-based.
    pub const fn is_date_based(&self) -> bool {
        matches!(
            self,
            Self::Minute1
                | Self::Minute5
                | Self::Minute10
                | Self::Minute30
                | Self::Hour1
                | Self::Hour4
                | Self::Hour12
                | Self::Day1
                | Self::Day3
                | Self::Week1
                | Self::Month1
                | Self::Month3
                | Self::Month6
                | Self::Year1
                | Self::Year10
        )
    }

    /// Convert an index value to a timestamp for time-based indexes.
    /// Returns None for non-time-based indexes.
    pub fn index_to_timestamp(&self, i: usize) -> Option<Timestamp> {
        let interval = match self {
            Self::Minute1 => MINUTE1_INTERVAL,
            Self::Minute5 => MINUTE5_INTERVAL,
            Self::Minute10 => MINUTE10_INTERVAL,
            Self::Minute30 => MINUTE30_INTERVAL,
            Self::Hour1 => HOUR1_INTERVAL,
            Self::Hour4 => HOUR4_INTERVAL,
            Self::Hour12 => HOUR12_INTERVAL,
            Self::Day3 => DAY3_INTERVAL,
            _ => return self.index_to_date(i).map(|d| d.into()),
        };
        Some(Timestamp::new(INDEX_EPOCH + i as u32 * interval))
    }

    /// Convert an index value to a date for date-based indexes.
    /// Returns None for non-date-based or sub-daily indexes.
    pub fn index_to_date(&self, i: usize) -> Option<Date> {
        match self {
            Self::Day1 => Some(Date::from(Day1::from(i))),
            Self::Week1 => Some(Date::from(Week1::from(i))),
            Self::Month1 => Some(Date::from(Month1::from(i))),
            Self::Year1 => Some(Date::from(Year1::from(i))),
            Self::Month3 => Some(Date::from(Month3::from(i))),
            Self::Month6 => Some(Date::from(Month6::from(i))),
            Self::Year10 => Some(Date::from(Year10::from(i))),
            _ => None,
        }
    }
}

impl TryFrom<&str> for Index {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v = value.to_lowercase();
        let v = v.as_str();
        for idx in Self::all() {
            if idx.possible_values().contains(&v) {
                return Ok(idx);
            }
        }
        Err(Error::Parse(format!("Invalid index: {value}")))
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<'de> Deserialize<'de> for Index {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        if let Ok(index) = Index::try_from(str.as_str()) {
            // dbg!(index);
            Ok(index)
        } else {
            Err(serde::de::Error::custom("Bad index"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_date_based_day1() {
        assert!(Index::Day1.is_date_based());
    }

    #[test]
    fn test_is_date_based_week1() {
        assert!(Index::Week1.is_date_based());
    }

    #[test]
    fn test_is_date_based_month1() {
        assert!(Index::Month1.is_date_based());
    }

    #[test]
    fn test_is_date_based_year1() {
        assert!(Index::Year1.is_date_based());
    }

    #[test]
    fn test_is_date_based_month3() {
        assert!(Index::Month3.is_date_based());
    }

    #[test]
    fn test_is_date_based_month6() {
        assert!(Index::Month6.is_date_based());
    }

    #[test]
    fn test_is_date_based_year10() {
        assert!(Index::Year10.is_date_based());
    }

    #[test]
    fn test_is_not_date_based_height() {
        assert!(!Index::Height.is_date_based());
    }

    #[test]
    fn test_is_not_date_based_txindex() {
        assert!(!Index::TxIndex.is_date_based());
    }

    #[test]
    fn test_index_to_date_day1_zero() {
        let date = Index::Day1.index_to_date(0).unwrap();
        assert_eq!(date, Date::from(Day1::from(0_usize)));
    }

    #[test]
    fn test_index_to_date_day1_one() {
        let date = Index::Day1.index_to_date(1).unwrap();
        assert_eq!(date, Date::from(Day1::from(1_usize)));
    }

    #[test]
    fn test_index_to_date_week1() {
        let date = Index::Week1.index_to_date(1).unwrap();
        assert_eq!(date, Date::from(Week1::from(1_usize)));
    }

    #[test]
    fn test_index_to_date_month1() {
        let date = Index::Month1.index_to_date(12).unwrap();
        assert_eq!(date, Date::from(Month1::from(12_usize)));
    }

    #[test]
    fn test_index_to_date_year1() {
        let date = Index::Year1.index_to_date(5).unwrap();
        assert_eq!(date, Date::from(Year1::from(5_usize)));
    }

    #[test]
    fn test_index_to_date_month3() {
        let date = Index::Month3.index_to_date(4).unwrap();
        assert_eq!(date, Date::from(Month3::from(4_usize)));
    }

    #[test]
    fn test_index_to_date_month6() {
        let date = Index::Month6.index_to_date(2).unwrap();
        assert_eq!(date, Date::from(Month6::from(2_usize)));
    }

    #[test]
    fn test_index_to_date_year10() {
        let date = Index::Year10.index_to_date(1).unwrap();
        assert_eq!(date, Date::from(Year10::from(1_usize)));
    }

    #[test]
    fn test_index_to_date_height_returns_none() {
        assert!(Index::Height.index_to_date(100).is_none());
    }

    #[test]
    fn test_index_to_date_txindex_returns_none() {
        assert!(Index::TxIndex.index_to_date(100).is_none());
    }
}
