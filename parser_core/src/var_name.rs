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

fn setup_lr_var_name_kernel<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
) -> bool {
    let recursion_flag: bool;
    {
        let mut cache = context.cache.borrow_mut();
        recursion_flag = cache.get_recursion_setup_flag();
        if recursion_flag {
            let abort = !cache.insert_into_involved_set(rule);
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

fn _var_name_kernel_direct_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
) -> (bool, u32, AST) {
    println!("{:?} Entering Var_Name Kernel", rule);
    // Below code executes and runs when growLr sets up for indirect left recursion.
    let recursion_setup_flag: bool =
        setup_lr_var_name_kernel(rule, context, source, position, func);
    if recursion_setup_flag {
        return (false, 0, AST::FAIL);
    }
    let recursion_execution_flag: bool;
    {
        let cache = context.cache.borrow();
        recursion_execution_flag = cache.get_recursion_execution_flag();
    }
    if recursion_execution_flag {
        let should_func_run: bool;
        {
            let cache = context.cache.borrow();
            should_func_run = cache.is_in_eval_set(rule);
            cache.print_eval_set();
        }
        if should_func_run {
            // Cacheless call
            let result = _var_name_kernel_body_cacheless(rule, context, source, position, func);
            {
                let mut cache = context.cache.borrow_mut();
                println!("Removing rule: {:?}", rule);
                cache.remove_from_eval_set(rule);
            }
            return result;
        } else {
            let result = _var_name_kernel_body(rule, context, source, position, func);
            return result;
        }
    } else {
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
        cache.set_active_rule(rule);
        cache.set_recursion_setup_flag();
        // cache.insert_into_involved_set(rule);
    }
    func(context, source, position);
    {
        let mut cache = context.cache.borrow_mut();
        cache.print_involved_set();
        cache.reset_recursion_setup_flag();
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
    println!("In GrowLR");
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
    loop {
        // Every Loop we need to replace the AST reference in our initial node value
        // With the new one which then uses the old one as a child.
        let (_pkey, ckey) = publisher_setup_node(context, rule);

        {
            // Every Growth Iteration we copy everything in involved set into eval set to prepare for the next growth iteration.
            let mut cache = context.cache.borrow_mut();
            cache.copy_involved_set_into_eval_set();
        }
        // Keep calling func until all are gone in eval set
        let mut ans: (bool, u32, AST);
        // ans = func(context, source, position);
        {
            let mut cache = context.cache.borrow_mut();
            println!("\x1b[33mRecursion Execution Flag Set");
            cache.set_recursion_execution_flag();
        }
        loop {
            println!("{:?}: GrowLR Before Func", rule);
            ans = func(context, source, position);
            println!("{:?}: GrowLR After Func", rule);

            {
                let mut cache = context.cache.borrow();
                cache.print_eval_set();
                if cache.eval_set_is_empty() {
                    println!("Eval Set is emtpy");
                    break;
                }
            }
        }

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

                cache.reset_recursion_execution_flag();
            }
            return (temp_bool, temp_pos, temp_ans);
        }
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
