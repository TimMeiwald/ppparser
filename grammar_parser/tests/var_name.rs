use cache::MyCache4;
use grammar_parser::*;
use parser_core::Source;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;
use rules::RULES_SIZE;

#[test]
fn test_var_name_false() {
    let string = "_this_is_not_a_valid_var_name".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);

    let result = var_name(&context, &source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_var_name_true() {
    let string = "<this_is_a_valid_var_name>".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);
    let result = var_name(&context, &source, position);
    context.stack.borrow().print(Key(0), None);
    assert_eq!(result, (true, 26));
}
#[test]
fn test_var_name_true2() {
    let string = "<Alphabet_Upper>".to_string();

    let src_len = string.len() as u32;
    let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);

    let source = Source::new(&string);
    let position: u32 = 0;
    let result = var_name(&context, &source, position);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_var_name_false2() {
    let string = " ".to_string();

    let src_len = string.len() as u32;
    let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);

    let source = Source::new(&string);
    let position: u32 = 0;
    let result = var_name(&context, &source, position);
    assert_eq!(result, (false, 0));
}
