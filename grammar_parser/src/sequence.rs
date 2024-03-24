use cache::Cache;
use parser_core::{Context};
use rules::rules::Rules;

use parser_core::{Source, _sequence, _subexpression, _var_name, _zero_or_more};
use stack::Stack;

use crate::{atom, symbols::comma, whitespace};

pub fn sequence<T: Cache, S: Stack>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::Atom, context, atom);
    let v2 = _var_name(Rules::Whitespace, context, whitespace);
    let v3 = _var_name(Rules::Comma, context, comma);
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
