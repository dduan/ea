use crate::archive;

pub fn print(number: usize, format: &str) {
    let mut result = format.to_string();
    let list = archive::read();
    if number <= list.len() {
        let location = &list[number-1];
        result = result.replace("{path}", &location.path);
        result = result.replace("{line}", &location.line.unwrap_or(0).to_string());
        result = result.replace("{column}", &location.column.unwrap_or(0).to_string());

        println!("{}", &result);
    }
}
