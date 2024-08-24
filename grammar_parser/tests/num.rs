use cache::DirectLeftRecursionCache;
// use cache::MyCache4;
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
    let context = Context::<DirectLeftRecursionCache, Tree>::new(src_len, RULES_SIZE);

    let result = num(&context, &source, position);
    assert_eq!((result.0, result.1), (false, 0));
}
#[test]
fn test_num_true() {
    let string = "511".to_string();
    let src_len = string.len() as u32;

    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<DirectLeftRecursionCache, Tree>::new(src_len, RULES_SIZE);

    let result = num(&context, &source, position);
    assert_eq!((result.0, result.1), (true, 1));
}
