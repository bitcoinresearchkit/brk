use std::ops::{Deref, DerefMut};

use crate::{
    DateIndex, DecadeIndex, DifficultyEpoch, EmptyOutputIndex, HalvingEpoch, Height, MonthIndex,
    OpReturnIndex, OutputType, P2AAddressIndex, P2MSOutputIndex, P2PK33AddressIndex,
    P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex, P2WPKHAddressIndex,
    P2WSHAddressIndex, QuarterIndex, SemesterIndex, TxInIndex, TxIndex, TxOutIndex, TypeIndex,
    UnknownOutputIndex, WeekIndex, YearIndex,
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
#[derive(Debug, Clone)]
pub struct ComputeIndexes {
    indexes: Indexes,
    pub dateindex: DateIndex,
    pub weekindex: WeekIndex,
    pub monthindex: MonthIndex,
    pub quarterindex: QuarterIndex,
    pub semesterindex: SemesterIndex,
    pub yearindex: YearIndex,
    pub decadeindex: DecadeIndex,
    pub difficultyepoch: DifficultyEpoch,
    pub halvingepoch: HalvingEpoch,
}

impl ComputeIndexes {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        indexes: Indexes,
        dateindex: DateIndex,
        weekindex: WeekIndex,
        monthindex: MonthIndex,
        quarterindex: QuarterIndex,
        semesterindex: SemesterIndex,
        yearindex: YearIndex,
        decadeindex: DecadeIndex,
        difficultyepoch: DifficultyEpoch,
        halvingepoch: HalvingEpoch,
    ) -> Self {
        Self {
            indexes,
            dateindex,
            weekindex,
            monthindex,
            quarterindex,
            semesterindex,
            yearindex,
            decadeindex,
            difficultyepoch,
            halvingepoch,
        }
    }
}

impl Deref for ComputeIndexes {
    type Target = Indexes;
    fn deref(&self) -> &Self::Target {
        &self.indexes
    }
}

impl DerefMut for ComputeIndexes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.indexes
    }
}
