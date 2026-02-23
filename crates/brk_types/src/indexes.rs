use derive_more::{Deref, DerefMut};

use crate::{
    Day1, Day3, Year10, DifficultyEpoch, EmptyOutputIndex, HalvingEpoch, Height,
    Hour1, Hour4, Hour12, Minute1, Minute5, Minute10, Minute30, Month1,
    OpReturnIndex, OutputType, P2AAddressIndex, P2MSOutputIndex, P2PK33AddressIndex,
    P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex, P2WPKHAddressIndex,
    P2WSHAddressIndex, Month3, Month6, TxInIndex, TxIndex, TxOutIndex, TypeIndex,
    UnknownOutputIndex, Week1, Year1,
};

/// Blockchain-level indexes tracking current positions for various entity types.
/// Used by brk_indexer during block processing.
#[derive(Debug, Default, Clone)]
pub struct Indexes {
    pub emptyoutputindex: EmptyOutputIndex,
    pub height: Height,
    pub opreturnindex: OpReturnIndex,
    pub p2msoutputindex: P2MSOutputIndex,
    pub p2pk33addressindex: P2PK33AddressIndex,
    pub p2pk65addressindex: P2PK65AddressIndex,
    pub p2pkhaddressindex: P2PKHAddressIndex,
    pub p2shaddressindex: P2SHAddressIndex,
    pub p2traddressindex: P2TRAddressIndex,
    pub p2wpkhaddressindex: P2WPKHAddressIndex,
    pub p2wshaddressindex: P2WSHAddressIndex,
    pub p2aaddressindex: P2AAddressIndex,
    pub txindex: TxIndex,
    pub txinindex: TxInIndex,
    pub txoutindex: TxOutIndex,
    pub unknownoutputindex: UnknownOutputIndex,
}

impl Indexes {
    pub fn to_typeindex(&self, outputtype: OutputType) -> TypeIndex {
        match outputtype {
            OutputType::Empty => *self.emptyoutputindex,
            OutputType::OpReturn => *self.opreturnindex,
            OutputType::P2A => *self.p2aaddressindex,
            OutputType::P2MS => *self.p2msoutputindex,
            OutputType::P2PK33 => *self.p2pk33addressindex,
            OutputType::P2PK65 => *self.p2pk65addressindex,
            OutputType::P2PKH => *self.p2pkhaddressindex,
            OutputType::P2SH => *self.p2shaddressindex,
            OutputType::P2TR => *self.p2traddressindex,
            OutputType::P2WPKH => *self.p2wpkhaddressindex,
            OutputType::P2WSH => *self.p2wshaddressindex,
            OutputType::Unknown => *self.unknownoutputindex,
        }
    }

    /// Increments the address index for the given address type and returns the previous value.
    /// Only call this for address types (P2PK65, P2PK33, P2PKH, P2SH, P2WPKH, P2WSH, P2TR, P2A).
    #[inline]
    pub fn increment_address_index(&mut self, addresstype: OutputType) -> TypeIndex {
        match addresstype {
            OutputType::P2PK65 => self.p2pk65addressindex.copy_then_increment(),
            OutputType::P2PK33 => self.p2pk33addressindex.copy_then_increment(),
            OutputType::P2PKH => self.p2pkhaddressindex.copy_then_increment(),
            OutputType::P2SH => self.p2shaddressindex.copy_then_increment(),
            OutputType::P2WPKH => self.p2wpkhaddressindex.copy_then_increment(),
            OutputType::P2WSH => self.p2wshaddressindex.copy_then_increment(),
            OutputType::P2TR => self.p2traddressindex.copy_then_increment(),
            OutputType::P2A => self.p2aaddressindex.copy_then_increment(),
            _ => unreachable!(),
        }
    }
}

/// Extended indexes with time-based granularities.
/// Used by brk_computer for time-series aggregation.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct ComputeIndexes {
    #[deref]
    #[deref_mut]
    indexes: Indexes,
    pub minute1: Minute1,
    pub minute5: Minute5,
    pub minute10: Minute10,
    pub minute30: Minute30,
    pub hour1: Hour1,
    pub hour4: Hour4,
    pub hour12: Hour12,
    pub day1: Day1,
    pub day3: Day3,
    pub week1: Week1,
    pub month1: Month1,
    pub month3: Month3,
    pub month6: Month6,
    pub year1: Year1,
    pub year10: Year10,
    pub halvingepoch: HalvingEpoch,
    pub difficultyepoch: DifficultyEpoch,
}

impl ComputeIndexes {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        indexes: Indexes,
        minute1: Minute1,
        minute5: Minute5,
        minute10: Minute10,
        minute30: Minute30,
        hour1: Hour1,
        hour4: Hour4,
        hour12: Hour12,
        day1: Day1,
        day3: Day3,
        week1: Week1,
        month1: Month1,
        month3: Month3,
        month6: Month6,
        year1: Year1,
        year10: Year10,
        halvingepoch: HalvingEpoch,
        difficultyepoch: DifficultyEpoch,
    ) -> Self {
        Self {
            indexes,
            minute1,
            minute5,
            minute10,
            minute30,
            hour1,
            hour4,
            hour12,
            day1,
            day3,
            week1,
            month1,
            month3,
            month6,
            year1,
            year10,
            halvingepoch,
            difficultyepoch,
        }
    }
}

