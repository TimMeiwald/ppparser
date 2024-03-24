use crate::{semantic_instructions, var_name::var_name_decl, whitespace};
use cache::Cache;
use parser_core::{Context, Source, _optional, _sequence, _subexpression, _var_name};
use rules::rules::Rules;

use stack::Stack;

pub fn lhs<T: Cache, S: Stack>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::VarNameDecl, context, var_name_decl);
    let v2 = _var_name(Rules::Whitespace, context, whitespace);
    let v3 = _var_name(Rules::SemanticInstructions, context, semantic_instructions);

    let oc1 = _sequence(&v2, &v3);
    let oc2 = _sequence(&oc1, &v2);
    let sub1 = _subexpression(&oc2);
    let opt1 = _optional(&sub1);
    let s1 = _sequence(&v1, &opt1);
    s1(source, position)
}
