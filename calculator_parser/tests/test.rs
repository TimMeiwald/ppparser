#[cfg(test)]
mod tests {
    use calculator::*;
    use core::cell::RefCell;
    use std::env;
    use std::fs::{canonicalize, read_to_string};
    #[test]
    fn test_ppparser_dsl_grammar_rule_should_fail_on_calculator_since_different_grammar() {
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
        assert_eq!((result.0, result.1), (false, 0));
    }

    #[test]
    fn test_calc_basic99() {
        let string = "20/10+5".to_string();
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
    fn test_calc_basic100() {
        let string = "20+10/5".to_string();
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
    fn test_calc_basic111() {
        let string = "20/10".to_string();
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
    fn test_calc_basic() {
        let string = "20+10".to_string();
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
    fn test_calc_basic2() {
        let string = "20+10+10".to_string();
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
    fn test_calc_basic7() {
        let string = "20-10+10".to_string();
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
    fn test_calc_basic11() {
        let string = "20+10-10".to_string();
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
    fn test_calc_basic3() {
        let string = "20-10+10*10/5".to_string();
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
    fn test_calc_basic21() {
        let string = "20/10*10+10-5".to_string();
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
    fn test_calc_basic5() {
        let string = "20/10*10-10+5".to_string();
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
    fn test_calc_basic4() {
        let string = "20-10+10*10/5-2".to_string();
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
    fn test_calc_basic105() {
        let string = "20-10+10*10/5/20*100+7-9".to_string();
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
}
