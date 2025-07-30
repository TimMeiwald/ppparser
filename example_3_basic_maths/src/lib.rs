mod test_number;
use example_3_basic_maths_parser::*;
fn evaluate_tree(publisher: BasicPublisher, source: &String) -> i64 {
    let publisher = publisher.clear_false();
    let grammar_node = publisher.get_node(Key(0));
    evaluate_tree_kernel(&publisher, source, grammar_node)
}

fn evaluate_binary_node(publisher: &BasicPublisher, source: &String, node: &Node) -> (i64, i64) {
    // Eval Nodes with two children.
    let children = node.get_children();
    debug_assert!(children.len() == 2); // Must have two children exactly.
    let lhs = publisher.get_node(children[0]);
    let lhs_result = evaluate_tree_kernel(publisher, source, lhs);
    let rhs = publisher.get_node(children[1]);
    let rhs_result = evaluate_tree_kernel(publisher, source, rhs);
    (lhs_result, rhs_result)
}

fn evaluate_number(publisher: &BasicPublisher, source: &String, node: &Node) -> i64 {
    debug_assert!(node.rule == Rules::Number);
    debug_assert!(
        node.get_children().len() >= 1,
        "Must have at least one child, Integer"
    );
    let children = node.get_children();
    if children.len() > 1 {
        panic!("We do not currently support Sign/Exponent/Fraction");
    }
    let integer_node = publisher.get_node(children[0]);
    debug_assert!(integer_node.rule == Rules::Integer);

    let integer = integer_node.get_string(source);
    println!("Integer String: {integer}");
    let integer: i64 = integer.parse().expect("Should be valid integer");
    println!("Integer i64: {integer}");
    integer
}

fn evaluate_tree_kernel(publisher: &BasicPublisher, source: &String, node: &Node) -> i64 {
    let mut evaluation_result: Option<i64> = None;
    for child_key in node.get_children() {
        let child = publisher.get_node(*child_key);
        let result: i64;
        match child.rule {
            Rules::Grammar | Rules::Term | Rules::Factor | Rules::Expr | Rules::Parentheses => {
                result = evaluate_tree_kernel(publisher, source, child);
                println!("{:?} Result: {result}", child.rule);
                evaluation_result = Some(result);
            }
            Rules::Addition => {
                let (lhs, rhs) = evaluate_binary_node(publisher, source, child);
                println!("Addition Result: {lhs} + {rhs} = {}", lhs + rhs);
                evaluation_result = Some(lhs + rhs);
            }
            Rules::Subtraction => {
                let (lhs, rhs) = evaluate_binary_node(publisher, source, child);
                evaluation_result = Some(lhs - rhs);
                println!("Subtraction Result: {lhs} - {rhs} = {}", lhs - rhs);
            }
            Rules::Multiplication => {
                let (lhs, rhs) = evaluate_binary_node(publisher, source, child);
                evaluation_result = Some(lhs * rhs);
                println!("Multiplication Result: {lhs} * {rhs} = {}", lhs * rhs);
            }
            Rules::Division => {
                let (lhs, rhs) = evaluate_binary_node(publisher, source, child);
                evaluation_result = Some(lhs / rhs); // Div by Zero will fail this is not production grade.
                                                     // Could be easily handled by returning Result instead
                                                     // And throwing an error instead.
                println!("Division Result: {lhs} / {rhs} = {}", lhs / rhs);
            }
            Rules::Number => {
                let integer = evaluate_number(publisher, source, child);
                evaluation_result = Some(integer)
            }
            Rules::Exponent | Rules::Fraction | Rules::Integer | Rules::Sign => {
                // Should not be reached
                unreachable!("These rules can only be accessed via a number which get's handled by number function call!.")
            }
        }
    }
    match evaluation_result {
        Some(res) => {
            return res;
        }
        None => {
            panic!("Some fatal logical flaw has occurred in the program.")
        }
    }
}

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use example_3_basic_maths_parser::*;
    use std::env;
    use std::fs::{canonicalize, read_to_string};

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
            let involved_set: Vec<Rules> = vec![
                Rules::Subtraction,
                Rules::Addition,
                Rules::Expr,
                Rules::Term,
            ];
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Grammar, &context, grammar);
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
            let involved_set: Vec<Rules> = vec![
                Rules::Subtraction,
                Rules::Addition,
                Rules::Expr,
                Rules::Term,
            ];
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Grammar, &context, grammar);
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
            let involved_set: Vec<Rules> = vec![
                Rules::Subtraction,
                Rules::Addition,
                Rules::Expr,
                Rules::Term,
            ];
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Grammar, &context, grammar);
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
            let involved_set: Vec<Rules> = vec![
                Rules::Subtraction,
                Rules::Addition,
                Rules::Expr,
                Rules::Term,
            ];
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Grammar, &context, grammar);
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

    #[test]
    fn test_5() {
        let string = "7/5+3".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let involved_set: Vec<Rules> = vec![
                Rules::Subtraction,
                Rules::Addition,
                Rules::Expr,
                Rules::Term,
            ];
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_6() {
        let string = "7+5/3".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let involved_set: Vec<Rules> = vec![
                Rules::Subtraction,
                Rules::Addition,
                Rules::Expr,
                Rules::Term,
            ];
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_7() {
        let string = "7/5-3".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let involved_set: Vec<Rules> = vec![
                Rules::Subtraction,
                Rules::Addition,
                Rules::Expr,
                Rules::Term,
            ];
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, -2);
    }

    #[test]
    fn test_8() {
        let string = "7-5/3".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let involved_set: Vec<Rules> = vec![
                Rules::Subtraction,
                Rules::Addition,
                Rules::Expr,
                Rules::Term,
            ];
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {:?}", result);
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 6);
    }
}
