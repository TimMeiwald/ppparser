#![allow(clippy::type_complexity)]

use std::any::Any;
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

pub fn _var_name<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
) -> impl Fn(&Source, u32) -> (bool, u32, AST) + '_ {
    move |source: &Source, position: u32| {
        _var_name_kernel_direct_lr(rule, context, source, position, func)
    }
}

// Memo(R, P) -> MemoEntry in paper
// In our case Cache(R, P, ...) -> Option<(Option<bool>, u32, Key)> where Key is essentially the AST/ which in the paper can have
// a special value of FAIL, I used the option in bool instead which is wrong.
// Ideally I'd use Cache(R, P) -> Option<(bool, u32, Option<Key>)>

fn publisher_setup_node<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    rule: Rules,
) -> (Option<Key>, Key) {
    let mut publisher = context.stack.borrow_mut();
    let parent_key: Option<Key> = publisher.last_node();
    let current_key = publisher.add_node(rule, 0, 0, false);
    publisher.set_last_node(Some(current_key));
    return (parent_key, current_key);
}

fn publisher_get_last_node<T: Cache, S: Publisher>(context: &Context<T, S>) -> Option<Key> {
    let publisher = context.stack.borrow_mut();
    let parent_key: Option<Key> = publisher.last_node();
    return parent_key;
}

fn publisher_update_node<T: Cache, S: Publisher>(
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
fn publisher_update_node_no_set_last_node<T: Cache, S: Publisher>(
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
}

fn _var_name_kernel_basic<T: Cache, S: Publisher>(
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

fn _var_name_kernel_deny_lr<T: Cache, S: Publisher>(
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
                // Cached Values do exist.
                println!("{:?} Entering Cached Value Block", rule);
                println!("{:?} Exiting Cached Value Block", rule);
                println!("{:?} Exiting Var_Name Kernel", rule);
                if memo_entry.ast_or_lr == ASTOrLR::AST(AST::FAIL) {
                    println!("{:?} Left Recursion was detected! Aborting", rule);
                    exit(-1);
                }
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
                return (
                    memo_entry.is_true,
                    memo_entry.position,
                    memo_entry.ast_or_lr.into(),
                );
            }
            None => {}
        }
    }
    // If not cached value we just run the func.

    let (parent_key, current_key) = publisher_setup_node(context, rule);
    println!("{:?} Entering No Cached Value Block", rule);
    {
        // Necessary modificaiton to deny LR.
        let mut cache = context.cache.borrow_mut();
        cache.push(rule, false, position, 0, cache::ASTOrLR::AST(AST::FAIL))
    }
    let ans = func(context, source, position);
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
    return ans;
}

