use cache::Cache;
use parser_core::{Context, Rules};
use parser_core::{Source, _sequence, _var_name};

use crate::{
    rhs,
    symbols::{left_bracket, right_bracket},
};

pub fn subexpression<T: Cache>(
    context: &Context<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::LeftBracket, context, left_bracket);
    let v2 = _var_name(Rules::Rhs, context, rhs);
    let v3 = _var_name(Rules::RightBracket, context, right_bracket);

    let s1 = _sequence(&v1, &v2);
    let s2 = _sequence(&s1, &v3);

    s2(source, position)
}
