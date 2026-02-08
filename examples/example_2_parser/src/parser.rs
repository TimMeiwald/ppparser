#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn a<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::A, Rules::B].to_vec();
    let closure_1 =
        _var_name_indirect_left_recursion(user_state, &involved_set, Rules::B, context, b);
    let closure_2 = _terminal(b'a');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _terminal(b'a');
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn b<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::A, Rules::B].to_vec();
    let closure_1 =
        _var_name_indirect_left_recursion(user_state, &involved_set, Rules::A, context, a);
    let closure_2 = _terminal(b'b');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _terminal(b'b');
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn grammar<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::A, Rules::B].to_vec();
    let closure_1 =
        _var_name_indirect_left_recursion(user_state, &involved_set, Rules::A, context, a);
    closure_1(parent, source, position)
}
