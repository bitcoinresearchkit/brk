#[cfg(feature = "storage")]
use std::str as StdStr;

#[cfg(feature = "storage")]
use std::fmt;
use std::ops::AddAssign;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    LOSS_COUNT, Loss, LossId, PROFIT_COUNT, PROFITABILITY_RANGE_COUNT, Profit, ProfitId,
    ProfitabilityRange, ProfitabilityRangeId,
};

#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

#[cfg(feature = "storage")]
use vecdb::Formattable;

pub const PROFITABILITY_COUNT: usize = PROFITABILITY_RANGE_COUNT + PROFIT_COUNT + LOSS_COUNT;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct Profitability<T> {
    pub range: ProfitabilityRange<T>,
    pub profit: Profit<T>,
    pub loss: Loss<T>,
}

impl<T> Profitability<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.range
            .iter()
            .chain(self.profit.iter())
            .chain(self.loss.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.range
            .iter_mut()
            .chain(self.profit.iter_mut())
            .chain(self.loss.iter_mut())
    }
}

impl<T> Profitability<T>
where
    T: AddAssign + Copy + Default,
{
    pub fn from_ranges(range: ProfitabilityRange<T>) -> Self {
        let mut profit = Profit::default();
        let mut profit_ranges = range.iter().take(PROFIT_COUNT + 1);
        let mut total = *profit_ranges.next().expect("profitability profit range");
        for (threshold, &value) in profit.iter_mut().rev().zip(profit_ranges) {
            total += value;
            *threshold = total;
        }

        let mut loss = Loss::default();
        let mut loss_ranges = range.iter().skip(PROFIT_COUNT + 1).rev();
        let mut total = *loss_ranges.next().expect("profitability loss range");
        for (threshold, &value) in loss.iter_mut().rev().zip(loss_ranges) {
            total += value;
            *threshold = total;
        }

        Self {
            range,
            profit,
            loss,
        }
    }
}

#[cfg(feature = "storage")]
impl<T: Formattable> Formattable for Profitability<T> {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(b"{\"range\":");
        write_array(self.range.iter(), buf);
        buf.extend_from_slice(b",\"profit\":");
        write_array(self.profit.iter(), buf);
        buf.extend_from_slice(b",\"loss\":");
        write_array(self.loss.iter(), buf);
        buf.push(b'}');
    }

    fn fmt_csv(&self, output: &mut String) -> fmt::Result {
        let mut json = Vec::new();
        self.write_to(&mut json);
        let json = StdStr::from_utf8(&json).map_err(|_| fmt::Error)?;

        output.push('"');
        for character in json.chars() {
            if character == '"' {
                output.push('"');
            }
            output.push(character);
        }
        output.push('"');
        Ok(())
    }
}

#[cfg(feature = "storage")]
fn write_array<'a, T: Formattable + 'a>(values: impl Iterator<Item = &'a T>, buf: &mut Vec<u8>) {
    buf.push(b'[');
    for (index, value) in values.enumerate() {
        if index > 0 {
            buf.push(b',');
        }
        value.write_to(buf);
    }
    buf.push(b']');
}

/// Every profitability range and aggregate threshold in iteration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ProfitabilityId {
    RangeOver1000PctInProfit,
    Range500To1000PctInProfit,
    Range300To500PctInProfit,
    Range200To300PctInProfit,
    Range100To200PctInProfit,
    Range90To100PctInProfit,
    Range80To90PctInProfit,
    Range70To80PctInProfit,
    Range60To70PctInProfit,
    Range50To60PctInProfit,
    Range40To50PctInProfit,
    Range30To40PctInProfit,
    Range20To30PctInProfit,
    Range10To20PctInProfit,
    Range0To10PctInProfit,
    Range0To10PctInLoss,
    Range10To20PctInLoss,
    Range20To30PctInLoss,
    Range30To40PctInLoss,
    Range40To50PctInLoss,
    Range50To60PctInLoss,
    Range60To70PctInLoss,
    Range70To80PctInLoss,
    Range80To90PctInLoss,
    Range90To100PctInLoss,
    Profit,
    ProfitOver10Pct,
    ProfitOver20Pct,
    ProfitOver30Pct,
    ProfitOver40Pct,
    ProfitOver50Pct,
    ProfitOver60Pct,
    ProfitOver70Pct,
    ProfitOver80Pct,
    ProfitOver90Pct,
    ProfitOver100Pct,
    ProfitOver200Pct,
    ProfitOver300Pct,
    ProfitOver500Pct,
    Loss,
    LossOver10Pct,
    LossOver20Pct,
    LossOver30Pct,
    LossOver40Pct,
    LossOver50Pct,
    LossOver60Pct,
    LossOver70Pct,
    LossOver80Pct,
}

