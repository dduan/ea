use atty::Stream;
use std::io;

fn echo() {
    loop {
        let mut input = String::new();
        if let Ok(count) = io::stdin().read_line(&mut input) {
            if count == 0 {
                return;
            }

            print!("{}", input);
        }
    }
}

fn main() -> io::Result<()> {
    if atty::is(Stream::Stdin) {
        println!("hello");
    } else {
        echo()
    }

    Ok(())
}
