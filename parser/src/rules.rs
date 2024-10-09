#![allow(non_camel_case_types)] // Again due to generation -> Might solve eventually

use num_derive::FromPrimitive;
impl From<u32> for Rules {
    fn from(i: u32) -> Rules {
        let element = num::FromPrimitive::from_u32(i);
        match element {
            Some(rule) => rule,
            None => panic!("Not a valid Rule"),
        }
    }
}

#[allow(dead_code)]
pub static RULES_SIZE: u32 = 41; // Used in tests to know what size the cache needs(sometimes, cache dependent)
#[allow(clippy::upper_case_acronyms)] // Again due to generation -> Might solve eventually
#[derive(PartialEq, Eq, Hash, FromPrimitive, Clone, Copy, Debug, Ord, PartialOrd)]
pub enum Rules {
    ASCII,
    And_Predicate,
    Atom,
    Comment,
    Delete,
    Epsilon,
    Grammar,
    Hex,
    HexVal,
    Inline,
    Integer,
    LHS,
    Newline,
    Not_Predicate,
    Nucleus,
    Num,
    NumNoZero,
    One_Or_More,
    Optional,
    OrderedChoiceMatchRange,
    Ordered_Choice,
    Passthrough,
    RHS,
    Rule,
    Semantic_Instructions,
    Sequence,
    StringTerminal,
    Subexpression,
    Terminal,
    Var_Name_Decl,
    Var_Name_Ref,
    Zero_Or_More,
    test_LR_expr,
    test_LR_num,
    test_fact,
    test_indirect_LR_expr,
    test_indirect_LR_num,
    test_indirect_three_level_A,
    test_indirect_three_level_B,
    test_indirect_three_level_C,
    test_term,
}
