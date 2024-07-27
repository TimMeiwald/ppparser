use grammar_parser::*;
//use cache::{BTreeCache, DenyLeftRecursionCache, MyCache4};
use cache::MyCache4;
use parser_core::Source;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;
use std::env;
use std::fs::{canonicalize, read_to_string};
#[test]
fn test_grammar_true() {
    let string = "<Spaces> PASSTHROUGH = '\n'/'\t'/'\r'/' ';".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = grammar(&context, &source, position);
    assert_eq!(result, (true, src_len));
}
#[test]
fn test_grammar_true2() {
    let string = "<Specials> PASSTHROUGH = '+'/'*'/'-'/'&'/'!'/'?'/'<'/'>'/'''/'('/')'/'_'/','/'/'/';'/'='/'\\'/'#'/':'/'|'/'.'/'{'/'}'/'['/']'/'%'/'''/'^'/'~';".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);

    let result = rule(&context, &source, position);
    assert_eq!(result, (true, src_len));
}
#[test]
fn test_grammar_true3() {
    println!("{:?}", env::current_dir().unwrap());
    let path = "../grammar_parser/tests/newGrammar_test_only_dont_modify.dsl";
    let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    let string = read_to_string(pathbuf).expect("If it's moved change the string above");

    let src_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);
    let result = grammar(&context, &source, position);
    assert_eq!(result, (true, src_len));
}

// #[test]
// fn test_deny_left_recursion_cache() {
//     println!("{:?}", env::current_dir().unwrap());
//     let path = "../parser_core/tests/Grammar.txt";
//     let pathbuf = canonicalize(path).expect("If it's moved change the string above");
//     let string = read_to_string(pathbuf).expect("If it's moved change the string above");

//     let src_len = string.len() as u32;
//     let source = Source::new(string);
//     let position: u32 = 0;
//     let context = Context::<DenyLeftRecursionCache, NoopPublisher>::new(src_len, 45);
//     let result = grammar(&context, &source, position);
//     assert_eq!(result, (true, src_len));
// }
#[test]
fn test_my_cache_4() {
    println!("{:?}", env::current_dir().unwrap());
    let path = "../grammar_parser/tests/newGrammar_test_only_dont_modify.dsl";
    let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    let string = read_to_string(pathbuf).expect("If it's moved change the string above");

    let src_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);
    let result = grammar(&context, &source, position);
    assert_eq!(result, (true, src_len));
}
#[test]
fn test_json_description() {
    println!("{:?}", env::current_dir().unwrap());
    let path = "../json_parser/json.dsl";
    let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    let string = read_to_string(pathbuf).expect("If it's moved change the string above");

    let src_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);
    let result = grammar(&context, &source, position);
    context.stack.borrow().print(Key(0), None);
    let only_true_tree = context.stack.borrow().clear_false();
    assert_eq!(result, (true, src_len));
}
// #[test]
// fn test_btree_cache() {
//     println!("{:?}", env::current_dir().unwrap());
//     let path = "../parser_core/tests/Grammar.txt";
//     let pathbuf = canonicalize(path).expect("If it's moved change the string above");
//     let string = read_to_string(pathbuf).expect("If it's moved change the string above");

//     let src_len = string.len() as u32;
//     let source = Source::new(string);
//     let position: u32 = 0;
//     let context = Context::<BTreeCache, NoopPublisher>::new(src_len, 45);
//     let result = grammar(&context, &source, position);
//     assert_eq!(result, (true, src_len));
// }

#[test]
fn test_basic_publisher() {
    println!("{:?}", env::current_dir().unwrap());
    let path = "../grammar_parser/tests/newGrammar_test_only_dont_modify.dsl";
    let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    let string = read_to_string(pathbuf).expect("If it's moved change the string above");
    let src_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);
    let result = grammar(&context, &source, position);
    //context.stack.borrow().print(Key(0), Some(true));
    let only_true_tree = context.stack.borrow().clear_false();
    //only_true_tree.print(Key(0), None);
    //println!("Capacity: {:?}, Len: {:?}", context.stack.borrow().capacity(), context.stack.borrow().len());
    //println!("Capacity: {:?}, Len: {:?}", only_true_tree.capacity(), only_true_tree.len());
    // for i in &*context.publisher.borrow() {
    //     // if i[0] == 20 || i[0] == 36 || i[0] == 29 || (i[0] >= 26 && i[0] <= 32) {
    //     //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);
    //     // }
    //     //println!("{}",i[0]);
    //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);
    // }
    assert_eq!(result, (true, src_len));
}

#[test]
fn test_basic_publisher_short() {
    let string = "<Rule>='A'/'B';".to_string();
    let src_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);
    let result = grammar(&context, &source, position);
    context.stack.borrow().print(Key(0), None);
    let only_true_tree = context.stack.borrow().clear_false();
    println!("\n###############################################\nWithout False Values\n######################################################");

    only_true_tree.print(Key(0), None);
    // for i in &*context.publisher.borrow() {
    //     // if i[0] == 20 || i[0] == 36 || i[0] == 29 || (i[0] >= 26 && i[0] <= 32) {
    //     //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);p
    //     // }
    //     //println!("{}",i[0]);
    //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);
    // }
    assert_eq!(result, (true, src_len));
}
