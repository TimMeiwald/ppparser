use parser_core::{Source, _var_name, _ordered_choice, _subexpression, _sequence};
use parser_core::{Context, Rules};

use crate::{terminal, var_name, whitespace, subexpression};

pub fn nucleus(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(Rules::Subexpression, &context,subexpression);
    let v2 = _var_name(Rules::Lterminal, &context,terminal);
    let v3 = _var_name(Rules::VarName, &context,var_name);
    let v4 = _var_name(Rules::Whitespace, &context,whitespace);

    let oc1 = _ordered_choice(&v1, &v2);
    let oc2 = _ordered_choice(&oc1, &v3);
    let sub1 = _subexpression(&oc2);
    
    let s1 = _sequence(&sub1, &v4);
    s1(source, position)
}