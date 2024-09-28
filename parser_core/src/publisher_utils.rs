#![allow(clippy::type_complexity)]

use std::any::Any;
use std::borrow::BorrowMut;
use std::mem::ManuallyDrop;
use std::process::exit;
use std::u32;

// While complex still under development and the core of the entire program is here so complexity is
// Acceptable
use crate::source::Source;
use crate::Context;
use cache::ASTOrLR;
use cache::Cache;
use cache::MemoEntry;
use cache::AST;
use cache::LR;
use publisher::Publisher;
use rules::{Key, Rules};

pub fn publisher_setup_node<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    rule: Rules,
) -> (Option<Key>, Key) {
    let mut publisher = context.stack.borrow_mut();
    let parent_key: Option<Key> = publisher.last_node();
    let current_key = publisher.add_node(rule, 0, 0, false);
    publisher.set_last_node(Some(current_key));
    return (parent_key, current_key);
}

pub fn publisher_get_last_node<T: Cache, S: Publisher>(context: &Context<T, S>) -> Option<Key> {
    let publisher = context.stack.borrow_mut();
    let parent_key: Option<Key> = publisher.last_node();
    return parent_key;
}

pub fn publisher_update_node<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    start_position: u32,
    end_position: u32,
    is_true: bool,
    parent_key: Option<Key>,
    current_key: Key,
) {
    let mut publisher = context.stack.borrow_mut();
    match parent_key {
        None => {}
        Some(pkey) => {
            // REMOVE THIS IF STATEMENT IF YOU ALSO WANT TO SEE ERRORS,
            // E.G FOR LINTERS SO THEY CAN FIND THE LONGEST MATCH AND SUGGEST A FIX.
            if is_true {
                publisher.connect(pkey, current_key);
            }
        }
    }
    publisher.set_node_start_position(current_key, start_position);
    publisher.set_node_end_position(current_key, end_position);
    publisher.set_node_result(current_key, is_true);
    publisher.set_last_node(parent_key);
}
pub fn publisher_update_node_only_connect_if_not_connected<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    start_position: u32,
    end_position: u32,
    is_true: bool,
    parent_key: Option<Key>,
    current_key: Key,
) {
    let mut publisher = context.stack.borrow_mut();
    match parent_key {
        None => {}
        Some(pkey) => {
            // REMOVE THIS IF STATEMENT IF YOU ALSO WANT TO SEE ERRORS,
            // E.G FOR LINTERS SO THEY CAN FIND THE LONGEST MATCH AND SUGGEST A FIX.
            if is_true {
                publisher.connect_if_not_connected(pkey, current_key);
            }
        }
    }
    publisher.set_node_start_position(current_key, start_position);
    publisher.set_node_end_position(current_key, end_position);
    publisher.set_node_result(current_key, is_true);
    publisher.set_last_node(parent_key);
}