pub const PROFITABILITY_IDS: [ProfitabilityId; PROFITABILITY_COUNT] = [
    ProfitabilityId::RangeOver1000PctInProfit,
    ProfitabilityId::Range500To1000PctInProfit,
    ProfitabilityId::Range300To500PctInProfit,
    ProfitabilityId::Range200To300PctInProfit,
    ProfitabilityId::Range100To200PctInProfit,
    ProfitabilityId::Range90To100PctInProfit,
    ProfitabilityId::Range80To90PctInProfit,
    ProfitabilityId::Range70To80PctInProfit,
    ProfitabilityId::Range60To70PctInProfit,
    ProfitabilityId::Range50To60PctInProfit,
    ProfitabilityId::Range40To50PctInProfit,
    ProfitabilityId::Range30To40PctInProfit,
    ProfitabilityId::Range20To30PctInProfit,
    ProfitabilityId::Range10To20PctInProfit,
    ProfitabilityId::Range0To10PctInProfit,
    ProfitabilityId::Range0To10PctInLoss,
    ProfitabilityId::Range10To20PctInLoss,
    ProfitabilityId::Range20To30PctInLoss,
    ProfitabilityId::Range30To40PctInLoss,
    ProfitabilityId::Range40To50PctInLoss,
    ProfitabilityId::Range50To60PctInLoss,
    ProfitabilityId::Range60To70PctInLoss,
    ProfitabilityId::Range70To80PctInLoss,
    ProfitabilityId::Range80To90PctInLoss,
    ProfitabilityId::Range90To100PctInLoss,
    ProfitabilityId::Profit,
    ProfitabilityId::ProfitOver10Pct,
    ProfitabilityId::ProfitOver20Pct,
    ProfitabilityId::ProfitOver30Pct,
    ProfitabilityId::ProfitOver40Pct,
    ProfitabilityId::ProfitOver50Pct,
    ProfitabilityId::ProfitOver60Pct,
    ProfitabilityId::ProfitOver70Pct,
    ProfitabilityId::ProfitOver80Pct,
    ProfitabilityId::ProfitOver90Pct,
    ProfitabilityId::ProfitOver100Pct,
    ProfitabilityId::ProfitOver200Pct,
    ProfitabilityId::ProfitOver300Pct,
    ProfitabilityId::ProfitOver500Pct,
    ProfitabilityId::Loss,
    ProfitabilityId::LossOver10Pct,
    ProfitabilityId::LossOver20Pct,
    ProfitabilityId::LossOver30Pct,
    ProfitabilityId::LossOver40Pct,
    ProfitabilityId::LossOver50Pct,
    ProfitabilityId::LossOver60Pct,
    ProfitabilityId::LossOver70Pct,
    ProfitabilityId::LossOver80Pct,
];

impl ProfitabilityId {
    pub fn series<T>(mut create: impl FnMut(Self, &'static str) -> T) -> Profitability<T> {
        Profitability {
            range: Self::range_series(&mut create),
            profit: Self::profit_series(&mut create),
            loss: Self::loss_series(create),
        }
    }

    pub fn try_series<T, E>(
        mut create: impl FnMut(Self, &'static str) -> Result<T, E>,
    ) -> Result<Profitability<T>, E> {
        let names = Self::series(|id, name| (id, name));
        Ok(Profitability {
            range: ProfitabilityRange::try_from_fn(|id| {
                let &(id, name) = id.select(&names.range);
                create(id, name)
            })?,
            profit: Profit::try_from_fn(|id| {
                let &(id, name) = id.select(&names.profit);
                create(id, name)
            })?,
            loss: Loss::try_from_fn(|id| {
                let &(id, name) = id.select(&names.loss);
                create(id, name)
            })?,
        })
    }

    pub fn range_ids() -> &'static [Self] {
        &PROFITABILITY_IDS[..PROFITABILITY_RANGE_COUNT]
    }

    pub fn profit_ids() -> &'static [Self] {
        &PROFITABILITY_IDS[PROFITABILITY_RANGE_COUNT..PROFITABILITY_RANGE_COUNT + PROFIT_COUNT]
    }

