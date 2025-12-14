mod test;
use test::shared;

use c_parser::{BasicContext, Rules, specifier_qualifier_list};

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(
        src,
        specifier_qualifier_list::<BasicContext>,
        Rules::Specifier_qualifier_list,
    );
    assert_eq!(result, (false, 0));
}

#[test]
fn test_2() {
    let src = "int x";
    let result = shared(
        src,
        specifier_qualifier_list::<BasicContext>,
        Rules::Specifier_qualifier_list,
    );
    assert_eq!(result, (true, src.len() as u32));
}
