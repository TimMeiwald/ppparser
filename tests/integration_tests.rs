extern crate ppparser;
use crate::ppparser::parse;
use std::path::Path;
#[test]
fn test_parse_grammar_file() {
    let path = Path::new("./src/Grammar.txt");
    let (bool, position, usize) = parse(path);
    println!("{:?}, {:?}", bool, position);
    assert_eq!(bool, true);
    assert_eq!(position, usize as u32);

}
