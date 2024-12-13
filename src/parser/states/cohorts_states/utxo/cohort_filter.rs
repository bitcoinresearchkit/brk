use crate::structs::{Epoch, Height};

pub enum UTXOFilter {
    To(u32),
    FromTo { from: u32, to: u32 },
    From(u32),
    Epoch(Epoch),
}

impl UTXOCheck for UTXOFilter {
    fn check(&self, days_old: &u32, height: &Height) -> bool {
        match self {
            UTXOFilter::From(from) => from <= days_old,
            UTXOFilter::To(to) => to > days_old,
            UTXOFilter::FromTo { from, to } => from <= days_old && to > days_old,
            UTXOFilter::Epoch(epoch) => *epoch == height.into(),
        }
    }

    fn check_days_old(&self, days_old: &u32) -> bool {
        match self {
            UTXOFilter::From(from) => from <= days_old,
            UTXOFilter::To(to) => to > days_old,
            UTXOFilter::FromTo { from, to } => from <= days_old && to > days_old,
            UTXOFilter::Epoch(_) => unreachable!(),
        }
    }
}

pub trait UTXOCheck {
    fn check(&self, days_old: &u32, height: &Height) -> bool;

    fn check_days_old(&self, days_old: &u32) -> bool;
}
