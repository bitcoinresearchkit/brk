use clap::{Parser, builder::PossibleValuesParser};
use serde::Deserialize;

use crate::{Format, Index};

#[derive(Debug, Deserialize, Parser)]
pub struct Params {
    #[clap(short, long, value_parser = PossibleValuesParser::new(Index::all_possible_values()))]
    /// Index of the values requested
    pub index: String,
    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    /// Names of the values requested
    pub values: Vec<String>,
    #[clap(short, long, allow_hyphen_values = true)]
    /// Inclusive starting index, if negative will be from the end
    pub from: Option<i64>,
    #[clap(short, long, allow_hyphen_values = true)]
    /// Inclusive ending index, if negative will be from the end
    pub to: Option<i64>,
    #[clap(long)]
    /// Format of the output
    pub format: Option<Format>,
}
