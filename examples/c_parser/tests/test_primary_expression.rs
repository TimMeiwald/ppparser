mod test;
use test::shared;

use c_parser::{BasicContext, Rules, primary_expression};

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(
        src,
        primary_expression::<BasicContext>,
        Rules::Primary_expression,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_2() {
    let src = "int x";
    let result = shared(
        src,
        primary_expression::<BasicContext>,
        Rules::Primary_expression,
    );
    assert_eq!(result, (true, 3));
}

#[test]
fn test_6() {
    let src = "\"I\"";
    let result = shared(
        src,
        primary_expression::<BasicContext>,
        Rules::Primary_expression,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_4() {
    let src = "28";
    let result = shared(
        src,
        primary_expression::<BasicContext>,
        Rules::Primary_expression,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_5() {
    let src = "28.20";
    let result = shared(
        src,
        primary_expression::<BasicContext>,
        Rules::Primary_expression,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_7() {
    let src = "229+50/30*2";
    let result = shared(
        src,
        primary_expression::<BasicContext>,
        Rules::Primary_expression,
    );
    assert_eq!(result, (true, src.len() as u32));
}
