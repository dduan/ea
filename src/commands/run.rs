use bincode;
use crate::location::Location;
use pty::fork::Fork;
use std::io::{Write, Read};
use std::process::Command;
use std;
use std::fs;

pub fn run(executable: &str, arguments: &Vec<String>) {
    let fork = Fork::from_ptmx().unwrap();

    if let Some(mut parent) = fork.is_parent().ok() {
        let mut output = String::new();
        match parent.read_to_string(&mut output) {
            Ok(_) => println!("{}", output),
            Err(e) => panic!("read error: {}", e),
        }
    } else {
        Command::new(executable)
            .args(arguments)
            .status()
            .expect(concat!("could not execute", stringify!(executable)));

        let loc = vec![Location { path: "hello world".to_string(), line: None, column: None }];
        let encoded: Vec<u8> = bincode::serialize(&loc).unwrap();
        {
            let _ = std::fs::write("/tmp/ea", &encoded);
        }

        {
            let buf = std::fs::read("/tmp/ea").unwrap();
            let decoded: Vec<Location> = bincode::deserialize(&buf).unwrap();
            println!("{:?}", decoded);
        }
    }
}
