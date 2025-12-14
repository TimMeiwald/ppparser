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
pub static RULES_SIZE: u32 = 23;
#[allow(clippy::upper_case_acronyms)] // Again due to generation -> Might solve eventually
#[derive(PartialEq, Eq, Hash, FromPrimitive, Clone, Copy, Debug, Ord, PartialOrd)]

pub enum Rules {
    Array,
    Char,
    Comment,
    Complex_assignment,
    Core_types,
    Enumeration_definition,
    Expression,
    Float,
    Function,
    Function_body,
    Function_parameters,
    Function_type_parameters,
    Grammar,
    Integer,
    Statement,
    String,
    Structure_definition,
    Type_definition,
    Type_name,
    Union_definition,
    Variable_assignment,
    Variable_name,
    Wsc,
}
