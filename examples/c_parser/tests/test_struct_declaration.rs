mod test;
use test::shared;

use c_parser::{BasicContext, Rules, struct_declaration};

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(
        src,
        struct_declaration::<BasicContext>,
        Rules::Struct_declaration,
    );
    assert_eq!(result, (false, 0));
}

#[test]
fn test_3() {
    let src = "struct MyStruct{}";
    let result = shared(
        src,
        struct_declaration::<BasicContext>,
        Rules::Struct_declaration,
    );
    assert_eq!(result, (false, 0));
}

#[test]
fn test_2() {
    let src = "int x";
    let result = shared(
        src,
        struct_declaration::<BasicContext>,
        Rules::Struct_declaration,
    );
    assert_eq!(result, (true, src.len() as u32));
}
