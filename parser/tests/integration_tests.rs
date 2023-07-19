extern crate parser;
use parser::parser::Lr;

use crate::parser::*;
use std::path::Path;
#[test]
fn test_parse_grammar_file() {
    let path = Path::new("./src/Grammar.txt");
    let (bool, position, usize, _source, _stack) = parse(path);
    println!("{:?}, {:?}", bool, position);
    assert_eq!(bool, true);
    assert_eq!(position, usize as u32);
}



#[test]
fn test_left_recursion(){
    let source = "AAB".to_string();
    let (bool, position) = parse_string(source, &Lr);
    println!("{:?}, {:?}", bool, position);
    assert_eq!(bool, true);
    assert_eq!(position, 3);
}