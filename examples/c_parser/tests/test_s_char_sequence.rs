mod test;

use std::cell::RefCell;

use c_parser::{BasicContext, Rules, s_char_sequence, *};

pub fn shared(
    input: &str,
    func: fn(&RefCell<UserState>, Key, &RefCell<BasicContext>, &Source, u32) -> (bool, u32),
    rule: Rules,
) -> (bool, u32) {
    let string = input.to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let result: (bool, u32);
    let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    let user_state: RefCell<UserState> = RefCell::new(UserState::default());
    {
        let involved_set: Vec<Rules> = [Rules::S_char_sequence].to_vec();
        let executor =
            _var_name_indirect_left_recursion(&user_state, &involved_set, rule, &context, func);
        result = executor(Key(0), &source, 0);
    }
    println!("Result: {:?}", result);
    //context.borrow().print_cache();
    //context.borrow().print_publisher();
    //context.borrow().print_node(Key(0));
    let publisher = context.into_inner().get_publisher().clear_false();
    publisher.print(Key(0), Some(true));
    result
}

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(src, s_char_sequence::<BasicContext>, Rules::S_char_sequence);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_2() {
    let src = "struct\n";
    let result = shared(src, s_char_sequence::<BasicContext>, Rules::S_char_sequence);
    assert_eq!(result, (true, (src.len() - 1) as u32));
}
#[test]
fn test_3() {
    let src = "s ";
    let result = shared(src, s_char_sequence::<BasicContext>, Rules::S_char_sequence);
    assert_eq!(result, (true, src.len() as u32));
}
