mod archive;
mod parsers;
pub mod commands;
pub mod interface;

use serde::{Deserialize, Serialize};
use std::option::Option;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd)]
pub struct Location {
    pub path: String,
    pub line: Option<u64>,
    pub column: Option<u64>,
}
