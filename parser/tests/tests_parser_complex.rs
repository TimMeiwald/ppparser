// These tests are the same as in tests_parser_basic
// The difference is that here we also check the output AST.
#[cfg(test)]
mod tests {
    use ::parser::*;
    use core::cell::RefCell;

    // #[test]
    // fn test_ppparser_dsl_grammar_rule() {
    //     let string = "<Spaces> PASSTHROUGH = '\n'/'\t'/'\r'/' ';".to_string();
    //     let src_len = string.len() as u32;
    //     let source = Source::new(&string);
    //     let position: u32 = 0;
    //     let result: (bool, u32);
    //     let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    //     {
    //         let executor = _var_name(Rules::Grammar, &context, grammar);
    //         result = executor(Key(0), &source, position);
    //     }
    //     println!("Result: {:?}", result);
    //     //context.borrow().print_cache();
    //     //context.borrow().print_publisher();
    //     context
    //         .into_inner()
    //         .get_publisher().clear_false()
    //
    //         .print(Key(0), None);
    //     assert_eq!((result.0, result.1), (true, src_len));
    //     // Key(0), Grammar, 0, 0, true, 1
    //     //     Key(2), Grammar, 0, 39, true, 1
    //     //         Key(3), Rule, 0, 39, true, 2
    //     //             Key(4), LHS, 0, 21, true, 2
    //     //                 Key(5), Var_Name_Decl, 0, 8, true, 0
    //     //                 Key(6), Semantic_Instructions, 9, 20, true, 1
    //     //                     Key(7), Passthrough, 9, 20, true, 0
    //     //             Key(8), RHS, 23, 38, true, 1
    //     //                 Key(9), Ordered_Choice, 23, 38, true, 4
    //     //                     Key(10), Atom, 23, 26, true, 1
    //     //                         Key(11), Nucleus, 23, 26, true, 1
    //     //                             Key(12), Terminal, 23, 26, true, 1
    //     //                                 Key(13), ASCII, 24, 25, true, 0
    //     //                     Key(14), Atom, 27, 30, true, 1
    //     //                         Key(15), Nucleus, 27, 30, true, 1
    //     //                             Key(16), Terminal, 27, 30, true, 1
    //     //                                 Key(17), ASCII, 28, 29, true, 0
    //     //                     Key(18), Atom, 31, 34, true, 1
    //     //                         Key(19), Nucleus, 31, 34, true, 1
    //     //                             Key(20), Terminal, 31, 34, true, 1
    //     //                                 Key(21), ASCII, 32, 33, true, 0
    //     //                     Key(22), Atom, 35, 38, true, 1
    //     //                         Key(23), Nucleus, 35, 38, true, 1
    //     //                             Key(24), Terminal, 35, 38, true, 1
    //     //                                 Key(25), ASCII, 36, 37, true, 0

    //     todo!("Add AST test - Need to make a test AST builder tool really.")
    // }

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
        println!("Result: {result:?}");
        // context.borrow().print_cache();
        // context.borrow().print_publisher();

