mod test;
use test::shared;

use c_parser::{BasicContext, Rules, function_definition};

#[test]
fn test_function_definition_1() {
    let src = "()";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (false, 0));
}

#[test]
fn test_function_definition_2() {
    let src = "void main(){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_function_definition_3() {
    let src = "void main (){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_function_definition_4() {
    let src = "void main ()\n{}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_function_definition_5() {
    let src = "void main ()\n{
        
          }";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}
