
use num_derive::FromPrimitive;
#[derive(FromPrimitive)]
#[derive(Clone, Copy, Debug)]
pub enum Rules {
    AlphabetUpper,
    AlphabetLower,
    Num,
    Spaces,
    Specials,
    Ascii,
    Apostrophe,
    LeftAngleBracket,
    RightAngleBracket,
    LeftBracket,
    RightBracket,
    Assignment,
    EndRule,
    Ampersand,
    ExclamationMark,
    Plus,
    Star,
    QuestionMark,
    Comma,
    Backslash,
    VarName,
    Subexpression,
    Epsilon,
    Terminal,
    Nucleus,
    Atom,
    AndPredicate,
    NotPredicate,
    Sequence,
    OrderedChoice,
    OneOrMore,
    ZeroOrMore,
    Optional,
    Whitespace,
    Rhs,
    Lhs,
    Rule,
    Grammar,
    Comment,
    SemanticInstructions,
    Delete,
    Passthrough,
    Collect,
    VarNameDecl,
    NewLine,
}

impl<'a> From<u32> for Rules {
    fn from(i: u32) -> Rules {
        let element = num::FromPrimitive::from_u32(i);
        match element {
            Some(rule) => {return rule;},
            None => panic!("Not a valid Rule")
        }
    }
}