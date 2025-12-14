mod test;
use test::shared;

use c_parser::{BasicContext, Rules, identifier};

#[test]
fn test_1() {
    let src = "-";
    let result = shared(src, identifier::<BasicContext>, Rules::Identifier);
    assert_eq!(result, (false, 0));
}

#[test]
fn test_2() {
    let src = "MyStruct";
    let result = shared(src, identifier::<BasicContext>, Rules::Identifier);
    assert_eq!(result, (true, src.len() as u32));
}
