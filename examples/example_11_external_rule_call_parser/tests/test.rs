#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use example_11_external_rule_call_parser::*;
    use std::env;
    use std::fs::{canonicalize, read_to_string};
    #[test]
    fn test_ppparser_dsl_grammar_rule() {
        let string = "Hello World!".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let result: (bool, u32);
        let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
        {
            let executor = _var_name(Rules::Grammar, &context, grammar);
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
}
