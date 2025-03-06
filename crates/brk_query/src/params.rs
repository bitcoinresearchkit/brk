use clap::{Parser, builder::PossibleValuesParser};
use serde::Deserialize;
use serde_with::{OneOrMany, formats::PreferOne, serde_as};

use crate::{Format, Index};

#[serde_as]
#[derive(Debug, Deserialize, Parser)]
pub struct Params {
    #[clap(short, long, value_parser = PossibleValuesParser::new(Index::all_possible_values()))]
    #[serde(alias = "i")]
    /// Index of the values requested
    pub index: String,
    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    #[serde(alias = "v")]
    #[serde_as(as = "OneOrMany<_, PreferOne>")]
    /// Names of the values requested
    pub values: Vec<String>,
    #[clap(short, long, allow_hyphen_values = true)]
    #[serde(alias = "f")]
    /// Inclusive starting index, if negative will be from the end
    pub from: Option<i64>,
    #[clap(short, long, allow_hyphen_values = true)]
    #[serde(default, alias = "t")]
    /// Inclusive ending index, if negative will be from the end
    pub to: Option<i64>,
    #[clap(short = 'F', long)]
    /// Format of the output
    pub format: Option<Format>,
}
