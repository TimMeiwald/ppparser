use cache::MyCache4;
use grammar_parser::*;
use parser_core::Source;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;

#[test]
fn test_terminal_false() {
    let string = "'a".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = terminal(&context, &source, position);
    context.stack.borrow().print(Key(0), None);

    assert_eq!(result, (false, 0));
}
#[test]
fn test_terminal_true() {
    let string = "'a'".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = terminal(&context, &source, position);
    context.stack.borrow().print(Key(0), None);
    assert_eq!(result, (true, 3));
}
#[test]
fn test_terminal_true1() {
    let string = "'\n'".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = terminal(&context, &source, position);
    assert_eq!(result, (true, 3));
}
#[test]
fn test_terminal_true2() {
    let string = "'\t'".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = terminal(&context, &source, position);
    assert_eq!(result, (true, 3));
}
#[test]
fn test_terminal_true3() {
    let string = "'\r'".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = terminal(&context, &source, position);
    assert_eq!(result, (true, 3));
}

#[test]
fn test_terminal_true4() {
    let string = "' '".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = terminal(&context, &source, position);
    assert_eq!(result, (true, 3));
}
