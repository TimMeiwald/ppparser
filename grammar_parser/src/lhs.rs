use parser_core::{Source, _ordered_choice, _var_name, _optional, _sequence, _subexpression};

use crate::{var_name, whitespace, semantic_instructions};


pub fn lhs(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(var_name);
    let v2 = _var_name(whitespace);
    let v3 = _var_name(semantic_instructions);

    let oc1 = _sequence(&v2, &v3);
    let oc2 = _sequence(&oc1, &v2);
    let sub1 = _subexpression(&oc2);
    let opt1 = _optional(&sub1);
    let s1 = _sequence(&v1, &opt1);
    s1(source, position)
}
