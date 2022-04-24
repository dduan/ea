use crate::location::Location;
use bincode;
use lazy_static::lazy_static;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
lazy_static! {
    static ref ARCHIVE_PATH: PathBuf = [
        "/tmp".to_string(),
        format!(
            "ea_aliases_{}.bin",
            env::var("USERNAME").unwrap_or_else(|_| "".to_string())
        ),
    ]
    .iter()
    .collect();
}

#[cfg(target_os = "windows")]
lazy_static! {
    static ref ARCHIVE_PATH: PathBuf = [
        env::var("TEMP").unwrap_or(env::var("HOME").unwrap_or_else(|_| r".".to_string())),
        format!(
            "ea_{}.bin",
            env::var("USERNAME").unwrap_or_else(|_| "".to_string())
        ),
    ]
    .iter()
    .collect();
}

pub fn write(list: &[Location]) -> io::Result<()> {
    let data: Vec<u8> = bincode::serialize(list).unwrap_or_default();
    fs::write(ARCHIVE_PATH.as_path(), &data)
}

pub fn read() -> Vec<Location> {
    let data: Vec<u8> = fs::read(ARCHIVE_PATH.as_path()).unwrap_or_default();
    bincode::deserialize(&data).unwrap_or_default()
}
