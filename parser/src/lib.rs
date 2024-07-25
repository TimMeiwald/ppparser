mod parse;
use parse::parse;
use std::env;
#[test]
fn test_basic_publisher() {
    println!("{:?}", env::current_dir().unwrap());
    // This file won't be valid for a different type of parser obviously
    let path = "../parser_core/tests/Grammar.txt"; // Switch to some other file location for tests if you wish
    let success = parse(path);
    assert!(success.unwrap())
}
