#[cfg(test)]
mod tests {
    use ::example_8_indirect_lr_3_level_parser::*;
    use core::cell::RefCell;

    #[test]
    fn test_left_recursion_1() {
        let string = "1-2-3-7-9-1   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        let result: (bool, u32);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        // context.borrow().print_cache();
        // context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 11));
        let result_tree = context.into_inner().get_publisher();
        result_tree.print(Key(0), Some(true));
    }
    #[test]
    fn test_left_recursion_2() {
        let string = "1-2   ".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        let result: (bool, u32);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        // context.borrow().print_cache();
        // context.borrow().print_publisher();
        assert_eq!((result.0, result.1), (true, 3));

        let result_tree = context.into_inner().get_publisher();
        result_tree.print(Key(0), Some(true));
    }
}
