use parser_core::{Context, Rules};
use parser_core::{Source, _sequence, _var_name};

use crate::{nucleus, symbols::exclamation_mark};

pub fn not_predicate(context: &Context, source: &Source, position: u32) -> (bool, u32) {
    let v1 = _var_name(Rules::ExclamationMark, context, exclamation_mark);
    let v2 = _var_name(Rules::Nucleus, context, nucleus);
    let s1 = _sequence(&v1, &v2);
    s1(source, position)
}
