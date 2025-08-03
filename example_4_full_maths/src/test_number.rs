#[cfg(test)]
mod tests {
    use crate::evaluate_tree;
    use core::cell::RefCell;
    use example_4_full_maths_parser::*;

    #[test]
    fn test_1() {
        let string = "20".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let executor = _var_name(Rules::Number, &context, number);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 20);
    }
}
