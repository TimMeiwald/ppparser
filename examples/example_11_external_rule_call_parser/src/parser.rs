#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::hooked_calls::unique_line_cache;
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn alphabet_lower<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(97, 122);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn unique_line_matcher<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Trivial context sensitive grammar, any lower case line will be matched but only once.

    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        alphabet_lower(user_state, parent, context, source, position)
    };
    let closure_2 = _terminal(b' ');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _one_or_more(&closure_4);
    let closure_6 = _terminal(b'\n');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = unique_line_cache(user_state, parent, context, source, position, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn grammar<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Unique_line_matcher,
        context,
        unique_line_matcher,
    );
    let closure_2 = _subexpression(&closure_1);
    let closure_3 = _one_or_more(&closure_2);
    closure_3(parent, source, position)
}
