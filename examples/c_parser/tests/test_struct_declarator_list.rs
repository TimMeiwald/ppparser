mod test;
use test::shared;

use c_parser::{BasicContext, Rules, struct_declarator_list};

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(
        src,
        struct_declarator_list::<BasicContext>,
        Rules::Struct_declarator_list,
    );
    assert_eq!(result, (true, 6));
}

#[test]
fn test_2() {
    let src = "int x";
    let result = shared(
        src,
        struct_declarator_list::<BasicContext>,
        Rules::Struct_declarator_list,
    );
    assert_eq!(result, (true, 4));
}
