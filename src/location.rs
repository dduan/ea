use serde::{Deserialize, Serialize};
use std::option::Option;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Location {
    pub path: String,
    pub line: Option<u64>,
    pub column: Option<u64>,
}
