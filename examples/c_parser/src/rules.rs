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
pub static RULES_SIZE: u32 = 18;
#[allow(clippy::upper_case_acronyms)] // Again due to generation -> Might solve eventually
#[derive(PartialEq, Eq, Hash, FromPrimitive, Clone, Copy, Debug, Ord, PartialOrd)]

pub enum Rules {
    Grammar,
    Comment,
    Ctype,
    Expression,
    Function_body,
    Function_call,
    Function_declaration,
    Function_definition,
    Function_header,
    Function_parameters,
    Int,
    Multiline_comment,
    Name,
    Parameter,
    Reserved_words,
    Return_statement,
    Statement,
    Wsc,
}
