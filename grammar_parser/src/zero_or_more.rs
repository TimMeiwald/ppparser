use cache::Cache;
use parser_core::{Source, _var_name, _sequence};

use crate::{symbols::{plus, star}, nucleus, whitespace};

pub fn zero_or_more(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(nucleus);
    let v2 = _var_name(whitespace);
    let v3 = _var_name(star);
    let s1 = _sequence(&v1, &v2);
    let s2 = _sequence(&s1, &v3);
    s2(source, position)
}