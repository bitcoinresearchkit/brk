use clap_derive::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    Parser,
    ValueEnum,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum Services {
    #[default]
    All,
    Processor,
    Server,
}
