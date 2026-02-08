mod test_number;
use example_4_full_maths_parser::*;
#[allow(dead_code)]
fn evaluate_tree(publisher: BasicPublisher, source: &String) -> i64 {
    let publisher = publisher.clear_false();
    let grammar_node = publisher.get_node(Key(0));
    evaluate_tree_kernel(&publisher, source, grammar_node)
}
#[allow(dead_code)]
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
#[allow(dead_code)]
fn evaluate_number(publisher: &BasicPublisher, source: &str, node: &Node) -> i64 {
    debug_assert!(node.rule == Rules::Number);
    debug_assert!(
        !node.get_children().is_empty(),
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
#[allow(dead_code)]
fn evaluate_tree_kernel(publisher: &BasicPublisher, source: &String, node: &Node) -> i64 {
    let mut evaluation_result: Option<i64> = None;
    for child_key in node.get_children() {
        let child = publisher.get_node(*child_key);
        let result: i64;
        match child.rule {
            Rules::Grammar
            | Rules::Parentheses
            | Rules::Term
            | Rules::Factor
            | Rules::Expr
            | Rules::Power_expr => {
                result = evaluate_tree_kernel(publisher, source, child);
                println!("{:?} Result: {result}", child.rule);
                evaluation_result = Some(result);
            }
            Rules::Power => {
                let (lhs, rhs) = evaluate_binary_node(publisher, source, child);
                let res = lhs.pow(rhs as u32);
                println!("{lhs}, {rhs}, {res:?}");
                println!("Power Result: {lhs} ^ {rhs} = {res:?}");
                evaluation_result = Some(res);
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
        Some(res) => res,
        None => {
            panic!("Some fatal logical flaw has occurred in the program.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluate_tree;
    use core::cell::RefCell;
    use example_4_full_maths_parser::*;
    #[test]
    fn test_1() {
        let string = "20+10".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
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
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
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
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
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
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
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
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
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
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
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
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
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
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_9() {
        let string = "20+10+20+40".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 90);
    }

    #[test]
    fn test_10() {
        let string = "7+5-3-60".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, -51);
    }

    #[test]
    fn test_11() {
        let string = "7/5+3-20".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, -16);
    }

    #[test]
    fn test_12() {
        let string = "7+5/3+40".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_13() {
        let string = "7/5-3+15".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_14() {
        let string = "7-5/3*4".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 3);
    }
    #[test]
    fn test_15() {
        let string = "(7-5)/(3*4)".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 0);
    }
    #[test]
    fn test_16() {
        let string = "(7-5)/(3*4)*20/2+5-19".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, -14);
    }
    #[test]
    fn test_17() {
        let string = "7^5".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 16807);
    }
    #[test]
    fn test_18() {
        let string = "7+4^5".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 1031);
    }
    #[test]
    fn test_19() {
        let string = "7^5+4".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 16811);
    }
    #[test]
    fn test_20() {
        let string = "7^(5+4)".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 40353607);
    }
    #[test]
    fn test_21() {
        let string = "(7/5)^4".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Grammar, &context, grammar);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_22() {
        let string = "20/10".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Term, &context, term);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_29() {
        let string = "20-10".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        let user_state: RefCell<UserState> = RefCell::new(UserState);
        {
            let executor = _var_name(&user_state, Rules::Expr, &context, expr);
            result = executor(Key(0), &source, position);
        }
        println!("Result: {result:?}");
        //context.borrow().print_cache();
        //context.borrow().print_publisher();
        let publisher = context.into_inner().get_publisher();
        publisher.print(Key(0), Some(true));
        assert_eq!((result.0, result.1), (true, src_len));
        let result = evaluate_tree(publisher, &string);
        assert_eq!(result, 10);
    }
}
