use atty;
use crate::archive;

pub fn list() {
    for (idx, location) in archive::read().iter().enumerate() {
        if atty::is(atty::Stream::Stdout) {
            println!("[\x1b[0m\x1b[31m{}\x1b[0m] {:?}", idx + 1, location);
        } else {
            println!("[{}] {:?}", idx + 1, location);
        }
    }
}
