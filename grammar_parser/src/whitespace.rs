use cache::Cache;
use parser_core::{Context, _var_name};
use parser_core::{Source, _ordered_choice, _subexpression, _terminal, _zero_or_more};
use stack::Stack;
use rules::rules::Rules;

pub fn whitespace<T: Cache, S: Stack>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let t1 = _terminal(b' ');
    let t2 = _var_name(Rules::NewLine, context, newline);
    let t3 = _terminal(b'\r');
    let oc1 = _ordered_choice(&t1, &t2);
    let oc2 = _ordered_choice(&oc1, &t3);
    let sub1 = _subexpression(&oc2);
    let z1 = _zero_or_more(&sub1);
    z1(source, position)
}

pub fn newline<T: Cache, S: Stack>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let t2 = _terminal(b'\n');
    t2(source, position)
}
