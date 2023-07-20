extern crate parser;
use parser::parser::{Lr, B};

use crate::parser::*;
use std::path::Path;
#[test]
fn test_parse_grammar_file() {
    let path = Path::new("./src/Grammar.txt");
    let (bool, position, usize, _source, _stack) = parse(path);
    println!("{:?}, {:?}", bool, position);
    assert_eq!(bool, true);
    assert_eq!(position, usize as i32);
}



#[test]
fn test_direct_left_recursion_single_char(){
    let source = "A".to_string();
    let (bool, position) = parse_string(source, &Lr);
    println!("{:?}, {:?}", bool, position);
    assert_eq!(bool, true);
    assert_eq!(position, 3);
}
#[test]
fn test_direct_left_recursion(){
    let source = "AA".to_string();
    let (bool, position) = parse_string(source, &Lr);
    println!("{:?}, {:?}", bool, position);
    assert_eq!(bool, true);
    assert_eq!(position, 4);
}
#[test]
fn test_direct_left_recursion_many_chars(){
    let source = "AAAAAA".to_string();
    let (bool, position) = parse_string(source, &Lr);
    println!("{:?}, {:?}", bool, position);
    assert_eq!(bool, true);
    assert_eq!(position, 6);
}



#[test]
fn test_indirect_left_recursion_single_char(){
    let source = "a".to_string();
    let (bool, position) = parse_string(source, &B);
    println!("{:?}, {:?}", bool, position);
    assert_eq!(bool, true);
    assert_eq!(position, 3);
}
#[test]
fn test_indirect_left_recursion(){
    let source = "aba".to_string();
    let (bool, position) = parse_string(source, &B);
    println!("{:?}, {:?}", bool, position);
    assert_eq!(bool, true);
    assert_eq!(position, 4);
}
#[test]
fn test_indirect_left_recursion_many_chars(){
    let source = "ababa".to_string();
    let (bool, position) = parse_string(source, &B);
    println!("{:?}, {:?}", bool, position);
    assert_eq!(bool, true);
    assert_eq!(position, 6);
}