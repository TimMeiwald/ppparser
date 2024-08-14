use cache::MyCache4;
use grammar_parser::*;
use parser_core::Source;
use publisher::Tree;

#[test]
fn test_rule_true() {
    let string = "<Alphabet_Upper> PASSTHROUGH = 'A'/'B'/'C'/'D'/'E'/'F'/'G'/'H'/'I'/'J'/'K'/'L'/'M'/'N'/'O'/'P'/'Q'/'R'/'S'/'T'/'U'/'V'/'W'/'X'/'Y'/'Z'; #We all love commments#".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_rule_true2() {
    let string = "<Spaces> PASSTHROUGH = '\n'/'\t'/'\r'/' ';".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_rule_true40() {
    let string = "<Alphabet_Upper> = ['A'..'Z'];
"
    .to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_rule_true45() {
    let string = "<Alphabet_Lower> PASSTHROUGH = ['a'..'z'];".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_rule_true3() {
    let string = "<Specials> PASSTHROUGH = '+'/'*'/'-'/'&'/'!'/'?'/'<'/'>'/'''/'('/')'/'_'/','/'/'/';'/'='/'\\'/'#'/':'/'|'/'.'/'{'/'}'/'['/']'/'%'/'''/'^'/'~';".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_rule_true25() {
    let string = "<Rule>='A'/'B';".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_rule_true101() {
    let string = "<Alphabet_Upper> PASSTHROUGH = ['A'..'Z']; #We all love commments#".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_rule_true4() {
    let string =
        "<ASCII> PASSTHROUGH = <Alphabet_Lower>/<Alphabet_Upper>/<Num>/<Spaces>/<Specials>;"
            .to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_rule_true5() {
    let string = "<RHS> PASSTHROUGH = <Sequence>/<Ordered_Choice>/<Atom>;
"
    .to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}
#[test]
fn test_rule_true6() {
    let string = "<LHS> = <Var_Name>, (<Whitespace>, <Semantic_Instructions>, <Whitespace>)?;
"
    .to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}
#[test]
fn test_rule_true7() {
    let string = "<Rule> = <LHS>, <Whitespace>, <Assignment>, <Whitespace>, <RHS>, <Whitespace>, <End_Rule>, <Whitespace>, <Comment>*;".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}
