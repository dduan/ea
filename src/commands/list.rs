use crate::archive;

pub fn list() {
    for (idx, location) in archive::read().iter().enumerate() {
        println!("[{}] {:?}", idx, location);
    }
}
