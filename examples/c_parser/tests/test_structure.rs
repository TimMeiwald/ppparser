mod test;
use test::shared;

use c_parser::{BasicContext, Rules, struct_or_union_specifier};

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(
        src,
        struct_or_union_specifier::<BasicContext>,
        Rules::Struct_or_union_specifier,
    );
    assert_eq!(result, (false, 0));
}

#[test]
fn test_2() {
    let src = "struct MyStruct";
    let result = shared(
        src,
        struct_or_union_specifier::<BasicContext>,
        Rules::Struct_or_union_specifier,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_3() {
    let src = "struct MyStruct{}";
    let result = shared(
        src,
        struct_or_union_specifier::<BasicContext>,
        Rules::Struct_or_union_specifier,
    );
    assert_eq!(result, (true, 15)); // You cannot have an empty struct which is why {} is not matched.
}
#[test]
fn test_4() {
    let src = "struct MyStruct{int x;}";
    let result = shared(
        src,
        struct_or_union_specifier::<BasicContext>,
        Rules::Struct_or_union_specifier,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_5() {
    let src = "struct MyStruct{int x;float y;}";
    let result = shared(
        src,
        struct_or_union_specifier::<BasicContext>,
        Rules::Struct_or_union_specifier,
    );
    assert_eq!(result, (true, src.len() as u32));
}
