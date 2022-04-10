use crate::archive;
use crate::location::Location;
use pty::fork::Fork;
use std;
use std::io::Read;
use std::process::Command;

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

        let locs = vec![Location {
            path: "hello world".to_string(),
            line: None,
            column: None,
        }];

        _ = archive::write(&locs);
        let found = archive::read();
        println!("{:?}", found);
    }
}
