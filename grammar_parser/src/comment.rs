use parser_core::{Source, _subexpression, _zero_or_more, _not_predicate, _sequence, _var_name, _terminal};

use crate::{whitespace, ascii, subexpression::subexpression};



pub fn comment(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(whitespace);
    let v2 = _var_name(ascii);
    let t1 = _terminal('#' as u8);

    let not1 = _not_predicate(&t1);
    let s1 = _sequence(&not1, &v2);
    let sub1 = _subexpression(&s1);
    let z1 = _zero_or_more(&sub1);
    
    let s2 = _sequence(&v1, &t1);
    let s3 = _sequence(&s2, &z1);
    let s4 = _sequence(&s3, &t1);
    let s5 = _sequence(&s4, &v1);
    s5(source, position)
}
