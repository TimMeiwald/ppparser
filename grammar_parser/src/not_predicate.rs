use parser_core::{Source, _var_name, _sequence};

use crate::{symbols::{ampersand, exclamation_mark}, nucleus};

pub fn not_predicate(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(exclamation_mark);
    let v2 = _var_name(nucleus);
    let s1 = _sequence(&v1, &v2);
    s1(source, position)
}