fn _var_name_kernel_direct_lr<T: Cache, S: Publisher>(
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
                // Cached Values do exist.
                println!("{:?} Entering Cached Value Block", rule);
                // println!("{:?} Exiting Cached Value Block", rule);
                // println!("{:?} Exiting Var_Name Kernel", rule);
                match memo_entry.ast_or_lr {
                    ASTOrLR::LR(lr) => {
                        // println!(
                        //     "Position: {:?}, Identified AstOrLR: {:?}",
                        //     position, memo_entry.ast_or_lr
                        // );
                        // Publisher we intentionally ignore this here.
                        {
                            cache.set_lr_detected(rule, position, LR::new(true));
                            // println!("{:?} Exiting Cached Value Block", rule);
                            // println!("{:?} Exiting ASTORLR LR Var_Name Kernel", rule);
                        }
                        println!("{:?} Exiting Var_Name Kernel", rule);
                        return (false, 0, AST::FAIL);
                    }
                    ASTOrLR::AST(ast) => {
                        // println!("{:?} Exiting Cached Value Block", rule);
                        // println!("{:?} Exiting Var_Name Kernel", rule);
                        // START Publisher: If cached we connect the parent to the existing key.
                        let current_key_ast: AST = memo_entry.ast_or_lr.into();
                        let current_key: Key = current_key_ast.into();
                        publisher_update_node(
                            context,
                            position,
                            memo_entry.position,
                            memo_entry.is_true,
                            publisher_get_last_node(context),
                            current_key,
                        );
                        // END PUBLISHER
                        println!("{:?} Exiting Var_Name Kernel", rule);
                        return (memo_entry.is_true, memo_entry.position, current_key_ast);
                    }
                }
            }
            None => {}
        }
    }
    let (parent_key, current_key) = publisher_setup_node(context, rule);
    println!("{:?} Entering No Cached Value Block", rule);
    {
        // Necessary modificaiton for direct LR.
        let mut cache = context.cache.borrow_mut();
        cache.push(rule, false, position, 0, ASTOrLR::LR(LR::new(false)))
    }
    let ans = func(context, source, position);
    publisher_update_node(context, position, ans.1, ans.0, parent_key, current_key);

    let lr_detected: bool;
    {
        let mut cache = context.cache.borrow_mut();
        let m = cache.check_lr(rule, position);
        // println!("Set memoEntry too: {:?}", m.unwrap());
    }
    {
        let mut cache = context.cache.borrow_mut();
        lr_detected = cache.get_lr_detected(rule, position);
    }
    // println!(
    //     "Rule: {:?}, Position: {:?}, LR DETECTED: {:?}, AST: {:?},  {:?}, {:?}   ",
    //     rule, position, lr_detected, ans.2, ans.0, ans.1,
    // );
    if lr_detected && ans.2 != AST::FAIL {
        // println!("Entering No Cached LR Detected");
        let ans = grow_lr_direct_lr(rule, context, source, position, func, current_key);
        let mut cache = context.cache.borrow_mut();
        cache.push(
            rule,
            ans.0,
            position,
            ans.1,
            ASTOrLR::AST(AST::SUCCESS(current_key)),
        );
        // println!("{:?} Exiting No Cached Value Block", rule);
        println!("{:?} Exiting Var_Name Kernel in GrowLR", rule);
        return (ans.0, ans.1, AST::SUCCESS(current_key));
    } else {
        // println!("Entering No Cached No LR Detected");
        let mut cache = context.cache.borrow_mut();
        cache.push(
            rule,
            ans.0,
            position,
            ans.1,
            ASTOrLR::AST(AST::SUCCESS(current_key)),
        );
        // println!("{:?} Exiting No Cached Value Block", rule);
        println!("{:?} Exiting Var_Name Kernel", rule);
        return (ans.0, ans.1, AST::SUCCESS(current_key));
    }
}

fn grow_lr_direct_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
    current_key: Key,
) -> (bool, u32, AST) {
    println!("In GrowLR");
    println!("{:?}", rule);
    let parent_root_key = publisher_get_last_node(context);
    let root_key = current_key;
    let mut current_key = current_key;
    let mut temp_pos = position;
    let mut temp_ans: AST = AST::FAIL;
    let mut temp_bool: bool = false;
    let mut temp_ckey: Key = Key(0);

    {
        let mut publisher = context.stack.borrow_mut();
        publisher.disconnect(parent_root_key.expect("Should exist"), current_key);
        let last_node = publisher.last_node();
        publisher.set_last_node(Some(last_node.unwrap()));
    }

    loop {
        // Every Loop we need to replace the AST reference in our initial cached value
        // With the new one which then uses the old one as a child.
        println!("GrowLR Before Func");
        let (pkey, ckey) = publisher_setup_node(context, rule);
        let ans = func(context, source, position);
        let last_node = publisher_get_last_node(context);
        println!("GrowLR: Last Node {:?}", last_node);

        println!("GrowLR After Func");

        println!("GrowLR {:?} {:?} {:?}", ans.0, ans.1, ans.2);
        if ans.2 == AST::FAIL || (ans.1 <= temp_pos) {
            publisher_update_node(
                context,
                position,
                temp_pos,
                temp_bool,
                parent_root_key,
                temp_ckey,
            );
            return (temp_bool, temp_pos, temp_ans);
        }
        // Don't connect until complete using the last result
        publisher_update_node(context, position, ans.1, ans.0, None, ckey);
        // publisher_update_node(
        //     context,
        //     position,
        //     ans.1,
        //     ans.0,
        //     Some(parent_root_key.unwrap()),
        //     ckey,
        // );
        temp_pos = ans.1;
        temp_ans = ans.2;
        temp_bool = ans.0;
        temp_ckey = ckey;
        {
            let mut cache: std::cell::RefMut<T> = context.cache.borrow_mut();
            cache.push(
                rule,
                ans.0,
                position,
                ans.1,
                ASTOrLR::AST(AST::SUCCESS(ckey)),
            )
        }
    }
}
