#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use ppparser::*;
    use std::env;
    use std::fs::{canonicalize, read_to_string};

    #[test]
    fn test_grammar_true() {
        let string = "<Spaces> PASSTHROUGH = '\n'/'\t'/'\r'/' ';".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let executor = _var_name(Rules::Grammar, &context, grammar);
        let result = executor(Key(0), &source, position);
        println!("Result: {:?}", result);
        //context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, src_len));
    }

    #[test]
    fn test_grammar_true3() {
        println!("{:?}", env::current_dir().unwrap());
        let path = "tests/test_data/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");

        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let executor = _var_name(Rules::Grammar, &context, grammar);
        let result = executor(Key(0), &source, position);
        println!("Result: {:?}", result);
        //context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, src_len));
    }
    #[test]
    fn test_left_recursion_direct_1() {
        let string = "1-2-3-7-9-1   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let executor = _var_name_direct_left_recursion(Rules::test_LR_expr, &context, test_lr_expr);
        let result = executor(Key(0), &source, position);
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
        let executor = _var_name_direct_left_recursion(Rules::test_LR_expr, &context, test_lr_expr);
        let result = executor(Key(0), &source, position);
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 3));
    }

    #[test]
    fn test_left_recursion_indirect_1() {
        let string = "1-2  ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let involved_set = vec![Rules::test_indirect_LR_expr, Rules::test_indirect_LR_num];
        let executor = _var_name_indirect_left_recursion(
            &involved_set,
            Rules::test_indirect_LR_expr,
            &context,
            test_indirect_lr_expr,
        );
        let result = executor(Key(0), &source, position);
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 3));
    }

    #[test]
    fn test_left_recursion_indirect_2() {
        let string = "1-2-3-7-9-1   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let involved_set = vec![Rules::test_indirect_LR_expr, Rules::test_indirect_LR_num];
        let executor = _var_name_indirect_left_recursion(
            &involved_set,
            Rules::test_indirect_LR_expr,
            &context,
            test_indirect_lr_expr,
        );
        let result = executor(Key(0), &source, position);
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 11));
    }

    #[test]
    fn test_left_recursion_direct_3() {
        let string = "1*2/3*7/9   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let executor = _var_name_direct_left_recursion(Rules::test_fact, &context, test_fact);
        let result = executor(Key(0), &source, position);
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
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
        let executor = _var_name_direct_left_recursion(Rules::test_term, &context, test_term);
        let result = executor(Key(0), &source, position);
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 9));
    }
}
