mod test;
use test::shared;

use c_parser::{BasicContext, Rules, string_literal};

#[test]
fn test_1() {
    let src = "\"struct\"";
    let result = shared(src, string_literal::<BasicContext>, Rules::String_literal);
    assert_eq!(result, (true, src.len() as u32));
}
