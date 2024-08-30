#![allow(clippy::type_complexity)]

use std::any::Any;
use std::borrow::BorrowMut;
use std::mem::ManuallyDrop;
use std::process::exit;
use std::u32;

// While complex still under development and the core of the entire program is here so complexity is
// Acceptable
use crate::publisher_utils::*;
use crate::source::Source;
use crate::Context;
use cache::ASTOrLR;
use cache::Cache;
use cache::MemoEntry;
use cache::AST;
use cache::LR;
use publisher::Publisher;
use rules::{Key, Rules};

// Memo(R, P) -> MemoEntry in paper
// In our case Cache(R, P, ...) -> Option<(Option<bool>, u32, Key)> where Key is essentially the AST/ which in the paper can have
// a special value of FAIL, I used the option in bool instead which is wrong.
// Ideally I'd use Cache(R, P) -> Option<(bool, u32, Option<Key>)>

fn setup_lr_var_name_kernel<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
    current_active_lr_position: u32,
) -> bool {
    let recursion_flag: bool;
    {
        let mut cache = context.cache.borrow_mut();
        recursion_flag = cache.get_recursion_setup_flag(current_active_lr_position);
        if recursion_flag {
            let abort = !cache.insert_into_involved_set(rule, current_active_lr_position);
            if abort {
                return true;
            }
        }
    }
    if recursion_flag {
        // This gets set in grow_LR.
        // We use this solely to setup LR by grabbing each rule that is triggered by a func call and
        // pushing it into a stack.
        println!("Rule on stack: {:?}", rule);

        // We add the rule + func to stack until we're repeating ourselves.
        func(context, source, position);

        return true;
    } else {
        false
    }
}

pub fn _var_name_kernel_indirect_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
) -> (bool, u32, AST) {
    println!("{:?} Entering Var_Name Kernel", rule);
    // Below code executes and runs when growLr sets up for indirect left recursion.
    let current_active_lr_position: Option<u32>;
    {
        let cache = context.cache.borrow();
        current_active_lr_position = cache.get_current_active_lr_position();
    }
    let active_lr_position: u32;
    match current_active_lr_position {
        None => {
            // If no LR is currently in use we skip every other check.
            println!("VAR NAME BODY 3");
            return _var_name_kernel_body(rule, context, source, position, func);
        }
        Some(lr_position) => {
            active_lr_position = lr_position;
        }
    }

    let recursion_setup_flag: bool =
        setup_lr_var_name_kernel(rule, context, source, position, func, active_lr_position);
    if recursion_setup_flag {
        return (false, 0, AST::FAIL);
    }
    let recursion_execution_flag: bool;
    {
        let cache = context.cache.borrow();
        recursion_execution_flag = cache.get_recursion_execution_flag(active_lr_position);
    }
    println!(
        "Position: {:?}, Recursion Execution Flag: {:?}",
        active_lr_position, recursion_execution_flag
    );
    if recursion_execution_flag {
        let should_func_run: bool;
        let active_rule: Rules;
        {
            let cache = context.cache.borrow();
            should_func_run = cache.is_in_eval_set(rule, active_lr_position);
            cache.print_eval_set(active_lr_position);
            active_rule = cache.get_active_rule(active_lr_position);
            println!(
                "{:?} ACTIVE RULE: {:?}, RULE: {:?}, SHOULD FUNC RUN: {:?}",
                position, active_rule, rule, should_func_run
            );
        }
        if should_func_run {
            // Cacheless call
            println!("VAR NAME CACHELESS");
            {
                let mut cache = context.cache.borrow_mut();
                println!("Removing rule: {:?}", rule);
                cache.remove_from_eval_set(rule, active_lr_position);
            }
            let result = _var_name_kernel_body_cacheless(rule, context, source, position, func);
            return result;
        } else {
            println!("VAR NAME BODY 1");
            let result = _var_name_kernel_body(rule, context, source, position, func);
            return result;
        }
    } else {
        println!("VAR NAME BODY 2");
        return _var_name_kernel_body(rule, context, source, position, func);
    }
}

fn _var_name_kernel_body<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
) -> (bool, u32, AST) {
    // Body of the _var_name_kernel_as_in_direct_kernel

    {
        let mut cache: std::cell::RefMut<T> = context.cache.borrow_mut();
        // Removes value from hashmap so we need to reinsert afterwards.
        let memo_entry = cache.check_lr(rule, position);
        match memo_entry {
            Some(memo_entry) => {
                // Cached Values do exist.
                // println!("{:?} Entering Cached Value Block", rule);
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
                        // println!("{:?} Exiting Var_Name Kernel", rule);
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
                        // println!("{:?} Exiting Var_Name Kernel", rule);
                        return (memo_entry.is_true, memo_entry.position, current_key_ast);
                    }
                }
            }
            None => {}
        }
    }
    return _var_name_kernel_body_cacheless(rule, context, source, position, func);
}

