use cache::DirectLeftRecursionCache;
// use cache::MyCache4;
use grammar_parser::*;
use parser_core::Source;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;
use rules::RULES_SIZE;
#[test]
fn test_ordered_choice_false() {
    let string = "<this_is_a_valid_var_name>".to_string();
    let str_len = string.len() as u32;

    let source = Source::new(&string);

    let position: u32 = 0;
    let context = Context::<DirectLeftRecursionCache, Tree>::new(str_len, RULES_SIZE);

    let result = ordered_choice(&context, &source, position);
    assert_eq!((result.0, result.1), (false, 0));
}
#[test]
fn test_ordered_choice_true() {
    let string = "'A'/'B'".to_string();
    let str_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<DirectLeftRecursionCache, Tree>::new(str_len, RULES_SIZE);

    let result = ordered_choice(&context, &source, position);
    assert_eq!((result.0, result.1), (true, 7));
}
#[test]
fn test_ordered_choice_true10() {
    let string = "<A>/<B>".to_string();
    let str_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<DirectLeftRecursionCache, Tree>::new(str_len, RULES_SIZE);

    let result = ordered_choice(&context, &source, position);
    assert_eq!((result.0, result.1), (true, 7));
}

#[test]
fn test_ordered_choice_true2() {
    let string = "'A'/'B'/'C'".to_string();
    let str_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<DirectLeftRecursionCache, Tree>::new(str_len, RULES_SIZE);
    let result = ordered_choice(&context, &source, position);
    assert_eq!((result.0, result.1), (true, 11));
}

#[test]
fn test_ordered_choice_true3() {
    let string = "'A'/'B'/'C'/'D'/'E'/'F'/'G'/'H'/'I'/'J'/'K'/'L'/'M'/'N'/'O'/'P'/'Q'/'R'/'S'/'T'/'U'/'V'/'W'/'X'/'Y'/'Z'".to_string();
    let str_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<DirectLeftRecursionCache, Tree>::new(str_len, RULES_SIZE);

    let result = ordered_choice(&context, &source, position);
    assert_eq!((result.0, result.1), (true, str_len));
}
#[test]
fn test_ordered_choice_true4() {
    let string = "'+'/'*'/'-'/'&'/'!'/'?'/'<'/'>'/'''/'('/')'/'_'/','/'/'/';'/'='/'\\'/'#'/':'/'|'/'.'/'{'/'}'/'['/']'/'%'/'''/'^'/'~'".to_string();
    let str_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<DirectLeftRecursionCache, Tree>::new(str_len, RULES_SIZE);

    let result = ordered_choice(&context, &source, position);
    assert_eq!((result.0, result.1), (true, str_len));
}

#[test]
fn test_ordered_choice_true5() {
    let string = "'\n'/'\t'/'\r'/' '".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<DirectLeftRecursionCache, Tree>::new(src_len, RULES_SIZE);

    let result = ordered_choice(&context, &source, position);
    context.stack.borrow().print(Key(0), None);
    assert_eq!((result.0, result.1), (true, src_len));
}
