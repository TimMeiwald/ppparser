use parser_core::{Source, _var_name, _ordered_choice, _subexpression, _sequence};

use crate::{terminal, var_name, whitespace, subexpression};

pub fn nucleus(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(subexpression);
    let v2 = _var_name(terminal);
    let v3 = _var_name(var_name);
    let v4 = _var_name(whitespace);

    let oc1 = _ordered_choice(&v1, &v2);
    let oc2 = _ordered_choice(&oc1, &v3);
    let sub1 = _subexpression(&oc2);
    
    let s1 = _sequence(&sub1, &v4);
    s1(source, position)
}