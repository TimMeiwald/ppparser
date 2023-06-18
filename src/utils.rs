use std::io::stdin;

pub fn read_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    return input;
}