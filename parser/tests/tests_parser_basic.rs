// These tests are often the same as in tests_parser_complex
// The difference is that here we only check the returned boolean and position.
// In order to easily determine whether it's the AST incorrect or the result.
// Please add any future tests to both.

#[cfg(test)]
mod tests {
    use ::parser::*;
    use core::cell::RefCell;
    use std::env;
    use std::fs::{canonicalize, read_to_string};

    #[test]
    fn test_ppparser_dsl_grammar_rule() {
        let string = "<Spaces> = '\n'/'\t'/'\r'/' ';".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state = RefCell::new(UserState::new());

        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        context
            .into_inner()
            .get_publisher()
            .clear_false()
            .print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
    }

    #[test]
    fn test_square_braces() {
        let string = "<Spaces> = ['A'..'Z'];".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state = RefCell::new(UserState::new());

        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        context
            .into_inner()
            .get_publisher()
            .clear_false()
            .print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
    }

    #[test]
    fn test_square_braces_in_mixed() {
        let string = "<Spaces> = (['A'..'Z'], 'Z');".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state = RefCell::new(UserState::new());

        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        context
            .into_inner()
            .get_publisher()
            .clear_false()
            .print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
    }

    #[test]
    fn test_external_rule_call() {
        let string = "<Spaces> = external_rule_call(['A'..'Z'], 'Z');".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state = RefCell::new(UserState::new());

        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        context
            .into_inner()
            .get_publisher()
            .clear_false()
            .print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
    }

    #[test]
    fn test_strings() {
        let string = r#"<Spaces> = (['A'..'Z'], "ZAA");"#.to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state = RefCell::new(UserState::new());

        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        context
            .into_inner()
            .get_publisher()
            .clear_false()
            .print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
    }

    #[test]
    fn test_dsl_grammar() {
        println!("{:?}", env::current_dir().unwrap());
        let path = "Grammar.dsl";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");

        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state = RefCell::new(UserState::new());

        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        let tree = context.into_inner();
        let publisher = tree.get_publisher();
        publisher.print(Key(0), Some(true));
        //context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, src_len));
    }
}
