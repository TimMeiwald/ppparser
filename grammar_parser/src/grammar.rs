use cache::Cache;
use parser_core::{Context, _optional};
use rules::rules::Rules;

use parser_core::{Source, _one_or_more, _sequence, _var_name};
use publisher::Publisher;

use crate::{rule, whitespace};

pub fn grammar<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::Rule, context, rule);
    let v2 = _var_name(Rules::Whitespace, context, whitespace);
    let o1 = _optional(&v2);
    let one1 = _one_or_more(&v1);
    let s1 = _sequence(&one1, &o1);
    s1(source, position)
}

#[cfg(test)]
mod tests {
    use super::*;
    //use cache::{BTreeCache, DenyLeftRecursionCache, MyCache4};
    use cache::{MyCache4};
    use parser_core::Source;
    use publisher::{BasicStack, NoopStack, PrinterStack};
    use std::env;
    use std::fs::{canonicalize, read_to_string};
    use rules::Key;
    #[test]
    fn test_grammar_true() {
        let string = "<Spaces> PASSTHROUGH = \"\n\"/\"\t\"/\"\r\"/\" \";".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(src_len, 45);

        let result = grammar(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }
    #[test]
    fn test_grammar_true2() {
        let string = "<Specials> PASSTHROUGH = \"+\"/\"*\"/\"-\"/\"&\"/\"!\"/\"?\"/\"<\"/\">\"/\"\"\"/\"(\"/\")\"/\"_\"/\",\"/\"/\"/\";\"/\"=\"/\"\\\"/\"#\"/\":\"/\"|\"/\".\"/\"{\"/\"}\"/\"[\"/\"]\"/\"%\"/\"'\"/\"^\"/\"~\";".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(src_len, 45);

        let result = rule(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }
    #[test]
    fn test_grammar_true3() {
        println!("{:?}", env::current_dir().unwrap());
        let path = "../parser_core/tests/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");

        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(src_len, 45);
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
        let path = "../parser_core/tests/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");

        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, PrinterStack>::new(src_len, 45);
        let result = grammar(&context, &source, position);
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
        let path = "../parser_core/tests/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, BasicStack>::new(src_len, 45);
        let result = grammar(&context, &source, position);
        context.stack.borrow().print(Key(0), None);
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
        let string = "<Rule>=\"A\"/\"B\";".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, BasicStack>::new(src_len, 45);
        let result = grammar(&context, &source, position);
        context.stack.borrow().print(Key(0), None);
        // for i in &*context.publisher.borrow() {
        //     // if i[0] == 20 || i[0] == 36 || i[0] == 29 || (i[0] >= 26 && i[0] <= 32) {
        //     //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);
        //     // }
        //     //println!("{}",i[0]);
        //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);
        // }
        assert_eq!(result, (true, src_len));
    }
}