    pub fn loss_ids() -> &'static [Self] {
        &PROFITABILITY_IDS[PROFITABILITY_RANGE_COUNT + PROFIT_COUNT..]
    }

    pub fn range_series<T>(
        mut create: impl FnMut(Self, &'static str) -> T,
    ) -> ProfitabilityRange<T> {
        ProfitabilityRange::from_fn(|id| {
            create(
                Self::range_ids()[id.index()],
                id.select(ProfitabilityRange::names()).id,
            )
        })
    }

    pub fn profit_series<T>(mut create: impl FnMut(Self, &'static str) -> T) -> Profit<T> {
        Profit::from_fn(|id| {
            create(
                Self::profit_ids()[id.index()],
                id.select(Profit::names()).id,
            )
        })
    }

    pub fn loss_series<T>(mut create: impl FnMut(Self, &'static str) -> T) -> Loss<T> {
        Loss::from_fn(|id| create(Self::loss_ids()[id.index()], id.select(Loss::names()).id))
    }
}

enum ProfitabilityGroupId {
    Range(ProfitabilityRangeId),
    Profit(ProfitId),
    Loss(LossId),
}

impl ProfitabilityId {
    const fn group(self) -> ProfitabilityGroupId {
        use ProfitabilityGroupId::{Loss, Profit, Range};

        match self {
            Self::RangeOver1000PctInProfit => Range(ProfitabilityRangeId::Over1000PctInProfit),
            Self::Range500To1000PctInProfit => {
                Range(ProfitabilityRangeId::From500PctTo1000PctInProfit)
            }
            Self::Range300To500PctInProfit => {
                Range(ProfitabilityRangeId::From300PctTo500PctInProfit)
            }
            Self::Range200To300PctInProfit => {
                Range(ProfitabilityRangeId::From200PctTo300PctInProfit)
            }
            Self::Range100To200PctInProfit => {
                Range(ProfitabilityRangeId::From100PctTo200PctInProfit)
            }
            Self::Range90To100PctInProfit => Range(ProfitabilityRangeId::From90PctTo100PctInProfit),
            Self::Range80To90PctInProfit => Range(ProfitabilityRangeId::From80PctTo90PctInProfit),
            Self::Range70To80PctInProfit => Range(ProfitabilityRangeId::From70PctTo80PctInProfit),
            Self::Range60To70PctInProfit => Range(ProfitabilityRangeId::From60PctTo70PctInProfit),
            Self::Range50To60PctInProfit => Range(ProfitabilityRangeId::From50PctTo60PctInProfit),
            Self::Range40To50PctInProfit => Range(ProfitabilityRangeId::From40PctTo50PctInProfit),
            Self::Range30To40PctInProfit => Range(ProfitabilityRangeId::From30PctTo40PctInProfit),
            Self::Range20To30PctInProfit => Range(ProfitabilityRangeId::From20PctTo30PctInProfit),
            Self::Range10To20PctInProfit => Range(ProfitabilityRangeId::From10PctTo20PctInProfit),
            Self::Range0To10PctInProfit => Range(ProfitabilityRangeId::From0PctTo10PctInProfit),
            Self::Range0To10PctInLoss => Range(ProfitabilityRangeId::From0PctTo10PctInLoss),
            Self::Range10To20PctInLoss => Range(ProfitabilityRangeId::From10PctTo20PctInLoss),
            Self::Range20To30PctInLoss => Range(ProfitabilityRangeId::From20PctTo30PctInLoss),
            Self::Range30To40PctInLoss => Range(ProfitabilityRangeId::From30PctTo40PctInLoss),
            Self::Range40To50PctInLoss => Range(ProfitabilityRangeId::From40PctTo50PctInLoss),
            Self::Range50To60PctInLoss => Range(ProfitabilityRangeId::From50PctTo60PctInLoss),
            Self::Range60To70PctInLoss => Range(ProfitabilityRangeId::From60PctTo70PctInLoss),
            Self::Range70To80PctInLoss => Range(ProfitabilityRangeId::From70PctTo80PctInLoss),
            Self::Range80To90PctInLoss => Range(ProfitabilityRangeId::From80PctTo90PctInLoss),
            Self::Range90To100PctInLoss => Range(ProfitabilityRangeId::From90PctTo100PctInLoss),
            Self::Profit => Profit(ProfitId::Total),
            Self::ProfitOver10Pct => Profit(ProfitId::Over10Pct),
            Self::ProfitOver20Pct => Profit(ProfitId::Over20Pct),
            Self::ProfitOver30Pct => Profit(ProfitId::Over30Pct),
            Self::ProfitOver40Pct => Profit(ProfitId::Over40Pct),
            Self::ProfitOver50Pct => Profit(ProfitId::Over50Pct),
            Self::ProfitOver60Pct => Profit(ProfitId::Over60Pct),
            Self::ProfitOver70Pct => Profit(ProfitId::Over70Pct),
            Self::ProfitOver80Pct => Profit(ProfitId::Over80Pct),
            Self::ProfitOver90Pct => Profit(ProfitId::Over90Pct),
            Self::ProfitOver100Pct => Profit(ProfitId::Over100Pct),
            Self::ProfitOver200Pct => Profit(ProfitId::Over200Pct),
            Self::ProfitOver300Pct => Profit(ProfitId::Over300Pct),
            Self::ProfitOver500Pct => Profit(ProfitId::Over500Pct),
            Self::Loss => Loss(LossId::Total),
            Self::LossOver10Pct => Loss(LossId::Over10Pct),
            Self::LossOver20Pct => Loss(LossId::Over20Pct),
            Self::LossOver30Pct => Loss(LossId::Over30Pct),
            Self::LossOver40Pct => Loss(LossId::Over40Pct),
            Self::LossOver50Pct => Loss(LossId::Over50Pct),
            Self::LossOver60Pct => Loss(LossId::Over60Pct),
            Self::LossOver70Pct => Loss(LossId::Over70Pct),
            Self::LossOver80Pct => Loss(LossId::Over80Pct),
        }
    }
}

impl ProfitabilityId {
    pub const ALL: &'static [Self] = &PROFITABILITY_IDS;

    pub fn select<T>(self, values: &Profitability<T>) -> &T {
        match self.group() {
            ProfitabilityGroupId::Range(id) => id.select(&values.range),
            ProfitabilityGroupId::Profit(id) => id.select(&values.profit),
            ProfitabilityGroupId::Loss(id) => id.select(&values.loss),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profitability_ids_cover_the_schema_in_storage_order() {
        assert_eq!(
            ProfitabilityId::range_ids().len(),
            PROFITABILITY_RANGE_COUNT
        );
        assert_eq!(ProfitabilityId::profit_ids().len(), PROFIT_COUNT);
        assert_eq!(ProfitabilityId::loss_ids().len(), LOSS_COUNT);
        for (index, id) in PROFITABILITY_IDS.into_iter().enumerate() {
            assert_eq!(id as usize, index);
        }

        assert!(
            ProfitabilityId::range_series(|id, _| id)
                .iter()
                .copied()
                .eq(ProfitabilityId::range_ids().iter().copied())
        );
        assert!(
            ProfitabilityId::profit_series(|id, _| id)
                .iter()
                .copied()
                .eq(ProfitabilityId::profit_ids().iter().copied())
        );
        assert!(
            ProfitabilityId::loss_series(|id, _| id)
                .iter()
                .copied()
                .eq(ProfitabilityId::loss_ids().iter().copied())
        );
    }

    #[test]
    fn ranges_expand_into_profit_prefixes_and_loss_suffixes() {
        let ranges = ProfitabilityRange::from_fn(|id| id.index() + 1);
        let values = Profitability::from_ranges(ranges.clone());

        assert_eq!(
            values.profit.total,
            ranges.iter().take(PROFIT_COUNT + 1).copied().sum::<usize>()
        );
        assert_eq!(
            values.profit._500pct,
            ranges.over_1000pct_in_profit + ranges._500pct_to_1000pct_in_profit
        );
        assert_eq!(
            values.loss.total,
            ranges.iter().skip(PROFIT_COUNT + 1).copied().sum::<usize>()
        );
        assert_eq!(
            values.loss._80pct,
            ranges.iter().rev().take(2).copied().sum::<usize>()
        );
    }

    #[test]
    fn aggregate_ids_select_their_exact_ranges() {
        let ranges = ProfitabilityRange::from_fn(|id| id.index() + 1);
        let values = Profitability::from_ranges(ranges.clone());
        for (index, &id) in ProfitabilityId::range_ids().iter().enumerate() {
            assert_eq!(*id.select(&values), index + 1);
        }
        for (threshold, &id) in ProfitabilityId::profit_ids().iter().enumerate() {
            assert_eq!(
                *id.select(&values),
                ranges
                    .iter()
                    .take(PROFIT_COUNT + 1 - threshold)
                    .sum::<usize>()
            );
        }
        for (threshold, &id) in ProfitabilityId::loss_ids().iter().enumerate() {
            assert_eq!(
                *id.select(&values),
                ranges
                    .iter()
                    .skip(PROFIT_COUNT + 1 + threshold)
                    .sum::<usize>()
            );
        }
    }
}
