#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn num<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn expr<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Should match 0-0-0-0-0-0-0-0 etc
    let involved_set: Vec<Rules> = [Rules::Expr].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(&involved_set, Rules::Expr, context, expr);
    let closure_2 = _terminal(b'-');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(user_state, Rules::Num, context, num);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name(user_state, Rules::Num, context, num);
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn grammar<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Expr].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(&involved_set, Rules::Expr, context, expr);
    closure_1(parent, source, position)
}
