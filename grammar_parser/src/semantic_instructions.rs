use cache::Cache;
use parser_core::Context;
use parser_core::{Source, _var_name, _sequence, _subexpression, _zero_or_more, _terminal, _ordered_choice};
use parser_core::Rules;


pub fn semantic_instructions(context: &Context, source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(Rules::Delete, &context, delete);
    let v2 = _var_name(Rules::Passthrough, &context, passthrough);
    let v3 = _var_name(Rules::Collect, &context, collect);
    let s1 = _ordered_choice(&v1, &v2);
    let s2 = _ordered_choice(&s1, &v3);
    s2(source, position)
}

pub fn collect(context: &Context, source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal('C' as u8);
    let t2 = _terminal('O' as u8);
    let t3 = _terminal('L' as u8);
    let t4 = _terminal('L' as u8);
    let t5 = _terminal('E' as u8);
    let t6 = _terminal('C' as u8);
    let t7 = _terminal('T' as u8);
    let s1 = _sequence(&t1, &t2);
    let s2 = _sequence(&s1, &t3);
    let s3 = _sequence(&s2, &t4);
    let s4 = _sequence(&s3, &t5);
    let s5 = _sequence(&s4, &t6);
    let s6 = _sequence(&s5, &t7);
    s6(source, position)
}

pub fn delete(context: &Context, source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal('D' as u8);
    let t2 = _terminal('E' as u8);
    let t3 = _terminal('L' as u8);
    let t4 = _terminal('E' as u8);
    let t5 = _terminal('T' as u8);
    let t6 = _terminal('E' as u8);
    let s1 = _sequence(&t1, &t2);
    let s2 = _sequence(&s1, &t3);
    let s3 = _sequence(&s2, &t4);
    let s4 = _sequence(&s3, &t5);
    let s5 = _sequence(&s4, &t6);
    s5(source, position)
}

pub fn passthrough(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal('P' as u8);
    let t2 = _terminal('A' as u8);
    let t3 = _terminal('S' as u8);
    let t4 = _terminal('S' as u8);
    let t5 = _terminal('T' as u8);
    let t6 = _terminal('H' as u8);
    let t7 = _terminal('R' as u8);
    let t8 = _terminal('O' as u8);
    let t9 = _terminal('U' as u8);
    let t10 = _terminal('G' as u8);
    let t11 = _terminal('H' as u8);
    let s1 = _sequence(&t1, &t2);
    let s2 = _sequence(&s1, &t3);
    let s3 = _sequence(&s2, &t4);
    let s4 = _sequence(&s3, &t5);
    let s5 = _sequence(&s4, &t6);
    let s6 = _sequence(&s5, &t7);
    let s7 = _sequence(&s6, &t8);
    let s8 = _sequence(&s7, &t9);
    let s9 = _sequence(&s8, &t10);
    let s10 = _sequence(&s9, &t11);
    s10(source, position)
}

#[cfg(test)]
mod tests {
use std::{rc::Rc, cell::RefCell};

use parser_core::Source;
use super::*;
use cache::{Cache, BTreeCache};

#[test]
fn test_semantic_instruction_true() {
    let string = "PASSTHROUGH".to_string();
    let str_len =string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);
    let result = semantic_instructions(&context,&source, position);
    assert_eq!(result, (true, str_len));
}
#[test]
fn test_semantic_instruction_true_cache() {
    let string = "PASSTHROUGH".to_string();
    let str_len =string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);
    let result = semantic_instructions(&context,&source, position);
    assert_eq!(result, (true, str_len), "1");
    let result = semantic_instructions(&context,&source, position);
    println!("{:?}, {:?}", result.0, result.1);
    assert_eq!(result, (true, str_len), "2");
}



}