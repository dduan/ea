use crate::archive;

pub fn list() {
    for (idx, location) in archive::read().iter().enumerate() {
        println!("[\x1b[0m\x1b[31m{}\x1b[0m] {:?}", idx + 1, location);
    }
}
