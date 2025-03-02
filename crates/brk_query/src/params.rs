use clap::{Parser, builder::PossibleValuesParser};
use serde::Deserialize;

use crate::{Format, Index};

#[derive(Debug, Deserialize, Parser)]
pub struct Params {
    #[clap(short, long, value_parser = PossibleValuesParser::new(Index::ids()))]
    pub index: String,
    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    pub values: Vec<String>,
    #[clap(short, long, allow_hyphen_values = true)]
    pub from: Option<i64>,
    #[clap(short, long, allow_hyphen_values = true)]
    pub to: Option<i64>,
    #[clap(long)]
    pub format: Option<Format>,
}
