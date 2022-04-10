use std::option::Option;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub path: String,
    pub line: Option<u64>,
    pub column: Option<u64>,
}
