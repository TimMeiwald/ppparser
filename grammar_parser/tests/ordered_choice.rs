use cache::MyCache4;
use grammar_parser::*;
use parser_core::Source;
use publisher::Tree;

#[test]
fn test_ordered_choice_false() {
    let string = "<this_is_a_valid_var_name>".to_string();
    let str_len = string.len() as u32;

    let source = Source::new(&string);

    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(str_len, 52);

    let result = ordered_choice(&context, &source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_ordered_choice_true() {
    let string = "'A'/'B'".to_string();
    let str_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(str_len, 52);

    let result = ordered_choice(&context, &source, position);
    assert_eq!(result, (true, 7));
}
#[test]
fn test_ordered_choice_true10() {
    let string = "<A>/<B>".to_string();
    let str_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(str_len, 52);

    let result = ordered_choice(&context, &source, position);
    assert_eq!(result, (true, 7));
}

#[test]
fn test_ordered_choice_true2() {
    let string = "'A'/'B'/'C'".to_string();
    let str_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(str_len, 52);
    let result = ordered_choice(&context, &source, position);
    assert_eq!(result, (true, 11));
}

#[test]
fn test_ordered_choice_true3() {
    let string = "'A'/'B'/'C'/'D'/'E'/'F'/'G'/'H'/'I'/'J'/'K'/'L'/'M'/'N'/'O'/'P'/'Q'/'R'/'S'/'T'/'U'/'V'/'W'/'X'/'Y'/'Z'".to_string();
    let str_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(str_len, 52);

    let result = ordered_choice(&context, &source, position);
    assert_eq!(result, (true, str_len));
}
#[test]
fn test_ordered_choice_true4() {
    let string = "'+'/'*'/'-'/'&'/'!'/'?'/'<'/'>'/'''/'('/')'/'_'/','/'/'/';'/'='/'\\'/'#'/':'/'|'/'.'/'{'/'}'/'['/']'/'%'/'''/'^'/'~'".to_string();
    let str_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(str_len, 52);

    let result = ordered_choice(&context, &source, position);
    assert_eq!(result, (true, str_len));
}

#[test]
fn test_ordered_choice_true5() {
    let string = "'\n'/'\t'/'\r'/' '".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = ordered_choice(&context, &source, position);
    assert_eq!(result, (true, src_len));
}
