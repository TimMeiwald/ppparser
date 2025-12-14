mod test;
use test::shared;

use c_parser::{BasicContext, Rules, declarator};

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(src, declarator::<BasicContext>, Rules::Declarator);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_2() {
    let src = "int main()";
    let result = shared(src, declarator::<BasicContext>, Rules::Declarator);
    assert_eq!(result, (true, src.len() as u32));
}
