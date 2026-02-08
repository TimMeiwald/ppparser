mod test;
use test::shared;

use c_parser::{BasicContext, Rules, function_definition};

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (false, 0));
}

#[test]
fn test_2() {
    let src = "int main(){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_3() {
    let src = "int main(int x){return 0;}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}
