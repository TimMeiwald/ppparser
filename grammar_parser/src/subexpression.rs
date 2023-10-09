use parser_core::{Source, _var_name, _ordered_choice, _subexpression, _sequence};
use parser_core::{Context, Rules};

use crate::{terminal, var_name, whitespace, symbols::{left_bracket, right_bracket}, rhs};

pub fn subexpression(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(Rules::LeftBracket, &context,left_bracket);
    let v2 = _var_name(Rules::Rhs, &context,rhs);
    let v3 = _var_name(Rules::RightBracket, &context,right_bracket);

    let s1 = _sequence(&v1, &v2);
    let s2 = _sequence(&s1, &v3);
 
    s2(source, position)
}