use example_2_parser::*;
#[allow(dead_code)]
fn evaluate_tree(publisher: BasicPublisher, source: &String) -> String {
    let publisher = publisher.clear_false();
    let grammar_node = publisher.get_node(Key(0));
    evaluate_tree_kernel(&publisher, source, grammar_node)
}
#[allow(dead_code)]
fn evaluate_tree_kernel(publisher: &BasicPublisher, source: &String, node: &Node) -> String {
    let mut results: String = "".to_string();
    for child_key in node.get_children() {
        let child = publisher.get_node(*child_key);
        let mut result: String;
        match child.rule {
            Rules::Grammar => {
                result = evaluate_tree_kernel(publisher, source, child);
                println!("Grammar Result: {result}");
            }
            Rules::A => {
                result = evaluate_tree_kernel(publisher, source, child);

                result += "a";
            }
            Rules::B => {
                result = evaluate_tree_kernel(publisher, source, child);
                result += "b";
            }
        }
        results.push_str(&result);
    }
    return results;
}

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use example_2_parser::*;
    use crate::evaluate_tree;

    #[test]
    fn test_1() {
        let string = "ababababa".to_string();
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
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, "ababababa");
    }

    #[test]
    fn test_2() {
        let string = "babababababa".to_string();
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
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (false, 0));
    }

    #[test]
    fn test_3() {
        let string = "bababababab".to_string();
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
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (false, 0));
        // let result = evaluate_tree(publisher, &string);
        // assert_eq!(result, 50);
    }

    #[test]
    fn test_4() {
        let string = "aba".to_string();
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
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        // let result = evaluate_tree(publisher, &string);
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, "aba");
    }

    #[test]
    fn test_5() {
        let string = "a".to_string();
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
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        // let result = evaluate_tree(publisher, &string);
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, "a");
    }

    #[test]
    fn test_6() {
        let string = "b".to_string();
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
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (false, 0));
        // let result = evaluate_tree(publisher, &string);
    }
}
