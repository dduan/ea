use crate::location::Location;
use lazy_static::lazy_static;
use regex::Regex;
use std::option::Option;

pub fn ripgrep(input: &[u8]) -> (Vec<u8>, Vec<Location>) {
    lazy_static! {
        static ref RE_PATH: Regex = Regex::new(r#"^\x1B\[0m\x1B\[\d+?m(.+?)\x1B\[0m\n"#).unwrap();
        static ref RE_LINE: Regex =
            Regex::new(r#"^\x1B\[0m\x1B\[\d+?m(\d+?)\x1B\[0m:.+?\n"#).unwrap();
    }

    let input_str = std::str::from_utf8(input).unwrap();

    //let mut file: Option<&[u8]> = None;
    //for line in input_str.split('\n') {
    //    println!("{}", line);
    //}

    (vec![], vec![])
    //unimplemented!()
}
