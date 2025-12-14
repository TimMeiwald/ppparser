use c_parser::{_var_name, BasicContext, Context, Key, RULES_SIZE, Rules, Source, grammar};
use std::cell::RefCell;
use std::fs;
use test_grammar_proc_macro::test_grammar_files_in_dir;

pub fn shared<'a>(
    input: &str,
    func: for<'c> fn(Key, &RefCell<BasicContext>, &Source<'c>, u32) -> (bool, u32),
    rule: Rules,
) -> (bool, u32) {
    let string = input.to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let result: (bool, u32);
    let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    {
        let executor = _var_name(rule, &context, func);
        result = executor(Key(0), &source, position);
    }
    println!("Result: {:?}", result);
    //context.borrow().print_cache();
    //context.borrow().print_publisher();
    //context.borrow().print_node(Key(0));
    let publisher = context.into_inner().get_publisher().clear_false();
    publisher.print(Key(0), Some(true));
    result
}

test_grammar_files_in_dir!("examples/c_parser/tests/example_files/structs/");

test_grammar_files_in_dir!("examples/c_parser/tests/example_files");
