use cache::Cache;
use parser_core::{Context, Rules};
use parser_core::{Source, _sequence, _var_name};

use crate::{nucleus, symbols::question_mark, whitespace};

pub fn optional<T: Cache>(context: &Context<T>, source: &Source, position: u32) -> (bool, u32) {
    let v1 = _var_name(Rules::Nucleus, context, nucleus);
    let v2 = _var_name(Rules::Whitespace, context, whitespace);
    let v3 = _var_name(Rules::QuestionMark, context, question_mark);
    let s1 = _sequence(&v1, &v2);
    let s2 = _sequence(&s1, &v3);
    s2(source, position)
}
