use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    collections::HashSet,
};

use crate::{BasicContext, Context, Key, Source, publisher, user_state::UserState};

pub fn is_typedef<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
    func: &impl Fn(Key, &Source, u32) -> (bool, u32),
) -> impl Fn(Key, &Source, u32) -> (bool, u32) {
    // We check userstate for typedefs and if the result succeeds
    // and there is a typedef of that name we return true, end position
    // if not we return false and the start positon
    move |parent: Key, source: &Source, position: u32| {
        println!("Before typedef_name execution");
        let result = func(parent, source, position);
        println!("After typedef_name execution: {result:?}");
        result
    }
}


pub fn declared_new_typedef<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
    func: &impl Fn(Key, &Source, u32) -> (bool, u32),
) -> impl Fn(Key, &Source, u32) -> (bool, u32) {
    // Wraps declaration specifier, if the responses first child is storage class specifier that contains 'typedef'
    // Then it means we may be declaring a new typedef
    // If that is true then we successfully declared a typedef and we store it in UserState
    move |parent: Key, source: &Source, position: u32| {
        let publisher: <T as Context>::P = context.get_mut().get_publisher().;
        let node = publisher
        for child_key in node.get_children() {
        let child = publisher.get_node(*child_key);
    }
        let result = func(parent, source, position);
        result
    }
}