use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    collections::HashSet,
};

use crate::{
    BasicContext, Context, Key, Source, publisher, publisher_trait::Publisher,
    user_state::UserState,
};

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
        println!("Before typedef_name");
        let result = func(parent, source, position);
        if result.0 {
            let typedef_name = source
                .get_string(position, result.1)
                .expect("If result is true it should have content");
            println!("Typedef name: {typedef_name:?}");
            let contains_typedef_name: bool;
            {
                contains_typedef_name = user_state.borrow().typedef_names.contains(typedef_name);
            }
            println!("CONTAINES TYPEDEF NAME: {contains_typedef_name:?}");
            if contains_typedef_name {
                println!("CONTAINS TYPEDEF NAME");
                return result;
            } else {
                println!("DOES NOT CONTAIN TYPEDEF NAME");
                return (false, position);
            }
        }
        println!("After typedef_name: {result:?}");
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
        println!("Before declared_new_typedef");
        let result = func(parent, source, position);
        {
            let ctx = context.borrow();
            let publisher = ctx.borrow_publisher();
            let node = publisher.get_node(parent);
            for child_key in node.get_children() {
                let child = publisher.get_node(*child_key);
                println!("Child: {:?}", child.rule);
            }
        }
        println!("After declared_new_typedef");
        result
    }
}
