#![allow(non_camel_case_types)] // Again due to generation -> Might solve eventually but not tha

use num_derive::FromPrimitive;
pub static RULES_SIZE: u32 = 55;
#[derive(Eq, Hash, FromPrimitive, Clone, Copy, Debug)]
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
}
