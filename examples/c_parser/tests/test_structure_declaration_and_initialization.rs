mod test;
use test::shared;

use c_parser::{BasicContext, Rules, structure_declaration_and_initialization};

#[test]
fn test_1() {
    let src = "struct myStruct a = {};";
    let result = shared(
        src,
        structure_declaration_and_initialization::<BasicContext>,
        Rules::Structure_declaration_and_initialization,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_2() {
    let src = "struct myStruct a = {20};";
    let result = shared(
        src,
        structure_declaration_and_initialization::<BasicContext>,
        Rules::Structure_declaration_and_initialization,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_3() {
    let src = "struct myStruct a = {x};";
    let result = shared(
        src,
        structure_declaration_and_initialization::<BasicContext>,
        Rules::Structure_declaration_and_initialization,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_4() {
    let src = "struct myStruct a = {x, 20};";
    let result = shared(
        src,
        structure_declaration_and_initialization::<BasicContext>,
        Rules::Structure_declaration_and_initialization,
    );
    assert_eq!(result, (true, src.len() as u32));
}
