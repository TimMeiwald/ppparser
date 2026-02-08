#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn rr<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  rr ::= "1" <rr> / "1"
    let closure_1 = _terminal(b'1');
    let closure_2 = _var_name(user_state, Rules::Rr, context, rr);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _terminal(b'1');
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn lr<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  lr ::= <lr> "1" / "1"
    let involved_set: Vec<Rules> = [Rules::Lr].to_vec();
    let closure_1 =
        _var_name_indirect_left_recursion(user_state, &involved_set, Rules::Lr, context, lr);
    let closure_2 = _terminal(b'1');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _terminal(b'1');
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
    // We do not use grammar for this benchmark but we need one for the generation to work.
    let closure_1 = _var_name(user_state, Rules::Rr, context, rr);
    closure_1(parent, source, position)
}
