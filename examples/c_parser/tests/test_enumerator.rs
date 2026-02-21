mod test;
use std::cell::RefCell;

use test::shared;

use c_parser::{
    BasicContext, Rules, UserState, enum_specifier, enumerator, function_definition,
    type_specifier, user_state,
};

use crate::test::shared_custom_user_state;

#[test]
fn test_1() {
    let src = "VALUE";
    let result = shared(src, enumerator::<BasicContext>, Rules::Enumerator);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_2() {
    let src = "VALUE=0";
    let result = shared(src, enumerator::<BasicContext>, Rules::Enumerator);
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_3() {
    let src = "MEDIUM\n";
    let result = shared(src, enumerator::<BasicContext>, Rules::Enumerator);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_4() {
    let src = "enum Level {
  LOW,
  }";
    let result = shared(src, enum_specifier::<BasicContext>, Rules::Enum_specifier);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_5() {
    let src = "enum Level {
                                    LOW,
                                    MEDIUM
                                }";
    let result = shared(src, enum_specifier::<BasicContext>, Rules::Enum_specifier);
    assert_eq!(result, (true, src.len() as u32));
}
