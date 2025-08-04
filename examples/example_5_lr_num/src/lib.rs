use example_5_lr_num_parser::*;


#[cfg(test)]
mod tests {
    use ::example_5_lr_num_parser::*;
    use core::cell::RefCell;

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
                _var_name(Rules::Expr, &context, expr);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
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
        let key_1 = expected_tree.add_node(Rules::Expr, 0, 11, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::Expr, 0, 9, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::Expr, 0, 7, true);
        expected_tree.connect(key_2, key_3);
        let key_4 = expected_tree.add_node(Rules::Expr, 0, 5, true);
        expected_tree.connect(key_3, key_4);
        let key_5 = expected_tree.add_node(Rules::Expr, 0, 3, true);
        expected_tree.connect(key_4, key_5);
        let key_6 = expected_tree.add_node(Rules::Expr, 0, 1, true);
        expected_tree.connect(key_5, key_6);
        // test_lr_num
        let key_7 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_6, key_7);
        let key_8 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_5, key_8);
        let key_9 = expected_tree.add_node(Rules::Num, 4, 5, true);
        expected_tree.connect(key_4, key_9);
        let key_10 = expected_tree.add_node(Rules::Num, 6, 7, true);
        expected_tree.connect(key_3, key_10);
        let key_11 = expected_tree.add_node(Rules::Num, 8, 9, true);
        expected_tree.connect(key_2, key_11);
        let key_12 = expected_tree.add_node(Rules::Num, 10, 11, true);
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
                _var_name(Rules::Expr, &context, expr);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
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
        let key_1 = expected_tree.add_node(Rules::Expr, 0, 3, true);
        expected_tree.connect(Key(0), key_1);
        let key_2 = expected_tree.add_node(Rules::Expr, 0, 1, true);
        expected_tree.connect(key_1, key_2);
        let key_3 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_2, key_3);
        let key_6 = expected_tree.add_node(Rules::Num, 0, 1, true);
        expected_tree.connect(key_3, key_6);
        let key_4 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_1, key_4);
        let key_5 = expected_tree.add_node(Rules::Num, 2, 3, true);
        expected_tree.connect(key_4, key_5);
        println!("Expected tree:");
        expected_tree.print(Key(0), Some(true));
        assert_eq!(expected_tree, result_tree);
    }

}
