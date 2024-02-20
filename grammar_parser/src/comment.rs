use cache::Cache;
use parser_core::{Context, Rules};
use parser_core::{
    Source, _not_predicate, _sequence, _subexpression, _terminal, _var_name, _zero_or_more,
};
use stack::Stack;

use crate::{ascii, whitespace};

pub fn comment<T: Cache, S: Stack>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::Whitespace, context, whitespace);
    let v2 = _var_name(Rules::Ascii, context, ascii);
    let t1 = _terminal(b'#');

    let not1 = _not_predicate(&t1);
    let s1 = _sequence(&not1, &v2);
    let sub1 = _subexpression(&s1);
    let z1 = _zero_or_more(&sub1);

    let s2 = _sequence(&v1, &t1);
    let s3 = _sequence(&s2, &z1);
    let s4 = _sequence(&s3, &t1);
    let s5 = _sequence(&s4, &v1);
    s5(source, position)
}
