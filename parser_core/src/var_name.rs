#![allow(clippy::type_complexity)]

use std::any::Any;
use std::mem::ManuallyDrop;
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
    move |source: &Source, position: u32| publisher_kernel(rule, context, source, position, func)
}

// Memo(R, P) -> MemoEntry in paper
// In our case Cache(R, P, ...) -> Option<(Option<bool>, u32, Key)> where Key is essentially the AST/ which in the paper can have
// a special value of FAIL, I used the option in bool instead which is wrong.
// Ideally I'd use Cache(R, P) -> Option<(bool, u32, Option<Key>)>

fn publisher_setup<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    rule: Rules,
) -> (Option<Key>, Key) {
    let mut publisher = context.stack.borrow_mut();
    let parent_key: Option<Key> = publisher.last_node();
    let current_key = publisher.add_node(rule, 0, 0, false);
    publisher.set_last_node(Some(current_key));
    return (parent_key, current_key);
}

fn publisher_update<T: Cache, S: Publisher>(
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
            publisher.connect(pkey, current_key);
        }
    }
    publisher.set_node_start_position(current_key, start_position);
    publisher.set_node_end_position(current_key, end_position);
    publisher.set_node_result(current_key, is_true);
    publisher.set_last_node(parent_key);
}

fn publisher_kernel<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
) -> (bool, u32, AST) {
    println!("{:?} Entering Publisher Kernel", rule);
    let (parent_key, current_key) = publisher_setup(context, rule);
    let (ans, pos, ast) =
        _var_name_kernel_direct_lr(rule, context, source, position, func, current_key);
    publisher_update(context, position, pos, ans, parent_key, current_key);
    println!("{:?} Exiting Publisher Kernel", rule);
    return (ans, pos, ast);
}

fn _var_name_kernel_basic<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
    current_key: Key,
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
    println!("{:?} Entering No Cached Value Block", rule);
    let ans = func(context, source, position);
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

fn _var_name_kernel_deny_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
    current_key: Key,
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
                    panic!("{:?} Left Recursion was detected", rule);
                }
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
    println!("{:?} Entering No Cached Value Block", rule);
    {
        // Necessary modificaiton to deny LR.
        let mut cache = context.cache.borrow_mut();
        cache.push(rule, false, position, 0, cache::ASTOrLR::AST(AST::FAIL))
    }
    let ans = func(context, source, position);
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
    current_key: Key,
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
                match memo_entry.ast_or_lr {
                    ASTOrLR::LR(lr) => {
                        println!(
                            "Position: {:?}, Identified AstOrLR: {:?}",
                            position, memo_entry.ast_or_lr
                        );

                        {
                            cache.set_lr_detected(rule, position, LR::new(true));
                            println!("{:?} Exiting Cached Value Block", rule);
                            println!("{:?} Exiting ASTORLR LR Var_Name Kernel", rule);
                        }
                        return (false, 0, AST::FAIL);
                    }
                    ASTOrLR::AST(ast) => {
                        println!("{:?} Exiting Cached Value Block", rule);
                        println!("{:?} Exiting Var_Name Kernel", rule);
                        return (
                            memo_entry.is_true,
                            memo_entry.position,
                            memo_entry.ast_or_lr.into(),
                        );
                    }
                }
            }
            None => {}
        }
    }
    println!("{:?} Entering No Cached Value Block", rule);
    {
        // Necessary modificaiton for direct LR.
        let mut cache = context.cache.borrow_mut();
        cache.push(rule, false, position, 0, ASTOrLR::LR(LR::new(false)))
    }
    let ans = func(context, source, position);

    let lr_detected: bool;
    {
        let mut cache = context.cache.borrow_mut();
        let m = cache.check_lr(rule, position);
        println!("Set memoEntry too: {:?}", m.unwrap());
    }
    {
        let mut cache = context.cache.borrow_mut();
        lr_detected = cache.get_lr_detected(rule, position);
    }
    println!(
        "Rule: {:?}, Position: {:?}, LR DETECTED: {:?}, AST: {:?},  {:?}, {:?}   ",
        rule, position, lr_detected, ans.2, ans.0, ans.1,
    );
    if lr_detected && ans.2 != AST::FAIL {
        println!("Entering No Cached LR Detected");
        let ans = grow_lr_direct_lr(rule, context, source, position, func, current_key);
        let mut cache = context.cache.borrow_mut();
        cache.push(
            rule,
            ans.0,
            position,
            ans.1,
            ASTOrLR::AST(AST::SUCCESS(current_key)),
        );
        println!("{:?} Exiting No Cached Value Block", rule);
        println!("{:?} Exiting Var_Name Kernel", rule);
        return ans;
    } else {
        println!("Entering No Cached No LR Detected");
        let mut cache = context.cache.borrow_mut();
        cache.push(
            rule,
            ans.0,
            position,
            ans.1,
            ASTOrLR::AST(AST::SUCCESS(current_key)),
        );
        println!("{:?} Exiting No Cached Value Block", rule);
        println!("{:?} Exiting Var_Name Kernel", rule);
        return ans;
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
    let mut temp_pos = position;
    let mut temp_ans: AST = AST::FAIL;
    let mut temp_bool: bool = false;
    loop {
        println!("GrowLR Before Func");
        let ans = func(context, source, position);
        println!("GrowLR After Func");

        println!("GrowLR {:?} {:?} {:?}", ans.0, ans.1, ans.2);
        if ans.2 == AST::FAIL || (ans.1 <= temp_pos) {
            return (temp_bool, temp_pos, temp_ans);
        }
        temp_pos = ans.1;
        temp_ans = ans.2;
        temp_bool = ans.0;
        {
            let mut cache: std::cell::RefMut<T> = context.cache.borrow_mut();
            cache.push(
                rule,
                ans.0,
                position,
                ans.1,
                ASTOrLR::AST(AST::SUCCESS(current_key)),
            )
        }
    }
}
