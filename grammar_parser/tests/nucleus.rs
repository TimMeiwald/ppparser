use cache::MyCache4;
use grammar_parser::*;
use parser_core::Source;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;
use rules::RULES_SIZE;

#[test]
fn test_nucleus_true() {
    let string = "'A'/'B'/'C'/'D'/'E'/'F'/'G'/'H'/'I'/'J'/'K'/'L'/'M'/'N'/'O'/'P'/'Q'/'R'/'S'/'T'/'U'/'V'/'W'/'X'/'Y'/'Z'".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);

    let result = nucleus(&context, &source, position);
    context.stack.borrow().print(Key(0), None);
    assert_eq!(result, (true, 3));
}
#[test]
fn test_nucleus_char() {
    let string = "'A'".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);

    let result = nucleus(&context, &source, position);
    context.stack.borrow().print(Key(0), None);

    assert_eq!(result, (true, 3));
}
