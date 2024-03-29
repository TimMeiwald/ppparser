use cache::Cache;
use parser_core::{Context, Rules};
use parser_core::{Source, _sequence, _var_name};
use stack::Stack;

use crate::{nucleus, symbols::ampersand};

pub fn and_predicate<T: Cache, S: Stack>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::Ampersand, context, ampersand);
    let v2 = _var_name(Rules::Nucleus, context, nucleus);
    let s1 = _sequence(&v1, &v2);
    s1(source, position)
}
