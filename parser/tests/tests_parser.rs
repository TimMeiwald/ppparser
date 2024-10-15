#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use parser::*;
    use std::env;
    use std::fs::{canonicalize, read_to_string};

    #[test]
    fn test_grammar_true() {
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
    fn test_grammar_true3() {
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

        let result_tree = context.into_inner().get_publisher();
        // Create comparative BasicPublisher
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        // Result: (true, 11)
        // Key(0), Grammar, 0, 0, true, 1
        //     Key(16), test_LR_expr, 0, 11, true, 2
        //         Key(13), test_LR_expr, 0, 9, true, 2
        //             Key(10), test_LR_expr, 0, 7, true, 2
        //                 Key(7), test_LR_expr, 0, 5, true, 2
        //                     Key(4), test_LR_expr, 0, 3, true, 2
        //                         Key(1), test_LR_expr, 0, 1, true, 1
        //                             Key(2), test_LR_num, 0, 1, true, 1
        //                                 Key(3), Num, 0, 1, true, 0
        //                         Key(5), test_LR_num, 2, 3, true, 1
        //                             Key(6), Num, 2, 3, true, 0
        //                     Key(8), test_LR_num, 4, 5, true, 1
        //                         Key(9), Num, 4, 5, true, 0
        //                 Key(11), test_LR_num, 6, 7, true, 1
        //                     Key(12), Num, 6, 7, true, 0
        //             Key(14), test_LR_num, 8, 9, true, 1
        //                 Key(15), Num, 8, 9, true, 0
        //         Key(17), test_LR_num, 10, 11, true, 1
        //             Key(18), Num, 10, 11, true, 0

        // test_lr_expr
        let key_1 = expected_tree.add_node(Rules::test_LR_expr, 0, 11, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_LR_expr, 0, 9, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_LR_expr, 0, 7, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::test_LR_expr, 0, 5, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::test_LR_expr, 0, 3, true);
        expected_tree.connect(key_4, key_5);
        let key_6 = expected_tree.add_node(Rules::test_LR_expr, 0, 1, true);
        expected_tree.connect(key_5, key_6);
        // test_lr_num
        let key_7 = expected_tree.add_node(Rules::test_LR_num, 0, 1, true);
        expected_tree.connect(key_6, key_7);
        let key_8 = expected_tree.add_node(Rules::test_LR_num, 2, 3, true);
        expected_tree.connect(key_5, key_8);
        let key_9 = expected_tree.add_node(Rules::test_LR_num, 4, 5, true);
        expected_tree.connect(key_4, key_9);
        let key_10 = expected_tree.add_node(Rules::test_LR_num, 6, 7, true);
        expected_tree.connect(key_3, key_10);
        let key_11 = expected_tree.add_node(Rules::test_LR_num, 8, 9, true);
        expected_tree.connect(key_2, key_11);
        let key_12 = expected_tree.add_node(Rules::test_LR_num, 10, 11, true);
        expected_tree.connect(key_1, key_12);
        // Num
        let key_13 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_7, key_13);
        let key_14 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_8, key_14);
        let key_15 = expected_tree.add_node(Rules::Num, 4, 5, true);
        expected_tree.connect(key_9, key_15);
        let key_16 = expected_tree.add_node(Rules::Num, 6, 7, true);
        expected_tree.connect(key_10, key_16);
        let key_17 = expected_tree.add_node(Rules::Num, 8, 9, true);
        expected_tree.connect(key_11, key_17);
        let key_18 = expected_tree.add_node(Rules::Num, 10, 11, true);
        expected_tree.connect(key_12, key_18);

        println!("Expected tree:");
        expected_tree.print(Key(0), Some(true));
        assert_eq!(expected_tree, result_tree);
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

        let result_tree = context.into_inner().get_publisher();
        // Create comparative BasicPublisher
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        // Key(0), Grammar, 0, 0, true, 1
        //     Key(4), test_LR_expr, 0, 3, true, 2
        //         Key(1), test_LR_expr, 0, 1, true, 1
        //             Key(2), test_LR_num, 0, 1, true, 1
        //                 Key(3), Num, 0, 1, true, 0
        //         Key(5), test_LR_num, 2, 3, true, 1
        //             Key(6), Num, 2, 3, true, 0
        let key_1 = expected_tree.add_node(Rules::test_LR_expr, 0, 3, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_LR_expr, 0, 1, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_LR_num, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        let key_6 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_3, key_6);
        let key_4 = expected_tree.add_node(Rules::test_LR_num, 2, 3, true);
        expected_tree.connect(key_1, key_4);
        let key_5 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_4, key_5);
        println!("Expected tree:");
        expected_tree.print(Key(0), Some(true));
        assert_eq!(expected_tree, result_tree);
    }

    #[test]
    fn test_left_recursion_indirect_1() {
        let string = "1-2  ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));

        let result: (bool, u32);
        {
            let involved_set = vec![Rules::test_indirect_LR_expr, Rules::test_indirect_LR_num];
            let executor = _var_name_indirect_left_recursion(
                &involved_set,
                Rules::test_indirect_LR_expr,
                &context,
                test_indirect_lr_expr,
            );
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        //context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 3));

        let result_tree = context.into_inner().get_publisher().clear_false();
        result_tree.print(Key(0), Some(true));
        // Create comparative BasicPublisher
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        // Key(0), Grammar, 0, 0, true, 1
        //     Key(1), test_indirect_LR_expr, 0, 3, true, 2
        //         Key(2), test_indirect_LR_num, 0, 1, true, 1
        //             Key(3), test_indirect_LR_expr, 0, 1, true, 1
        //                 Key(4), test_LR_num, 0, 1, true, 1
        //                     Key(5), Num, 0, 1, true, 0
        //         Key(6), test_LR_num, 2, 3, true, 1
        //             Key(7), Num, 2, 3, true, 0
        let key_1 = expected_tree.add_node(Rules::test_indirect_LR_expr, 0, 3, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_indirect_LR_num, 0, 1, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_indirect_LR_expr, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::test_LR_num, 0, 1, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_4, key_5);
        let key_6 = expected_tree.add_node(Rules::test_LR_num, 2, 3, true);
        expected_tree.connect(key_1, key_6);
        let key_7 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_6, key_7);
        println!("Expected tree:");
        expected_tree.print(Key(0), Some(true));
        assert_eq!(expected_tree, result_tree);
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
            let executor = _var_name_indirect_left_recursion(
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

        let result_tree = context.into_inner().get_publisher().clear_false();

        result_tree.print(Key(0), Some(true));
        // Create comparative BasicPublisher
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        // Key(0), Grammar, 0, 0, true, 1
        //     Key(8), test_indirect_LR_expr, 0, 5, true, 2
        //         Key(9), test_indirect_LR_num, 0, 3, true, 1
        //             Key(10), test_indirect_LR_expr, 0, 3, true, 2
        //                 Key(2), test_indirect_LR_num, 0, 1, true, 1
        //                     Key(3), test_indirect_LR_expr, 0, 1, true, 1
        //                         Key(4), test_LR_num, 0, 1, true, 1
        //                             Key(5), Num, 0, 1, true, 0
        //                 Key(6), test_LR_num, 2, 3, true, 1
        //                     Key(7), Num, 2, 3, true, 0
        //         Key(11), test_LR_num, 4, 5, true, 1
        //             Key(12), Num, 4, 5, true, 0
        let key_a = expected_tree.add_node(Rules::test_indirect_LR_expr, 0, 5, true);
        expected_tree.connect(Key(0), key_a);
        let key_b = expected_tree.add_node(Rules::test_indirect_LR_num, 0, 3, true);
        expected_tree.connect(key_a, key_b);
        let key_1 = expected_tree.add_node(Rules::test_indirect_LR_expr, 0, 3, true);
        expected_tree.connect(key_b, key_1);
        let key_2 = expected_tree.add_node(Rules::test_indirect_LR_num, 0, 1, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_indirect_LR_expr, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::test_LR_num, 0, 1, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_4, key_5);
        let key_6 = expected_tree.add_node(Rules::test_LR_num, 2, 3, true);
        expected_tree.connect(key_1, key_6);
        let key_7 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_6, key_7);
        let key_8 = expected_tree.add_node(Rules::test_LR_num, 4, 5, true);
        expected_tree.connect(key_a, key_8);
        let key_9 = expected_tree.add_node(Rules::Num, 4, 5, true);
        expected_tree.connect(key_8, key_9);
        println!("Expected tree:");
        expected_tree.print(Key(0), Some(true));
        assert_eq!(expected_tree, result_tree);
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
            let executor = _var_name_indirect_left_recursion(
                &involved_set,
                Rules::test_indirect_LR_expr,
                &context,
                test_indirect_lr_expr,
            );
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        let p = context.into_inner().get_publisher();
        p.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, 11));
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
        let result_tree = context.into_inner().get_publisher();
        // Create comparative BasicPublisher
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        // Key(0), Grammar, 0, 0, true, 1
        //     Key(8), test_indirect_LR_expr, 0, 5, true, 2
        //         Key(9), test_indirect_LR_num, 0, 3, true, 1
        //             Key(10), test_indirect_LR_expr, 0, 3, true, 2
        //                 Key(2), test_indirect_LR_num, 0, 1, true, 1
        //                     Key(3), test_indirect_LR_expr, 0, 1, true, 1
        //                         Key(4), test_LR_num, 0, 1, true, 1
        //                             Key(5), Num, 0, 1, true, 0
        //                 Key(6), test_LR_num, 2, 3, true, 1
        //                     Key(7), Num, 2, 3, true, 0
        //         Key(11), test_LR_num, 4, 5, true, 1
        //             Key(12), Num, 4, 5, true, 0
        let key_1 = expected_tree.add_node(Rules::test_fact, 0, 9, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_fact, 0, 7, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_fact, 0, 5, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::test_fact, 0, 3, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::test_fact, 0, 1, true);
        expected_tree.connect(key_4, key_5);
        let key_6 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_5, key_6);
        let key_7 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_4, key_7);
        let key_8 = expected_tree.add_node(Rules::Num, 4, 5, true);
        expected_tree.connect(key_3, key_8);
        let key_9 = expected_tree.add_node(Rules::Num, 6, 7, true);
        expected_tree.connect(key_2, key_9);
        let key_10 = expected_tree.add_node(Rules::Num, 8, 9, true);
        expected_tree.connect(key_1, key_10);

        println!("Expected tree:");
        expected_tree.print(Key(0), Some(true));
        assert_eq!(expected_tree, result_tree);
    }

    #[test]
    fn test_fact_indirect_1() {
        let string = "1*2/3*7/9".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let involved_set = vec![Rules::test_fact_indirect];
            let executor = _var_name_indirect_left_recursion(
                &involved_set,
                Rules::test_fact_indirect,
                &context,
                test_fact_indirect,
            );
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();

        // context.borrow().print_publisher();

        assert_eq!((result.0, result.1), (true, 9));
        let result_tree = context.into_inner().get_publisher();
        println!("Result Tree:\n");
        result_tree.print(Key(0), None);
        // Create comparative BasicPublisher
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        // Key(0), Grammar, 0, 0, true, 1
        //     Key(8), test_indirect_LR_expr, 0, 5, true, 2
        //         Key(9), test_indirect_LR_num, 0, 3, true, 1
        //             Key(10), test_indirect_LR_expr, 0, 3, true, 2
        //                 Key(2), test_indirect_LR_num, 0, 1, true, 1
        //                     Key(3), test_indirect_LR_expr, 0, 1, true, 1
        //                         Key(4), test_LR_num, 0, 1, true, 1
        //                             Key(5), Num, 0, 1, true, 0
        //                 Key(6), test_LR_num, 2, 3, true, 1
        //                     Key(7), Num, 2, 3, true, 0
        //         Key(11), test_LR_num, 4, 5, true, 1
        //             Key(12), Num, 4, 5, true, 0
        let key_1 = expected_tree.add_node(Rules::test_fact_indirect, 0, 9, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_fact_indirect, 0, 7, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_fact_indirect, 0, 5, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::test_fact_indirect, 0, 3, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::test_fact_indirect, 0, 1, true);
        expected_tree.connect(key_4, key_5);
        let key_6 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_5, key_6);
        let key_7 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_4, key_7);
        let key_8 = expected_tree.add_node(Rules::Num, 4, 5, true);
        expected_tree.connect(key_3, key_8);
        let key_9 = expected_tree.add_node(Rules::Num, 6, 7, true);
        expected_tree.connect(key_2, key_9);
        let key_10 = expected_tree.add_node(Rules::Num, 8, 9, true);
        expected_tree.connect(key_1, key_10);

        println!("Expected tree:");
        expected_tree.print(Key(0), Some(true));
        assert_eq!(expected_tree, result_tree);
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

        // Key(0), Grammar, 0, 0, true, 1
        //     Key(11), test_term, 0, 9, true, 2
        //         Key(5), test_term, 0, 5, true, 2
        //             Key(1), test_term, 0, 1, true, 1
        //                 Key(2), test_fact, 0, 1, true, 1
        //                     Key(3), Num, 0, 1, true, 0
        //             Key(8), test_fact, 2, 5, true, 2
        //                 Key(6), test_fact, 2, 3, true, 1
        //                     Key(7), Num, 2, 3, true, 0
        //                 Key(9), Num, 4, 5, true, 0
        //         Key(14), test_fact, 6, 9, true, 2
        //             Key(12), test_fact, 6, 7, true, 1
        //                 Key(13), Num, 6, 7, true, 0
        //             Key(15), Num, 8, 9, true, 0
        let result_tree = context.into_inner().get_publisher();
        // Create comparative BasicPublisher
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        let key_1 = expected_tree.add_node(Rules::test_term, 0, 9, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_term, 0, 5, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_term, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::test_fact, 0, 1, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_4, key_5);
        let key_6 = expected_tree.add_node(Rules::test_fact, 2, 5, true);
        expected_tree.connect(key_2, key_6);
        let key_7 = expected_tree.add_node(Rules::test_fact, 2, 3, true);
        expected_tree.connect(key_6, key_7);
        let key_8 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_7, key_8);
        let key_9 = expected_tree.add_node(Rules::Num, 4, 5, true);
        expected_tree.connect(key_6, key_9);
        let key_10 = expected_tree.add_node(Rules::test_fact, 6, 9, true);
        expected_tree.connect(key_1, key_10);
        let key_11 = expected_tree.add_node(Rules::test_fact, 6, 7, true);
        expected_tree.connect(key_10, key_11);
        let key_12 = expected_tree.add_node(Rules::Num, 6, 7, true);
        expected_tree.connect(key_11, key_12);
        let key_13 = expected_tree.add_node(Rules::Num, 8, 9, true);
        expected_tree.connect(key_10, key_13);
        println!("Expected tree:");
        expected_tree.print(Key(0), Some(true));
        assert_eq!(expected_tree, result_tree);
    }
    #[test]
    fn test_left_recursion_indirect_8() {
        let string = "1+2/3+4/5   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let involved_set = vec![Rules::test_term_indirect];
            let executor = _var_name_indirect_left_recursion(
                &involved_set,
                Rules::test_term_indirect,
                &context,
                test_term_indirect,
            );
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        // context.borrow().print_cache();
        //context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 9));

        // Key(0), Grammar, 0, 0, true, 1
        //     Key(11), test_term, 0, 9, true, 2
        //         Key(5), test_term, 0, 5, true, 2
        //             Key(1), test_term, 0, 1, true, 1
        //                 Key(2), test_fact, 0, 1, true, 1
        //                     Key(3), Num, 0, 1, true, 0
        //             Key(8), test_fact, 2, 5, true, 2
        //                 Key(6), test_fact, 2, 3, true, 1
        //                     Key(7), Num, 2, 3, true, 0
        //                 Key(9), Num, 4, 5, true, 0
        //         Key(14), test_fact, 6, 9, true, 2
        //             Key(12), test_fact, 6, 7, true, 1
        //                 Key(13), Num, 6, 7, true, 0
        //             Key(15), Num, 8, 9, true, 0
        let result_tree = context.into_inner().get_publisher().clear_false();
        // Create comparative BasicPublisher
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        let key_1 = expected_tree.add_node(Rules::test_term, 0, 9, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_term, 0, 5, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_term, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::test_fact, 0, 1, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_4, key_5);
        let key_6 = expected_tree.add_node(Rules::test_fact, 2, 5, true);
        expected_tree.connect(key_2, key_6);
        let key_7 = expected_tree.add_node(Rules::test_fact, 2, 3, true);
        expected_tree.connect(key_6, key_7);
        let key_8 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_7, key_8);
        let key_9 = expected_tree.add_node(Rules::Num, 4, 5, true);
        expected_tree.connect(key_6, key_9);
        let key_10 = expected_tree.add_node(Rules::test_fact, 6, 9, true);
        expected_tree.connect(key_1, key_10);
        let key_11 = expected_tree.add_node(Rules::test_fact, 6, 7, true);
        expected_tree.connect(key_10, key_11);
        let key_12 = expected_tree.add_node(Rules::Num, 6, 7, true);
        expected_tree.connect(key_11, key_12);
        let key_13 = expected_tree.add_node(Rules::Num, 8, 9, true);
        expected_tree.connect(key_10, key_13);
        println!("Expected tree:");
        expected_tree.print(Key(0), Some(true));
        assert_eq!(expected_tree, result_tree);
    }
}
