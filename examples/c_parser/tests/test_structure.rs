mod test;
use test::shared;

use c_parser::{BasicContext, Rules, function_definition, struct_declaration};

#[test]
fn test_function_definition_1() {
    let src = "struct MyStruct{};";
    let result = shared(
        src,
        struct_declaration::<BasicContext>,
        Rules::Struct_declaration,
    );
    assert_eq!(result, (false, 0));
}
