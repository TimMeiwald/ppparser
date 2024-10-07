#![allow(non_camel_case_types)] // Again due to generation -> Might solve eventually

use num_derive::FromPrimitive;
#[allow(dead_code)]
pub static RULES_SIZE: u32 = 57; // Used in tests to know what size the cache needs(sometimes, cache dependent)
#[derive(PartialEq, Eq, Hash, FromPrimitive, Clone, Copy, Debug, Ord, PartialOrd)]
#[allow(clippy::upper_case_acronyms)] // Again due to generation -> Might solve eventually

pub enum Rules {
    ASCII,
    Alphabet_Lower,
    Alphabet_Upper,
    Ampersand,
    And_Predicate,
    Apostrophe,
    Assignment,
    Atom,
    Backslash,
    Comma,
    Comment,
    Delete,
    End_Rule,
    Epsilon,
    Exclamation_Mark,
    Grammar,
    Hex,
    HexVal,
    Inline,
    Integer,
    LHS,
    Left_Angle_Bracket,
    Left_Bracket,
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
    Plus,
    Question_Mark,
    QuotationMark,
    RHS,
    Right_Angle_Bracket,
    Right_Bracket,
    Rule,
    Semantic_Instructions,
    Sequence,
    Spaces,
    Specials,
    Star,
    StringTerminal,
    Subexpression,
    Terminal,
    Var_Name,
    Var_Name_Decl,
    Zero_Or_More,
    test_LR_expr,
    test_LR_num,
    test_indirect_LR_expr,
    test_indirect_LR_num,
    test_term,
    test_fact,
}

impl From<u32> for Rules {
    fn from(i: u32) -> Rules {
        let element = num::FromPrimitive::from_u32(i);
        match element {
            Some(rule) => rule,
            None => panic!("Not a valid Rule"),
        }
    }
}
