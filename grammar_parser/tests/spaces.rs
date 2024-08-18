use cache::MyCache4;
use grammar_parser::*;
use parser_core::Source;
use publisher::Tree;
use rules::RULES_SIZE;

#[test]
fn test_num_false() {
    let string = "aaa".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);

    let result = spaces(&context, &source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_num_true() {
    let string = "\n".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);

    let result = spaces(&context, &source, position);
    assert_eq!(result, (true, 1));
}
