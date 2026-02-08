use c_parser::{
    _var_name, BasicContext, Context, Key, RULES_SIZE, Rules, Source, UserState, grammar,
};
use std::cell::RefCell;
use std::fs;
use test_grammar_proc_macro::test_grammar_files_in_dir;

pub fn shared(
    input: &str,
    func: fn(&RefCell<UserState>, Key, &RefCell<BasicContext>, &Source, u32) -> (bool, u32),
    rule: Rules,
) -> (bool, u32) {
    let string = input.to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let result: (bool, u32);
    let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    let user_state: RefCell<UserState> = RefCell::new(UserState::default());
    {
        let executor = _var_name(&user_state, rule, &context, func);
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

test_grammar_files_in_dir!("examples/c_parser/tests/example_files");
