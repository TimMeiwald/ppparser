#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::hooked_calls::logger;
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn grammar<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Matches 'Hello World!' exactly but has external hooks.
    let closure_1 = _string_terminal_opt_ascii(b"Hello World!");
    let closure_2 = _subexpression(&closure_1);
    let closure_3 = logger(&closure_2);
    closure_3(parent, source, position)
}
