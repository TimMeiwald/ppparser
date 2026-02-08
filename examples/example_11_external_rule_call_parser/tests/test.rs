#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use example_11_external_rule_call_parser::*;
    use std::collections::{HashMap, HashSet};
    use std::env;
    use std::fs::{canonicalize, read_to_string};

    #[test]
    fn test_example11() {
        let string = "hello world\n".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let user_state = RefCell::new(UserState::new());

        let context: RefCell<BasicContext> =
            RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
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
    fn test_example11_multiline() {
        // Should only match first two strings with context sensitivity
        // Because whilst the third line is a valid string it then get's rewritten as a failure
        // because it already exists in the cache(the hashset in user_state that the hooked_call in hooked_call.rs manipulates.)
        // Both hooked_call.rs and user_state.rs do not get regenerated if it already exists. If you wish to regenerate thems simply delete then
        // and rerun the generator.
        let string = "hello world\nhello there\nhello world\n".to_string();
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
        assert_eq!((result.0, result.1), (true, 24));
    }
}
