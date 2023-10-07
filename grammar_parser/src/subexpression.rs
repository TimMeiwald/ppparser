use parser_core::{Source, _var_name, _ordered_choice, _subexpression, _sequence};

use crate::{terminal, var_name, whitespace, symbols::{left_bracket, right_bracket}, rhs};

pub fn subexpression(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(left_bracket);
    let v2 = _var_name(rhs);
    let v3 = _var_name(right_bracket);

    let s1 = _sequence(&v1, &v2);
    let s2 = _sequence(&s1, &v3);
 
    s2(source, position)
}