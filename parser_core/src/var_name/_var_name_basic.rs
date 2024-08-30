#![allow(clippy::type_complexity)]

use crate::publisher_utils::*;
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

pub fn _var_name_kernel_basic<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
) -> (bool, u32, AST) {
    println!("{:?} Entering Var_Name Kernel", rule);

    {
        let mut cache: std::cell::RefMut<T> = context.cache.borrow_mut();
        // Removes value from hashmap so we need to reinsert afterwards.
        let memo_entry = cache.check_lr(rule, position);
        match memo_entry {
            Some(memo_entry) => {
                // START Publisher: If cached we connect the parent to the existing key.
                let current_key: AST = memo_entry.ast_or_lr.into();
                let current_key: Key = current_key.into();
                publisher_update_node(
                    context,
                    position,
                    memo_entry.position,
                    memo_entry.is_true,
                    publisher_get_last_node(context),
                    current_key,
                );
                // END PUBLISHER

                // Cached Values do exist.
                println!("{:?} Entering Cached Value Block", rule);
                println!("{:?} Exiting Cached Value Block", rule);
                println!("{:?} Exiting Var_Name Kernel", rule);
                return (
                    memo_entry.is_true,
                    memo_entry.position,
                    memo_entry.ast_or_lr.into(),
                );
            }
            None => {}
        }
    }
    // Publisher: If not cached we need to create a Node prior to calling the function, then connect parent to the returned key of func.
    let (parent_key, current_key) = publisher_setup_node(context, rule);
    // If not cached value we just run the func.
    println!("{:?} Entering No Cached Value Block", rule);
    let ans = func(context, source, position);
    // Publisher, we then update the node which sets it's values and connects the parent to the current key.
    publisher_update_node(context, position, ans.1, ans.0, parent_key, current_key);
    let mut cache = context.cache.borrow_mut();
    cache.push(
        rule,
        ans.0,
        position,
        ans.1,
        cache::ASTOrLR::AST(AST::SUCCESS(current_key)),
    );
    println!("{:?} Exiting No Cached Value Block", rule);
    println!("{:?} Exiting Var_Name Kernel", rule);
    return (ans.0, ans.1, AST::SUCCESS(current_key));
}
