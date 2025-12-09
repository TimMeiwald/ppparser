mod test;
use test::shared;

use c_parser::{BasicContext, Rules, structure_definition};

#[test]
fn test_1() {
    let src = "()";
    let result = shared(
        src,
        structure_definition::<BasicContext>,
        Rules::Structure_definition,
    );
    assert_eq!(result, (false, 0));
}

#[test]
fn test_2() {
    let src = "struct hello{int x;};";
    let result = shared(
        src,
        structure_definition::<BasicContext>,
        Rules::Structure_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_3() {
    let src = "struct hello{int x;}   ;";
    let result = shared(
        src,
        structure_definition::<BasicContext>,
        Rules::Structure_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_4() {
    let src = "struct myHello{}   ;";
    let result = shared(
        src,
        structure_definition::<BasicContext>,
        Rules::Structure_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_5() {
    let src = "struct myStruct
    {
        int x;
        float y; // Should be just fine
    };";
    let result = shared(
        src,
        structure_definition::<BasicContext>,
        Rules::Structure_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}
