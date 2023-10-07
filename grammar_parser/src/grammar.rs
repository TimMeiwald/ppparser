use parser_core::{Source, _one_or_more, _sequence, _var_name};

use crate::{rule, whitespace};


pub fn grammar(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(rule);
    let v2 = _var_name(whitespace);

    let one1 = _one_or_more(&v1);
    let s1 = _sequence(&one1, &v2);
    s1(source, position)
}
