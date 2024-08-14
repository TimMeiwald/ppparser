use cache::MyCache4;
use grammar_parser::*;
use parser_core::Source;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;
#[test]
fn test_atom_true() {
    let string = "'A'/\"B\"/\"C\"/\"D\"/\"E\"/\"F\"/\"G\"/\"H\"/\"I\"/\"J\"/\"K\"/\"L\"/\"M\"/\"N\"/\"O\"/\"P\"/\"Q\"/\"R\"/\"S\"/\"T\"/\"U\"/\"V\"/\"W\"/\"X\"/\"Y\"/\"Z\"".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = atom(&context, &source, position);
    context.stack.borrow().print(Key(0), None);
    assert_eq!(result, (true, 3));
}

#[test]
fn test_atom_char() {
    let string = "'A'/'B'".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = atom(&context, &source, position);
    context.stack.borrow().print(Key(0), Some(true));
    assert_eq!(result, (true, 3));
}

#[test]
fn test_atom_string_term() {
    let string = "\"AB\"/\"B\"".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = atom(&context, &source, position);
    context.stack.borrow().print(Key(0), Some(true));
    assert_eq!(result, (true, 4));
}

#[test]
fn test_atom_string_termfalse() {
    let string = "\"A\"/\"B\"".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = atom(&context, &source, position);
    context.stack.borrow().print(Key(0), Some(true));
    assert_eq!(result, (false, 0));
}
