use parser_core::{Source, _var_name, _sequence, _subexpression, _zero_or_more};

use crate::{symbols::{question_mark, comma, backslash, assignment, end_rule}, nucleus, whitespace, atom, sequence, ordered_choice, lhs, rhs, comment};

pub fn rule(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(lhs);
    let v2 = _var_name(whitespace);
    let v3 = _var_name(assignment);
    let v4 = _var_name(rhs);
    let v5 = _var_name(end_rule);
    let v6 = _var_name(comment);
    let z1 = _zero_or_more(&v6);
    

    let s1 = _sequence(&v1, &v2);
    let s2 = _sequence(&s1, &v3);
    let s3 = _sequence(&s2, &v2);
    let s4 = _sequence(&s3, &v4);
    let s5 = _sequence(&s4, &v2);
    let s6 = _sequence(&s5, &v5);
    let s7 = _sequence(&s6, &v2);
    let s8 = _sequence(&s7, &z1);

    s8(source, position)
}