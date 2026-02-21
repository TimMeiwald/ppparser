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
        // println!("Before typedef_name");
        let result = func(parent, source, position);

        // If the typedef token was created then we accept any identifier as a new typedef name
        // and add it to userstate and return the result from identifier.
        println!("IN IS TYPEDEF");
        {
            let typedef_token_detected: bool;
            {
                typedef_token_detected = user_state.borrow().typedef_token_matched;
            }
            println!("TYPEDEF TOKEN DETECTED: {typedef_token_detected}");
            if typedef_token_detected {
                let mut usr_state = user_state.borrow_mut();
                if result.0 {
                    let typedef_name = source
                        .get_string(position, result.1)
                        .expect("If result is true it should have content");
                    usr_state.typedef_names.insert(typedef_name.to_string());
                }
                // Reset whether true or false. In false case then it terminates the parser anyway.
                usr_state.typedef_token_matched = false;
                return result;
            }
        }
        // If typedef was not set then if identifier is true we check if it's in the existing set
        // of typedef names.
        if result.0 {
            let typedef_name = source
                .get_string(position, result.1)
                .expect("If result is true it should have content");
            // println!("Typedef name: {typedef_name:?}");

            // If typedef_token_detected was set by declared_new_typedef which wraps the typedef string match
            // Then any valid identifier is a new typedef(valid hence only adding if result.0 is true)
            let contains_typedef_name: bool;
            {
                contains_typedef_name = user_state.borrow().typedef_names.contains(typedef_name);
            }
            // println!("CONTAINES TYPEDEF NAME: {contains_typedef_name:?}");
            if contains_typedef_name {
                // println!("CONTAINS TYPEDEF NAME");
                return result;
            } else {
                // println!("DOES NOT CONTAIN TYPEDEF NAME");
                return (false, position);
            }
        }
        // println!("After typedef_name: {result:?}");
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
        let result = func(parent, source, position);
        {
            if result.0 {
                // typedef string was matched.
                {
                    println!("SETTING DECLARED TYPEDEF TO TRUE");
                    user_state.borrow_mut().typedef_token_matched = true;
                }
            }
            return result;
        }
    }
}
