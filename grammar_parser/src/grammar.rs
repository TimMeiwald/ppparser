use parser_core::{Source, _one_or_more, _sequence, _var_name};
use parser_core::{Context, Rules};

use crate::{rule, whitespace};


pub fn grammar(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(Rules::Rule, &context,rule);
    let v2 = _var_name(Rules::Whitespace, &context,whitespace);

    let one1 = _one_or_more(&v1);
    let s1 = _sequence(&one1, &v2);
    s1(source, position)
}

#[cfg(test)]
mod tests {
use parser_core::Source;
use super::*;
use std::fs::{canonicalize, read_to_string};
use std::env;
use std::str;
#[test]
fn test_grammar_true() {
    let string = "<Spaces> PASSTHROUGH = \"\n\"/\"\t\"/\"\r\"/\" \";".to_string();
    let str_len =string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);

    let result = grammar(&context, &source, position);
    assert_eq!(result, (true, str_len));
}
#[test]
fn test_grammar_true2() {
    let string = "<Specials> PASSTHROUGH = \"+\"/\"*\"/\"-\"/\"&\"/\"!\"/\"?\"/\"<\"/\">\"/\"\"\"/\"(\"/\")\"/\"_\"/\",\"/\"/\"/\";\"/\"=\"/\"\\\"/\"#\"/\":\"/\"|\"/\".\"/\"{\"/\"}\"/\"[\"/\"]\"/\"%\"/\"'\"/\"^\"/\"~\";".to_string();
    let str_len =string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, str_len));
}
#[test]
fn test_grammar_true3() {
    println!("{:?}", env::current_dir().unwrap());
    let path = "../parser_core/tests/Grammar.txt";
    let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    let string = read_to_string(pathbuf).expect("If it's moved change the string above");

    let str_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);

    let result = grammar(&context, &source, position);
    assert_eq!(result, (true, str_len));
}
}