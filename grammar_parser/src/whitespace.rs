use cache::Cache;
use parser_core::Context;
use parser_core::{Source, _ordered_choice, _subexpression, _terminal, _zero_or_more};
use stack::Stack;

pub fn whitespace<T: Cache, S: Stack>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let t1 = _terminal(b' ');
    let t2 = _terminal(b'\n');
    let t3 = _terminal(b'\r');
    let oc1 = _ordered_choice(&t1, &t2);
    let oc2 = _ordered_choice(&oc1, &t3);
    let sub1 = _subexpression(&oc2);
    let z1 = _zero_or_more(&sub1);
    z1(source, position)
}
