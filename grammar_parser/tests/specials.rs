use cache::MyCache4;
use grammar_parser::*;
use publisher::Tree;

use parser_core::Source;
#[test]
fn test_specials_false() {
    let string = "aaa".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = specials(&context, &source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_specials_true() {
    let string = '~'.to_string();
    let src_len = string.len() as u32;

    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let source = Source::new(string);
    let position: u32 = 0;
    let result = specials(&context, &source, position);
    assert_eq!(result, (true, 1));
}
