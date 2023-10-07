use parser_core::{Source, _var_name, _sequence, _subexpression, _zero_or_more};

use crate::{symbols::{question_mark, comma, backslash}, nucleus, whitespace, atom};

pub fn ordered_choice(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(atom);
    let v2 = _var_name(whitespace);
    let v3 = _var_name(backslash);
    let s1 = _sequence(&v1, &v2);
    let s2 = _sequence(&s1, &v3);
    let s3 = _sequence(&s2, &v2);
    let s4 = _sequence(&s3, &v1);
    
    let s5 = _sequence(&v3, &v2);
    let s6 = _sequence(&s5, &v1);
    let sub1 = _subexpression(&s6);
    let z1 = _zero_or_more(&sub1);

    let s7 = _sequence(&s4, &z1);
    s7(source, position)
}