#![allow(clippy::type_complexity)]

use std::mem::ManuallyDrop;
use std::u32;

// While complex still under development and the core of the entire program is here so complexity is
// Acceptable
use crate::source::Source;
use crate::Context;
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
    let current_key = publisher.add_node(rule, 0, u32::MAX, false);
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
        _var_name_kernel_basic(rule, context, source, position, func, current_key);
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

    let m: Option<MemoEntry>;
    {
        let cache: std::cell::RefMut<T> = context.cache.borrow_mut();
        m = cache.check_lr(rule, position);
    }

    match m {
        None => {
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
        Some(m) => {
            println!("{:?} Entering Cached Value Block", rule);
            println!("{:?} Exiting Cached Value Block", rule);
            println!("{:?} Exiting Var_Name Kernel", rule);
            return (m.is_true, m.position, m.ast);
        }
    }
}

// fn _var_name_kernel_deny_lr<T: Cache, S: Publisher>(
//     rule: Rules,
//     context: &Context<T, S>,
//     source: &Source,
//     position: u32,
//     func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
//     current_key: Key,
// ) -> (bool, u32, AST) {
//     println!("{:?} Entering Var_Name Kernel", rule);

//     let m: Option<MemoEntry>;
//     {
//         let cache: std::cell::RefMut<T> = context.cache.borrow_mut();
//         m = cache.check_lr(rule, position);
//     }

//     match m {
//         None => {
//             println!("{:?} Entering No Cached Value Block", rule);
//             {
//                 // Necessary modificaiton to deny LR.
//                 let mut cache = context.cache.borrow_mut();
//                 cache.push(rule, false, position, u32::MAX, AST::FAIL)
//             }
//             let ans = func(context, source, position);
//             let mut cache = context.cache.borrow_mut();
//             cache.push(rule, ans.0, position, ans.1, AST::SUCCESS(current_key));
//             println!("{:?} Exiting No Cached Value Block", rule);
//             println!("{:?} Exiting Var_Name Kernel", rule);
//             return ans;
//         }
//         Some(m) => {
//             println!("{:?} Entering Cached Value Block", rule);
//             // Solely for better user error
//             if m.ast == AST::FAIL {
//                 panic!("{:?} Left Recursion was detected", rule);
//             }
//             println!("{:?} Exiting Cached Value Block", rule);
//             println!("{:?} Exiting Var_Name Kernel", rule);
//             return (m.is_true, m.position, m.ast);
//         }
//     }
// }

// fn grow_lr_direct_lr<T: Cache, S: Publisher>(
//     rule: Rules,
//     context: &Context<T, S>,
//     source: &Source,
//     position: u32,
//     func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
//     current_key: Key,
// ) -> (bool, u32, AST) {
//     println!("In GrowLR");
//     loop {
//         let ans = func(context, source, position);

//         if ans.1 == u32::MAX || (position >= ans.1) {
//             return ans;
//         }
//         {
//             let mut cache: std::cell::RefMut<T> = context.cache.borrow_mut();
//             cache.push(rule, ans.0, position, ans.1, AST::SUCCESS(current_key))
//         }
//     }
// }

// fn _var_name_kernel_direct_lr<T: Cache, S: Publisher>(
//     rule: Rules,
//     context: &Context<T, S>,
//     source: &Source,
//     position: u32,
//     func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
//     current_key: Key,
// ) -> (bool, u32, AST) {
//     println!("{:?} Entering Var_Name Kernel", rule);

//     let m: Option<MemoEntry>;
//     {
//         let cache: std::cell::RefMut<T> = context.cache.borrow_mut();
//         m = cache.check_lr(rule, position);
//     }

//     match m {
//         None => {
//             println!("{:?} Entering No Cached Value Block", rule);
//             {
//                 // Necessary modificaiton for direct LR.
//                 let mut cache = context.cache.borrow_mut();
//                 cache.push(rule, false, position, u32::MAX, AST::LR(false))
//             }
//             let ans = func(context, source, position);
//             let lr_detected: bool;
//             let is_fail: bool;
//             {
//                 let cache = context.cache.borrow();
//                 lr_detected = cache.get_lr_detected(rule, position);
//                 is_fail = cache.get_is_fail(rule, position);
//             }
//             if lr_detected && !is_fail {
//                 {
//                     let mut cache = context.cache.borrow_mut();
//                     cache.set_is_fail(rule, position, false);
//                 }
//                 println!("Entering No Cached LR Detected");
//                 let ans = grow_lr_direct_lr(rule, context, source, position, func, current_key);
//                 let mut cache = context.cache.borrow_mut();
//                 cache.push(rule, ans.0, position, ans.1, AST::SUCCESS(current_key));
//                 println!("{:?} Exiting No Cached Value Block", rule);
//                 println!("{:?} Exiting Var_Name Kernel", rule);
//                 return ans;
//             } else {
//                 println!("Entering No Cached No LR Detected");
//                 let mut cache = context.cache.borrow_mut();
//                 cache.push(rule, ans.0, position, ans.1, AST::SUCCESS(current_key));
//                 println!("{:?} Exiting No Cached Value Block", rule);
//                 println!("{:?} Exiting Var_Name Kernel", rule);
//                 return ans;
//             }
//         }
//         Some(m) => {
//             println!("{:?} Entering Cached Value Block", rule);
//             // Solely for better user error
//             if m.ast == LR{detected: false} {
//                 {
//                     let mut cache = context.cache.borrow_mut();
//                     cache.set_lr_detected(rule, position, true);
//                     println!("{:?} Exiting Cached Value Block", rule);
//                     println!("{:?} Exiting Var_Name Kernel", rule);
//                     cache.set_is_fail(rule, position, true);
//                     return (false, u32::MAX, AST::FAIL); // Using u32::MAX as FAIL since Rust won't just willy nilly change things like JS.
//                                                          // Ideally need to replace all code to use either Option<(bool, u32, AST)> or something.
//                 }
//             } else {
//                 println!("{:?} Exiting Cached Value Block", rule);
//                 println!("{:?} Exiting Var_Name Kernel", rule);
//                 return (m.is_true, m.position);
//             }
//         }
//     }
// }
