use cache::MyCache4;
use grammar_parser::{ascii, Context};
use parser_core::Source;
use publisher::Tree;
use rules::RULES_SIZE;

#[test]
fn test_ascii_true() {
    let string = "aaa".to_string();

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(0, RULES_SIZE);

    let result = ascii(&context, &source, position);
    assert_eq!(result, (true, 1));
}
#[test]
fn test_ascii_true2() {
    let string = "~".to_string();

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(0, RULES_SIZE);

    let result = ascii(&context, &source, position);
    assert_eq!(result, (true, 1));
}
