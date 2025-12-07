mod test;
use test::shared;

use c_parser::{BasicContext, Rules, function_parameters};

#[test]
fn test_function_paramters_1() {
    let src = "()";
    let result = shared(
        src,
        function_parameters::<BasicContext>,
        Rules::Function_parameters,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_function_paramters_2() {
    let src = "() ";
    let result = shared(
        src,
        function_parameters::<BasicContext>,
        Rules::Function_parameters,
    );
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_function_paramters_3() {
    let src = "(int x)";
    let result = shared(
        src,
        function_parameters::<BasicContext>,
        Rules::Function_parameters,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_function_paramters_4() {
    let src = "(int x )";
    let result = shared(
        src,
        function_parameters::<BasicContext>,
        Rules::Function_parameters,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_function_paramters_5() {
    let src = "(int x, yoyo y)";
    let result = shared(
        src,
        function_parameters::<BasicContext>,
        Rules::Function_parameters,
    );
    assert_eq!(result, (true, src.len() as u32));
}
