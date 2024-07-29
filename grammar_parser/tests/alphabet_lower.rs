use cache::MyCache4;
use grammar_parser::*;
use publisher::Tree;

#[test]
fn test_alphabet_lower_false() {
    let string = "AAA".to_string();

    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(0, 44);

    let result = alphabet_lower(&context, &source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_alphabet_lower_true() {
    let string = "aaa".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = alphabet_lower(&context, &source, position);
    assert_eq!(result, (true, 1));
}
#[test]
fn test_alphabet_lower_true2() {
    let string = "zzz".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = alphabet_lower(&context, &source, position);
    assert_eq!(result, (true, 1));
}
