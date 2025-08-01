#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn expr<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::expr, Rules::addition].to_vec();
    let closure_1 =
        _var_name_indirect_left_recursion(&involved_set, Rules::addition, context, addition);
    let involved_set: Vec<Rules> = [Rules::subtraction, Rules::expr].to_vec();
    let closure_2 =
        _var_name_indirect_left_recursion(&involved_set, Rules::subtraction, context, subtraction);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::integer, context, integer);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn addition<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::subtraction, Rules::expr, Rules::addition].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(&involved_set, Rules::expr, context, expr);
    let closure_2 = _terminal(b'+');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::integer, context, integer);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn subtraction<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::subtraction, Rules::expr, Rules::addition].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(&involved_set, Rules::expr, context, expr);
    let closure_2 = _terminal(b'-');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::integer, context, integer);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn onenine<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(49, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn digit<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn integer<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        onenine(parent, context, source, position)
    };
    let closure_2 =
        move |parent: Key, source: &Source, position: u32| digit(parent, context, source, position);
    let closure_3 = _zero_or_more(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    closure_4(parent, source, position)
}
#[allow(dead_code)]
pub fn grammar<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::subtraction, Rules::expr, Rules::addition].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(&involved_set, Rules::expr, context, expr);
    closure_1(parent, source, position)
}
