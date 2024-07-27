use cache::MyCache4;
use grammar_parser::*;
use parser_core::Source;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;

#[test]
fn test_hex() {
    let string = "0x0001F600".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = hex(&context, &source, position);
    context.stack.borrow().print(Key(0), None);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_hex2() {
    let string = "0x0".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = hex(&context, &source, position);
    context.stack.borrow().print(Key(0), None);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_hex3() {
    let string = "0xFF".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = hex(&context, &source, position);
    context.stack.borrow().print(Key(0), None);
    assert_eq!(result, (true, src_len));
}
