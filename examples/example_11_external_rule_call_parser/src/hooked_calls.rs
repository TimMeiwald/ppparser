use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    collections::HashSet,
};

use crate::{BasicContext, Context, Key, Source, user_state::UserState};

pub fn unique_line_cache<T: Context>(
    user_state: &RefCell<UserState>,
    _parent: Key,
    _context: &RefCell<T>,
    _source: &Source,
    _position: u32,
    func: &impl Fn(Key, &Source, u32) -> (bool, u32),
) -> impl Fn(Key, &Source, u32) -> (bool, u32) {
    move |parent: Key, source: &Source, position: u32| {
        println!("Before function execution");
        let result = func(parent, source, position);
        if result.0 {
            // If it is indeed a string we will store it in the hashset if it isn't already there and return
            // If it is there we will respond with (false, position).
            let unique_line = source.get_string(position, result.1).expect(
                "If it crashes here there is a logical error as we should never go out of bounds.",
            );
            let mut user_state = user_state.borrow_mut();
            let did_not_exist_before = user_state.0.insert(unique_line.to_string());
            if did_not_exist_before {
                // Did not exist already so we just continue
            } else {
                // Return false and the start not end position.
                return (false, position);
            }
        }

        println!("Result is {:?} {:?}", result.0, result.1);
        println!("After function execution");
        result
    }
}
