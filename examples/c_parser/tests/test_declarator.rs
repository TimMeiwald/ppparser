mod test;
use test::shared;

use c_parser::{
    BasicContext, Rules, compound_statement, declaration_specifiers, declarator, type_specifier,
};

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(src, declarator::<BasicContext>, Rules::Declarator);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_2() {
    let src = "main()";
    let result = shared(src, declarator::<BasicContext>, Rules::Declarator);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_3() {
    let src = "{}";
    let result = shared(
        src,
        compound_statement::<BasicContext>,
        Rules::Compound_statement,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_4() {
    let src = "int main(){}";
    let result = shared(
        src,
        declaration_specifiers::<BasicContext>,
        Rules::Declaration_specifiers,
    );
    assert_eq!(result, (true, 4));
}
