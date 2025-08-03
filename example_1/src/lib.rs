use example_1_parser::*;

#[allow(dead_code)]
fn evaluate_tree(publisher: BasicPublisher, source: &String) -> i64 {
    let publisher = publisher.clear_false();
    let grammar_node = publisher.get_node(Key(0));
    evaluate_tree_kernel(&publisher, source, grammar_node)
}
#[allow(dead_code)]
fn evaluate_tree_kernel(publisher: &BasicPublisher, source: &String, node: &Node) -> i64 {
    let mut results: Vec<i64> = Vec::new();
    for child_key in node.get_children() {
        let child = publisher.get_node(*child_key);
        let result: i64;
        match child.rule {
            Rules::Grammar => {
                result = evaluate_tree_kernel(publisher, source, child);
                println!("Grammar Result: {result}");
            }
            Rules::Expr => {
                result = evaluate_tree_kernel(publisher, source, child);
                println!("Expression Result: {result}");
            }
            Rules::Addition => {
                result = evaluate_tree_kernel(publisher, source, child);
                println!("Addition Result: {result}");
            }
            Rules::Subtraction => {
                result = evaluate_tree_kernel(publisher, source, child);
                println!("Subtraction Result: {result}");
            }
            Rules::Integer => {
                let integer = child.get_string(source);
                println!("Integer String: {integer}");
                let integer: i64 = integer.parse().expect("Should be valid integer");
                result = integer;
                println!("Integer Result: {result}");
            }
        }
        results.push(result);
    }
    println!("Results: {results:?}");
    if node.rule == Rules::Subtraction {
        results[1] = -results[1]
    }
    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use example_1_parser::*;
    use crate::evaluate_tree;

    #[test]
    fn test_1() {
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
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_2() {
        let string = "20-10".to_string();
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
        assert_eq!(result, 10);
    }

    #[test]
    fn test_3() {
        let string = "20+10+20".to_string();
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
        assert_eq!(result, 50);
    }

    #[test]
    fn test_4() {
        let string = "7+5-3".to_string();
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
        assert_eq!(result, 9);
    }
}