        assert_eq!((result.0, result.1), (true, 5));
        let result_tree = context.into_inner().get_publisher().clear_false();

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
        expected_tree.print(Key(0), None);
        assert_eq!(expected_tree, result_tree);
    }

    #[test]
    fn test_left_recursion_indirect_2() {
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
        println!("Result: {result:?}");
        // context.borrow().print_cache();
        // context.borrow().print_publisher();

        assert_eq!((result.0, result.1), (true, 3));
        let result_tree = context.into_inner().get_publisher().clear_false();

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
        expected_tree.print(Key(0), None);
        assert_eq!(expected_tree, result_tree);
    }

    #[test]
    fn test_left_recursion_indirect_84() {
        let string = "1  ".to_string();
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
        println!("Result: {result:?}");
        // context.borrow().print_cache();
        // context.borrow().print_publisher();

        assert_eq!((result.0, result.1), (true, 1));
        let result_tree = context.into_inner().get_publisher().clear_false();

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
        let key_1 = expected_tree.add_node(Rules::test_indirect_LR_expr, 0, 1, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_LR_num, 0, 1, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        println!("Expected tree:");
        expected_tree.print(Key(0), None);
        assert_eq!(expected_tree, result_tree);
    }

    // #[test]
    // fn test_left_recursion_indirect_2() {
    //     let string = "1-2-3-7-9-1   ".to_string();
    //     let src_len = string.len() as u32;
    //     let source = Source::new(&string);
    //     let position: u32 = 0;
    //     let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    //     let result: (bool, u32);
    //     {
    //         let involved_set = vec![Rules::test_indirect_LR_expr, Rules::test_indirect_LR_num];
    //         let executor = _var_name_indirect_left_recursion(
    //             &involved_set,
    //             Rules::test_indirect_LR_expr,
    //             &context,
    //             test_indirect_lr_expr,
    //         );
    //         result = executor(Key(0), &source, position);
    //     }
    //     println!("Result: {:?}", result);
    //     // context.borrow().print_cache();
    //     assert_eq!((result.0, result.1), (true, 11));
    //     let p = context.into_inner().get_publisher().clear_false();
    //     p.print(Key(0), None);
    //     todo!("Add AST")
    // }
    // #[test]
    // fn test_fact_indirect_2() {
    //     let string = "1*2/5".to_string();
    //     let src_len = string.len() as u32;
    //     let source = Source::new(&string);
    //     let position: u32 = 0;
    //     let result: (bool, u32);
    //     let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    //     {
    //         let involved_set = vec![Rules::test_fact_indirect];
    //         //result = test_fact_indirect(Key(0), &context, &source, position);
    //         let closure = _var_name_indirect_left_recursion(
    //             &involved_set,
    //             Rules::test_fact_indirect,
    //             &context,
    //             test_fact_indirect,
    //         );
    //         result = closure(Key(0), &source, 0);
    //     }
    //     // context.borrow().print_cache();

    //     // context.borrow().print_publisher();

    //     assert_eq!((result.0, result.1), (true, 5));
    //     todo!("Add AST");
    // }
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
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion(
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
        let result_tree = context.into_inner().get_publisher().clear_false();
        // println!("Result Tree:\n");
        result_tree.print(Key(0), None);
        // Create comparative BasicPublisher
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);

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
        expected_tree.print(Key(0), None);

        assert_eq!(expected_tree, result_tree);
    }

    #[test]
    fn test_test_fact_indirect_29() {
        let string = "1   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);

        {
            let involved_set = vec![];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion(
                &involved_set,
                Rules::test_fact_indirect,
                &context,
                test_fact_indirect,
            );
            result = closure(Key(0), &source, position);
        }

        println!("Result: {result:?}");
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 1));
        let result_tree = context.into_inner().get_publisher().clear_false();
        result_tree.print(Key(0), None);
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        let key_1 = expected_tree.add_node(Rules::test_fact_indirect, 0, 1, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_1, key_2);

        println!("Expected Tree:");
        expected_tree.print(Key(0), None);
        assert_eq!(expected_tree, result_tree);
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

            let closure = _var_name_indirect_left_recursion(
                &involved_set,
                Rules::test_term_indirect,
                &context,
                test_term_indirect,
            );
            result = closure(Key(0), &source, 0);
        }

        println!("Result: {result:?}");
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 1));
        let result_tree = context.into_inner().get_publisher().clear_false();
        result_tree.print(Key(0), None);
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        let key_1 = expected_tree.add_node(Rules::test_term_indirect, 0, 1, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_fact_indirect, 0, 1, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        println!("Expected Tree:");
        expected_tree.print(Key(0), None);
        assert_eq!(expected_tree, result_tree);
    }

    #[test]
    fn test_test_term_indirect_1() {
        let string = "1+2/3+4/5   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);

        {
            let involved_set = vec![Rules::test_term_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion(
                &involved_set,
                Rules::test_term_indirect,
                &context,
                test_term_indirect,
            );
            result = closure(Key(0), &source, 0);
        }

        println!("Result: {result:?}");
        // context.borrow().print_cache();

        assert_eq!((result.0, result.1), (true, 9));
        //context.borrow().print_publisher();

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
        result_tree.print(Key(0), None);

        // Create comparative BasicPublisher
        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        let key_1 = expected_tree.add_node(Rules::test_term_indirect, 0, 9, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_term_indirect, 0, 5, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_term_indirect, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::test_fact_indirect, 0, 1, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_4, key_5);
        let key_6 = expected_tree.add_node(Rules::test_fact_indirect, 2, 5, true);
        expected_tree.connect(key_2, key_6);
        let key_7 = expected_tree.add_node(Rules::test_fact_indirect, 2, 3, true);
        expected_tree.connect(key_6, key_7);
        let key_8 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_7, key_8);
        let key_9 = expected_tree.add_node(Rules::Num, 4, 5, true);
        expected_tree.connect(key_6, key_9);
        let key_10 = expected_tree.add_node(Rules::test_fact_indirect, 6, 9, true);
        expected_tree.connect(key_1, key_10);
        let key_11 = expected_tree.add_node(Rules::test_fact_indirect, 6, 7, true);
        expected_tree.connect(key_10, key_11);
        let key_12 = expected_tree.add_node(Rules::Num, 6, 7, true);
        expected_tree.connect(key_11, key_12);
        let key_13 = expected_tree.add_node(Rules::Num, 8, 9, true);
        expected_tree.connect(key_10, key_13);
        println!("Expected tree:");
        expected_tree.print(Key(0), None);
        assert_eq!(expected_tree, result_tree);
    }
    // #[test]
    // fn test_test_term_indirect_2() {
    //     let string = "1+2/3+4/5/7/9   ".to_string();
    //     let src_len = string.len() as u32;
    //     let source = Source::new(&string);
    //     let position: u32 = 0;
    //     let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    //     let result: (bool, u32);
    //     {
    //         let involved_set = vec![Rules::test_term_indirect];
    //         //result = test_fact_indirect(Key(0), &context, &source, position);
    //         let closure = _var_name_indirect_left_recursion(
    //             &involved_set,
    //             Rules::test_term_indirect,
    //             &context,
    //             test_term_indirect,
    //         );
    //         result = closure(Key(0), &source, 0);
    //     }
    //     println!("Result: {:?}", result);
    //     // context.borrow().print_cache();

    //     assert_eq!((result.0, result.1), (true, 13));
    //     todo!("Add AST")
    // }
    #[test]
    fn test_fact_indirect_9() {
        let string = "1*2/3".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let involved_set = vec![Rules::test_fact_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion(
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

        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);

        let key_1 = expected_tree.add_node(Rules::test_fact_indirect, 0, 5, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::test_fact_indirect, 0, 3, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_fact_indirect, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_2, key_5);
        let key_6 = expected_tree.add_node(Rules::Num, 4, 5, true);
        expected_tree.connect(key_1, key_6);

        let result_tree = context.into_inner().get_publisher().clear_false();
        result_tree.print(Key(0), None);
        println!("Expected tree:");
        expected_tree.print(Key(0), None);
        assert_eq!(expected_tree, result_tree);
    }
    #[test]
    fn test_fact_indirect_11() {
        let string = "1*2/3/7".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let involved_set = vec![Rules::test_fact_indirect];
            //result = test_fact_indirect(Key(0), &context, &source, position);
            let closure = _var_name_indirect_left_recursion(
                &involved_set,
                Rules::test_fact_indirect,
                &context,
                test_fact_indirect,
            );
            result = closure(Key(0), &source, 0);
        }
        // context.borrow().print_cache();

        // context.borrow().print_publisher();

        assert_eq!((result.0, result.1), (true, 7));

        let mut expected_tree = BasicPublisher::new(src_len as usize, RULES_SIZE as usize);
        let key_0 = expected_tree.add_node(Rules::test_fact_indirect, 0, 7, true);
        expected_tree.connect(Key(0), key_0);
        let key_1 = expected_tree.add_node(Rules::test_fact_indirect, 0, 5, true);
        expected_tree.connect(key_0, key_1);
        let key_2 = expected_tree.add_node(Rules::test_fact_indirect, 0, 3, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::test_fact_indirect, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_2, key_5);
        let key_6 = expected_tree.add_node(Rules::Num, 4, 5, true);
        expected_tree.connect(key_1, key_6);
        let key_7 = expected_tree.add_node(Rules::Num, 6, 7, true);
        expected_tree.connect(key_0, key_7);

        let result_tree = context.into_inner().get_publisher().clear_false();
        result_tree.print(Key(0), None);
        println!("Expected tree:");
        expected_tree.print(Key(0), None);
        assert_eq!(expected_tree, result_tree);
    }
}
