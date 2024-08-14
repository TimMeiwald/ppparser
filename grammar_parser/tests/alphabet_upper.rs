use cache::MyCache4;
use grammar_parser::*;
use parser_core::*;
use publisher::Tree;
use rules::Rules;

#[test]
fn test_alphabet_upper_false() {
    let string = "aaa".to_string();
    let src_len = string.len();
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len as u32, 52);

    let result = alphabet_upper(&context, &source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_alphabet_upper_true() {
    let string = "AAA".to_string();
    let src_len = string.len();
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len as u32, 52);

    let result = alphabet_upper(&context, &source, position);
    assert_eq!(result, (true, 1));
}
#[test]
fn test_alphabet_upper_true_with_var_name() {
    let string = "AAA".to_string();
    let src_len = string.len();
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len as u32, 52);

    let var_name_closure = _var_name(Rules::Alphabet_Upper, &context, alphabet_upper);
    let result = var_name_closure(&source, position);
    assert_eq!(result, (true, 1));
}
