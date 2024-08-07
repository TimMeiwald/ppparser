use cache::MyCache4;
use grammar_parser::*;
use parser_core::Source;
use publisher::Tree;

#[test]
fn test_rhs_true() {
    let string = "'A'/'B'/'C'/'D'/'E'/'F'/'G'/'H'/'I'/'J'/'K'/'L'/'M'/'N'/'O'/'P'/'Q'/'R'/'S'/'T'/'U'/'V'/'W'/'X'/'Y'/'Z'".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rhs(&context, &source, position);
    assert_eq!(result, (true, src_len));
}
