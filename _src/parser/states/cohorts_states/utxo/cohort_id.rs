use allocative::Allocative;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Allocative)]
pub enum UTXOCohortId {
    #[default]
    UpTo1d,
    UpTo1w,
    UpTo1m,
    UpTo2m,
    UpTo3m,
    UpTo4m,
    UpTo5m,
    UpTo6m,
    UpTo1y,
    UpTo2y,
    UpTo3y,
    UpTo5y,
    UpTo7y,
    UpTo10y,
    UpTo15y,

    From1dTo1w,
    From1wTo1m,
    From1mTo3m,
    From3mTo6m,
    From6mTo1y,
    From1yTo2y,
    From2yTo3y,
    From3yTo5y,
    From5yTo7y,
    From7yTo10y,
    From10yTo15y,

    From1y,
    From2y,
    From4y,
    From10y,
    From15y,

    Epoch1,
    Epoch2,
    Epoch3,
    Epoch4,
    Epoch5,

    ShortTermHolders,
    LongTermHolders,
}

impl UTXOCohortId {
    pub fn name(&self) -> &str {
        match self {
            Self::UpTo1d => "up_to_1d",
            Self::UpTo1w => "up_to_1w",
            Self::UpTo1m => "up_to_1m",
            Self::UpTo2m => "up_to_2m",
            Self::UpTo3m => "up_to_3m",
            Self::UpTo4m => "up_to_4m",
            Self::UpTo5m => "up_to_5m",
            Self::UpTo6m => "up_to_6m",
            Self::UpTo1y => "up_to_1y",
            Self::UpTo2y => "up_to_2y",
            Self::UpTo3y => "up_to_3y",
            Self::UpTo5y => "up_to_5y",
            Self::UpTo7y => "up_to_7y",
            Self::UpTo10y => "up_to_10y",
            Self::UpTo15y => "up_to_15y",

            Self::From1dTo1w => "from_1d_to_1w",
            Self::From1wTo1m => "from_1w_to_1m",
            Self::From1mTo3m => "from_1m_to_3m",
            Self::From3mTo6m => "from_3m_to_6m",
            Self::From6mTo1y => "from_6m_to_1y",
            Self::From1yTo2y => "from_1y_to_2y",
            Self::From2yTo3y => "from_2y_to_3y",
            Self::From3yTo5y => "from_3y_to_5y",
            Self::From5yTo7y => "from_5y_to_7y",
            Self::From7yTo10y => "from_7y_to_10y",
            Self::From10yTo15y => "from_10y_to_15y",

            Self::From1y => "from_1y",
            Self::From2y => "from_2y",
            Self::From4y => "from_4y",
            Self::From10y => "from_10y",
            Self::From15y => "from_15y",

            Self::Epoch1 => "epoch_1",
            Self::Epoch2 => "epoch_2",
            Self::Epoch3 => "epoch_3",
            Self::Epoch4 => "epoch_4",
            Self::Epoch5 => "epoch_5",

            Self::ShortTermHolders => "sth",
            Self::LongTermHolders => "lth",
        }
    }
}
