#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
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
pub fn character<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(0, 255);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn letter<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(97, 122);
    let closure_2 = _ordered_choice_match_range(65, 90);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn integer<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 =
        move |parent: Key, source: &Source, position: u32| digit(parent, context, source, position);
    let closure_2 = _one_or_more(&closure_1);
    closure_2(parent, source, position)
}
#[allow(dead_code)]
pub fn float<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Integer, context, integer);
    let closure_2 = _terminal(b'.');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Integer, context, integer);
    let closure_5 = _terminal(b'"');
    let closure_6 = _terminal(b'"');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _ordered_choice(&closure_4, &closure_7);
    let closure_9 = _subexpression(&closure_8);
    let closure_10 = _sequence(&closure_3, &closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn char<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\'');
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        character(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b'\'');
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn string<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'"');
    let closure_2 = _terminal(b'"');
    let closure_3 = _not_predicate(&closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        character(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _zero_or_more(&closure_6);
    let closure_8 = _sequence(&closure_1, &closure_7);
    let closure_9 = _terminal(b'"');
    let closure_10 = _sequence(&closure_8, &closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn comment<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"//");
    let closure_2 = _terminal(b'\n');
    let closure_3 = _not_predicate(&closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        character(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _zero_or_more(&closure_6);
    let closure_8 = _sequence(&closure_1, &closure_7);
    let closure_9 = _terminal(b'\n');
    let closure_10 = _sequence(&closure_8, &closure_9);
    closure_10(parent, source, position)
}
