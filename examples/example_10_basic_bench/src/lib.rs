#[cfg(test)]
mod tests {
    use ::example_10_basic_bench_parser::*;
    use core::cell::RefCell;

    #[test]
    fn test_rr_1() {
        let string = "1".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let executor = _var_name(Rules::Rr, &context, rr);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        let tree = context.into_inner();
        let publisher = tree.get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
    }

    #[test]
    fn test_rr_2() {
        let string = "1111111".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let executor = _var_name(Rules::Rr, &context, rr);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        let tree = context.into_inner();
        let publisher = tree.get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
    }
    #[test]
    fn test_lr_1() {
        let string = "1".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let involved_set: Vec<Rules> = [Rules::Lr].to_vec();
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Lr, &context, lr);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        let tree = context.into_inner();
        let publisher = tree.get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
    }
    #[test]
    fn test_lr_2() {
        let string = "1111111".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let result: (bool, u32);
        {
            let involved_set: Vec<Rules> = [Rules::Lr].to_vec();
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Lr, &context, lr);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        let tree = context.into_inner();
        let publisher = tree.get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
    }
}
