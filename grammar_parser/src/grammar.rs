use parser_core::{Source, _one_or_more, _sequence, _var_name};

use crate::{rule, whitespace};


pub fn grammar(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(rule);
    let v2 = _var_name(whitespace);

    let one1 = _one_or_more(&v1);
    let s1 = _sequence(&one1, &v2);
    s1(source, position)
}

#[cfg(test)]
mod tests {
use parser_core::Source;
use super::*;

#[test]
fn test_grammar_true() {
    let string = "<Spaces> PASSTHROUGH = \"\n\"/\"\t\"/\"\r\"/\" \";".to_string();
    let str_len =string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let result = grammar(&source, position);
    assert_eq!(result, (true, str_len));
}
#[test]
fn test_grammar_true2() {
    let string = "<Specials> PASSTHROUGH = \"+\"/\"*\"/\"-\"/\"&\"/\"!\"/\"?\"/\"<\"/\">\"/\"\"\"/\"(\"/\")\"/\"_\"/\",\"/\"/\"/\";\"/\"=\"/\"\\\"/\"#\"/\":\"/\"|\"/\".\"/\"{\"/\"}\"/\"[\"/\"]\"/\"%\"/\"'\"/\"^\"/\"~\";".to_string();
    let str_len =string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let result = rule(&source, position);
    assert_eq!(result, (true, str_len));
}
#[test]
fn test_grammar_true3() {
    let string = "<Alphabet_Upper> PASSTHROUGH = \"A\"/\"B\"/\"C\"/\"D\"/\"E\"/\"F\"/\"G\"/\"H\"/\"I\"/\"J\"/\"K\"/\"L\"/\"M\"/\"N\"/\"O\"/\"P\"/\"Q\"/\"R\"/\"S\"/\"T\"/\"U\"/\"V\"/\"W\"/\"X\"/\"Y\"/\"Z\"; #We all love commments#
    <Alphabet_Lower> PASSTHROUGH =\"a\"/\"b\"/\"c\"/\"d\"/\"e\"/\"f\"/\"g\"/\"h\"/\"i\"/\"j\"/\"k\"/\"l\"/\"m\"/\"n\"/\"o\"/\"p\"/\"q\"/\"r\"/\"s\"/\"t\"/\"u\"/\"v\"/\"w\"/\"x\"/\"y\"/\"z\";
    <Num> PASSTHROUGH = \"0\"/\"1\"/\"2\"/\"3\"/\"4\"/\"5\"/\"6\"/\"7\"/\"8\"/\"9\";
    <Spaces> PASSTHROUGH = \"\n\"/\"\t\"/\"\r\"/\" \";
    <Specials> PASSTHROUGH = \"+\"/\"*\"/\"-\"/\"&\"/\"!\"/\"?\"/\"<\"/\">\"/\"\"\"/\"(\"/\")\"/\"_\"/\",\"/\"/\"/\";\"/\"=\"/\"\\\"/\"#\"/\":\"/\"|\"/\".\"/\"{\"/\"}\"/\"[\"/\"]\"/\"%\"/\"'\"/\"^\"/\"~\";
    <ASCII> PASSTHROUGH = <Alphabet_Lower>/<Alphabet_Upper>/<Num>/<Spaces>/<Specials>;".to_string();
    let str_len =string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let result = rule(&source, position);
    assert_eq!(result, (true, str_len));
}
}