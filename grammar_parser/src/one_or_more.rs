use cache::Cache;
use parser_core::{Context, Rules};
use parser_core::{Source, _sequence, _var_name};
use stack::Stack;

use crate::{nucleus, symbols::plus, whitespace};

pub fn one_or_more<T: Cache, S: Stack>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::Nucleus, context, nucleus);
    let v2 = _var_name(Rules::Whitespace, context, whitespace);
    let v3 = _var_name(Rules::Plus, context, plus);
    let s1 = _sequence(&v1, &v2);
    let s2 = _sequence(&s1, &v3);
    s2(source, position)
}
