use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    collections::HashSet,
};

use crate::{BasicContext, Context, Key, Source, user_state::UserState};

pub fn typedef_name_handler<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
    func: &impl Fn(Key, &Source, u32) -> (bool, u32),
) -> impl Fn(Key, &Source, u32) -> (bool, u32) {
    move |parent: Key, source: &Source, position: u32| {
        println!("Before typedef_name execution");
        let result = func(parent, source, position);
        result
    }
}
