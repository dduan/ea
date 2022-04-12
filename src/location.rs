use serde::{Serialize, Deserialize};
use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub path: String,
    pub line: Option<u64>,
    pub column: Option<u64>,
}