fn _var_name_kernel_body_cacheless<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
) -> (bool, u32, AST) {
    let (parent_key, current_key) = publisher_setup_node(context, rule);
    // println!("{:?} Entering No Cached Value Block", rule);
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
    if lr_detected && ans.0 != false {
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
        // println!("{:?} Exiting Var_Name Kernel in GrowLR", rule);
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
        // println!("{:?} Exiting Var_Name Kernel", rule);
        return (ans.0, ans.1, AST::SUCCESS(current_key));
    }
}

fn setup_lr_grow_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
    current_key: Key,
) {
    println!("\x1b[31mRecursion Flag Set");
    {
        let mut cache = context.cache.borrow_mut();
        cache.set_active_rule(rule, position);
        cache.set_recursion_setup_flag(position);
        cache.set_current_active_lr_position(Some(position));
    }
    func(context, source, position);
    {
        let mut cache = context.cache.borrow_mut();
        cache.print_involved_set(position);
        cache.reset_recursion_setup_flag(position);
    }
    println!("Found all involved rules");
    println!("Recursion Flag Reset\x1b[0m");
}

fn grow_lr_direct_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
    current_key: Key,
) -> (bool, u32, AST) {
    println!("{:?} In GrowLR", position);
    println!("{:?}", rule);
    let parent_root_key = publisher_get_last_node(context);
    let root_key = current_key;
    let mut current_key = current_key;
    let mut temp_pos = position;
    let mut temp_ans: AST = AST::FAIL;
    let mut temp_bool: bool = false;
    let mut temp_ckey: Key = Key(0);
    println!("BEFORE SETUP LR IN GROW LR: {:?}", rule);

    setup_lr_grow_lr(rule, context, source, position, func, current_key);

    {
        let mut publisher = context.stack.borrow_mut();
        publisher.disconnect(parent_root_key.expect("Should exist"), current_key);
        let last_node = publisher.last_node();
        publisher.set_last_node(Some(last_node.unwrap()));
    }

    // Set Execution Flag
    println!("GROW LR STARTING WITH RULE: {:?}", rule);
    let mut counter = 0;
    let previous_active_lr_position: Option<u32>;
    {
        let mut cache = context.cache.borrow_mut();
        println!("\x1b[33mRecursion Execution Flag Set");
        cache.set_recursion_execution_flag(position);
        previous_active_lr_position = cache.get_current_active_lr_position();
    }
    loop {
        // Every Loop we need to replace the AST reference in our initial node value
        // With the new one which then uses the old one as a child.
        let (_pkey, ckey) = publisher_setup_node(context, rule);

        {
            // Every Growth Iteration we copy everything in involved set into eval set to prepare for the next growth iteration.
            let mut cache = context.cache.borrow_mut();
            cache.copy_involved_set_into_eval_set(position);
        }
        // Keep calling func until all are gone in eval set
        let mut ans: (bool, u32, AST);
        // ans = func(context, source, position);

        loop {
            println!("{:?}: GrowLR Before Func", rule);
            ans = func(context, source, position);
            println!("{:?}: GrowLR After Func", rule);

            {
                let mut cache = context.cache.borrow();
                if cache.head_exists(position) {
                    println!("Eval Set is emtpy");
                    break;
                }
                cache.print_eval_set(position);
            }
        }
        println!("Loop {:?}", counter);
        counter += 1;
        let last_node = publisher_get_last_node(context);
        println!("GrowLR: Last Node {:?}", last_node);

        println!("GrowLR After Func");

        println!("GrowLR {:?} {:?} {:?}", ans.0, ans.1, ans.2);
        if ans.0 == false || (ans.1 <= temp_pos) {
            publisher_update_node(
                context,
                position,
                temp_pos,
                temp_bool,
                parent_root_key,
                temp_ckey,
            );
            // Reset Execution Flag
            {
                let mut cache = context.cache.borrow_mut();
                println!("\x1b[0mRecursion Execution Flag Reset");
                cache.set_current_active_lr_position(previous_active_lr_position);
                cache.reset_recursion_execution_flag(position);
            }
            println!(
                "GROW DIRECT LR RESULT: {:?}",
                (temp_bool, temp_pos, temp_ans)
            );
            return (temp_bool, temp_pos, temp_ans);
        }
        println!("PUBLISHER");
        // Don't connect until complete using the last result
        publisher_update_node(context, position, ans.1, ans.0, None, ckey);
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
