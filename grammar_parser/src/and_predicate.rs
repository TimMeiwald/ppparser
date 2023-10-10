use parser_core::{Source, _var_name, _sequence};
use parser_core::{Context, Rules};

use crate::{symbols::ampersand, nucleus};

pub fn and_predicate(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(Rules::Ampersand, &context,ampersand);
    let v2 = _var_name(Rules::Nucleus, &context,nucleus);
    let s1 = _sequence(&v1, &v2);
    s1(source, position)
}