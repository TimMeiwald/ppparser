// These tests are often the same as in tests_parser_complex
// The difference is that here we only check the returned boolean and position.
// In order to easily determine whether it's the AST incorrect or the result.
// Please add any future tests to both.

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use parser::*;
    use std::env;
    use std::fs::{canonicalize, read_to_string};
    #[test]
    fn test_ppparser_dsl_grammar_rule() {
        let string = "<Spaces> PASSTHROUGH = '\n'/'\t'/'\r'/' ';".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let executor = _var_name(Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
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
        let path = "tests/test_data/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");

        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let executor = _var_name(Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        //context.borrow().print_cache();
        let tree = context.into_inner();
        let publisher = tree.get_publisher();
        publisher.print(Key(0), Some(true));
        //context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, src_len));
    }
    #[test]
    fn test_left_recursion_direct_1() {
        let string = "1-2-3-7-9-1   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let executor =
                _var_name_direct_left_recursion(Rules::test_LR_expr, &context, test_lr_expr);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 11));
    }
    #[test]
    fn test_left_recursion_direct_2() {
        let string = "1-2   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let executor =
                _var_name_direct_left_recursion(Rules::test_LR_expr, &context, test_lr_expr);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 3));
        let result_tree = context.into_inner().get_publisher().clear_false();
        result_tree.print(Key(0), None);
    }
    #[test]
    fn test_left_recursion_direct_99() {
        let string = "1   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let executor =
                _var_name_direct_left_recursion(Rules::test_LR_expr, &context, test_lr_expr);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 1));
        let result_tree = context.into_inner().get_publisher().clear_false();
        result_tree.print(Key(0), None);
    }

    #[test]
    fn test_left_recursion_direct_3() {
        let string = "1*2/3*7/9   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let executor = _var_name_direct_left_recursion(Rules::test_fact, &context, test_fact);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        //context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 9));
    }
    #[test]
    fn test_left_recursion_direct_4() {
        let string = "1/2/3/4+5+7+9   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let executor = _var_name_direct_left_recursion(Rules::test_term, &context, test_term);
        let result = executor(Key(0), &source, position);
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 13));
    }
    #[test]
    fn test_left_recursion_direct_5() {
        let string = "1+2+3+4/5/7/9   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let executor = _var_name_direct_left_recursion(Rules::test_term, &context, test_term);
        let result = executor(Key(0), &source, position);
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 13));
    }
    #[test]
    fn test_left_recursion_direct_6() {
        let string = "1+2/3+4/5   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let executor = _var_name_direct_left_recursion(Rules::test_term, &context, test_term);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 9));
    }

    #[test]
    fn test_left_recursion_indirect_5() {
        let string = "1-2-3  ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));

        let result: (bool, u32);
        {
            let involved_set = vec![Rules::test_indirect_LR_expr, Rules::test_indirect_LR_num];
            let executor = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_indirect_LR_expr,
                &context,
                test_indirect_lr_expr,
            );
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        // context.borrow().print_publisher();

        assert_eq!((result.0, result.1), (true, 5));
    }
    #[test]
    fn test_left_recursion_indirect_2() {
        let string = "1-2-3-7-9-1   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let involved_set = vec![Rules::test_indirect_LR_expr, Rules::test_indirect_LR_num];
            let executor = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_indirect_LR_expr,
                &context,
                test_indirect_lr_expr,
            );
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        assert_eq!((result.0, result.1), (true, 11));
    }
    #[test]
    fn test_fact_indirect_2() {
        let string = "1*2/5".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let involved_set = vec![Rules::test_fact_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_fact_indirect,
                &context,
                test_fact_indirect,
            );
            result = closure(Key(0), &source, 0);
        }
        // context.borrow().print_cache();

        // context.borrow().print_publisher();

        assert_eq!((result.0, result.1), (true, 5));
    }
    #[test]
    fn test_fact_indirect_1() {
        let string = "1*2/3*7/9".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let mut result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let involved_set = vec![Rules::test_fact_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_fact_indirect,
                &context,
                test_fact_indirect,
            );
            result = closure(Key(0), &source, 0);
        }
        // context.borrow().print_cache();

        // context.borrow().print_publisher();

        assert_eq!((result.0, result.1), (true, 9));
    }
    #[test]
    fn test_test_term_indirect_25() {
        let string = "1/2+3/4+5/9   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);

        {
            let involved_set = vec![Rules::test_term_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_term_indirect,
                &context,
                test_term_indirect,
            );
            result = closure(Key(0), &source, 0);
        }

        println!("Result: {:?}", result);
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 11));
        context
            .into_inner()
            .get_publisher()
            .print(Key(0), Some(true));
    }
    #[test]
    fn test_test_term_indirect_26() {
        let string = "1/2   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);

        {
            let involved_set = vec![Rules::test_term_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_term_indirect,
                &context,
                test_term_indirect,
            );
            result = closure(Key(0), &source, 0);
        }

        println!("Result: {:?}", result);
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 3));
        context
            .into_inner()
            .get_publisher()
            .print(Key(0), Some(true));
    }
    #[test]
    fn test_test_term_indirect_29() {
        let string = "1   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);

        {
            let involved_set = vec![Rules::test_term_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_term_indirect,
                &context,
                test_term_indirect,
            );
            result = closure(Key(0), &source, 0);
        }

        println!("Result: {:?}", result);
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 1));
        context
            .into_inner()
            .get_publisher()
            .print(Key(0), Some(true));
    }
    #[test]
    fn test_test_fact_indirect_26() {
        let string = "1/2   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);

        {
            let involved_set = vec![Rules::test_fact_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_fact_indirect,
                &context,
                test_fact_indirect,
            );
            result = closure(Key(0), &source, 0);
        }

        println!("Result: {:?}", result);
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 3));
    }
    #[test]
    fn test_test_fact_indirect_27() {
        let string = "1*2   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);

        {
            let involved_set = vec![Rules::test_fact_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_fact_indirect,
                &context,
                test_fact_indirect,
            );
            result = closure(Key(0), &source, 0);
        }

        println!("Result: {:?}", result);
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 3));
    }
    #[test]
    fn test_test_fact_indirect_28() {
        let string = "1   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);

        {
            let involved_set = vec![Rules::test_fact_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_fact_indirect,
                &context,
                test_fact_indirect,
            );
            result = closure(Key(0), &source, 0);
        }

        println!("Result: {:?}", result);
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 1));
    }

    #[test]
    fn test_fact_indirect_9() {
        let string = "1*2/3".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let mut result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let involved_set = vec![Rules::test_fact_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_fact_indirect,
                &context,
                test_fact_indirect,
            );
            result = closure(Key(0), &source, 0);
        }
        // context.borrow().print_cache();

        // context.borrow().print_publisher();

        assert_eq!((result.0, result.1), (true, 5));
    }
    #[test]
    fn test_test_term_indirect_1() {
        let string = "1+2/3+4/5+9   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);

        {
            let involved_set = vec![Rules::test_term_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_term_indirect,
                &context,
                test_term_indirect,
            );
            result = closure(Key(0), &source, 0);
        }

        println!("Result: {:?}", result);
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 11));
    }
    #[test]
    fn test_test_term_indirect_2() {
        let string = "1+2/3/7   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let involved_set = vec![Rules::test_term_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion2(
                &involved_set,
                Rules::test_term_indirect,
                &context,
                test_term_indirect,
            );
            result = closure(Key(0), &source, 0);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 7));
    }
}
