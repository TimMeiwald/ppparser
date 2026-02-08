mod test;
use test::shared;

use c_parser::{BasicContext, Rules, struct_declarator};

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(
        src,
        struct_declarator::<BasicContext>,
        Rules::Struct_declarator,
    );
    assert_eq!(result, (true, src.len() as u32));
}
