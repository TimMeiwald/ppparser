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
pub fn test_lr_num<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Num, context, num);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn test_indirect_three_level_a<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Test_indirect_three_level_A,
        Rules::Test_indirect_three_level_B,
        Rules::Test_indirect_three_level_C,
        Rules::Test_indirect_three_level_D,
        Rules::Test_indirect_three_level_E,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Test_indirect_three_level_B,
        context,
        test_indirect_three_level_b,
    );
    let closure_2 = _terminal(b'-');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(user_state, Rules::Test_LR_num, context, test_lr_num);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name(user_state, Rules::Test_LR_num, context, test_lr_num);
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn test_indirect_three_level_b<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Test_indirect_three_level_A,
        Rules::Test_indirect_three_level_B,
        Rules::Test_indirect_three_level_C,
        Rules::Test_indirect_three_level_D,
        Rules::Test_indirect_three_level_E,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Test_indirect_three_level_C,
        context,
        test_indirect_three_level_c,
    );
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn test_indirect_three_level_c<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Test_indirect_three_level_A,
        Rules::Test_indirect_three_level_B,
        Rules::Test_indirect_three_level_C,
        Rules::Test_indirect_three_level_D,
        Rules::Test_indirect_three_level_E,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Test_indirect_three_level_D,
        context,
        test_indirect_three_level_d,
    );
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn test_indirect_three_level_d<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Test_indirect_three_level_A,
        Rules::Test_indirect_three_level_B,
        Rules::Test_indirect_three_level_C,
        Rules::Test_indirect_three_level_D,
        Rules::Test_indirect_three_level_E,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Test_indirect_three_level_E,
        context,
        test_indirect_three_level_e,
    );
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn test_indirect_three_level_e<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Test_indirect_three_level_A,
        Rules::Test_indirect_three_level_B,
        Rules::Test_indirect_three_level_C,
        Rules::Test_indirect_three_level_D,
        Rules::Test_indirect_three_level_E,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Test_indirect_three_level_A,
        context,
        test_indirect_three_level_a,
    );
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn grammar<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Test_indirect_three_level_A,
        Rules::Test_indirect_three_level_B,
        Rules::Test_indirect_three_level_C,
        Rules::Test_indirect_three_level_D,
        Rules::Test_indirect_three_level_E,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Test_indirect_three_level_A,
        context,
        test_indirect_three_level_a,
    );
    closure_1(parent, source, position)
}
