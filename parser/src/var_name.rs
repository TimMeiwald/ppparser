use super::{Context, Rules, Source};
use crate::{cache::LR, Key};
use core::{cell::RefCell, panic};
use std::{collections::BTreeSet, thread::current};

fn memoized_behaviour<T: Context>(
    context: &RefCell<T>,
    rule: Rules,
    parent: Key,
    is_true: bool,
    start_position: u32,
    end_position: u32,
    memoized_key: Key,
) -> (bool, u32) {
    #[cfg(debug_assertions)]
    {
        println!(
            "Memoized: Rule: {:?}, Position: {:?}, Parent: {:?}, Result: {:?}",
            rule,
            start_position,
            parent,
            (is_true, end_position)
        )
    }
    context.borrow_mut().connect(parent, memoized_key);
    (is_true, end_position)
}

fn default_behaviour<T: Context>(
    source: &Source,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
    context: &RefCell<T>,
    rule: Rules,
    parent: Key,
    start_position: u32,
) -> (bool, u32) {
    let current_key = context.borrow_mut().reserve_publisher_entry(rule);
    let f = func(current_key, context, source, start_position);
    let mut c = context.borrow_mut();
    c.create_cache_entry(rule, f.0, start_position, f.1, current_key);
    c.update_publisher_entry(current_key, f.0, start_position, f.1);
    // Change to only connect on success to makes things a little faster
    c.connect(parent, current_key);
    #[cfg(debug_assertions)]
    {
        println!(
            "Default Behaviour: Rule: {:?}, Position: {:?}, Parent: {:?}, Result: {:?}",
            rule, start_position, parent, f
        )
    }
    f
}

pub fn _var_name<T: Context>(
    rule: Rules,
    context: &RefCell<T>,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
) -> impl Fn(Key, &Source, u32) -> (bool, u32) + '_ {
    move |parent: Key, source: &Source, position: u32| {
        _var_name_kernel(rule, context, parent, source, position, func)
    }
}

pub fn _var_name_kernel<T: Context>(
    rule: Rules,
    context: &RefCell<T>,
    parent: Key,
    source: &Source,
    position: u32,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    #[cfg(debug_assertions)]
    {
        println!(
            "Regular Kernel Start: Rule: {:?}, Position: {:?}, Parent: {:?}",
            rule, position, parent
        )
    }
    let memo = context.borrow().check(rule, position);
    return match memo {
        Some((is_true, end_position, memoized_key)) => memoized_behaviour(
            context,
            rule,
            parent,
            is_true,
            position,
            end_position,
            memoized_key,
        ),
        None => default_behaviour(source, func, context, rule, parent, position),
    };
}

pub fn _var_name_direct_left_recursion<T: Context>(
    rule: Rules,
    context: &RefCell<T>,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
) -> impl Fn(Key, &Source, u32) -> (bool, u32) + '_ {
    move |parent: Key, source: &Source, position: u32| {
        _var_name_kernel_direct_left_recursion(rule, context, parent, source, position, func)
    }
}

pub fn _var_name_kernel_direct_left_recursion<T: Context>(
    rule: Rules,
    context: &RefCell<T>,
    parent: Key,
    source: &Source,
    position: u32,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    let memo = context.borrow().check_lr(rule, position);
    return match memo {
        Some((is_true, end_position, memoized_key, lr)) => match lr {
            LR::Set => {
                context.borrow_mut().create_cache_entry_direct_lr(
                    rule,
                    is_true,
                    position,
                    end_position,
                    memoized_key,
                    LR::Unset,
                );

                return (is_true, end_position);
            }
            LR::Unset => {
                return (is_true, end_position);
            }
        },
        None => {
            // No Cached Value -> Run the function
            let mut current_key: Key;
            let mut last_key: Key = parent;

            let mut f: (bool, u32) = (false, position);
            let mut last_f: (bool, u32) = (false, position);
            loop {
                {
                    let mut c = context.borrow_mut();
                    current_key = c.reserve_publisher_entry(rule);
                    c.create_cache_entry_direct_lr(rule, f.0, position, f.1, current_key, LR::Set);
                }
                f = func(current_key, context, source, position);
                if !f.0 || (f.1 <= last_f.1) {
                    f = last_f;
                    break;
                }
                if parent != last_key {
                    context.borrow_mut().connect_front(current_key, last_key);
                }
                last_f = f;
                last_key = current_key;
                let mut c = context.borrow_mut();
                c.update_publisher_entry(last_key, f.0, position, f.1);
                // Change to only connect on success to makes things a little faster
            }
            context
                .borrow_mut()
                .update_publisher_entry(current_key, f.0, position, f.1);
            context.borrow_mut().connect(parent, last_key);

            f
        }
    };
}

// pub fn _var_name_indirect_left_recursion<'a, T: Context + 'static>(
//     involved_set: &'a Vec<Rules>,
//     rule: Rules,
//     context: &'a RefCell<T>,
//     func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
// ) -> impl Fn(Key, &Source, u32) -> (bool, u32) + 'a {
//     move |parent: Key, source: &Source, position: u32| {
//         _var_name_kernel_indirect_left_recursion(
//             involved_set,
//             rule,
//             context,
//             parent,
//             source,
//             position,
//             func,
//         )
//     }
// }

// pub fn _var_name_kernel_indirect_left_recursion<T: Context>(
//     involved_set: &Vec<Rules>,
//     rule: Rules,
//     context: &RefCell<T>,
//     parent: Key,
//     source: &Source,
//     position: u32,
//     func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
// ) -> (bool, u32) {
//     let head_rule = context.borrow().check_head(position);
//     if head_rule.is_none() {
//         #[cfg(debug_assertions)]
//         {
//             println!(
//                 "Rule: {:?}, Position: {:?}, Starting Left Recursion",
//                 rule, position
//             )
//         }
//         let mut involved: BTreeSet<Rules> = BTreeSet::new();
//         for r in involved_set {
//             involved.insert(*r);
//         }
//         context.borrow_mut().set_head(position, rule, involved);
//         let mut current_key: Key;
//         let mut last_key: Key = parent;

//         let mut f: (bool, u32) = (false, position);
//         let mut last_f: (bool, u32) = (false, position);

//         loop {
//             {
//                 let mut c = context.borrow_mut();
//                 current_key = c.reserve_publisher_entry(rule);
//                 c.create_cache_entry(rule, f.0, position, f.1, current_key);
//             }
//             println!("CURRENT KEY: {:?}", current_key);
//             f = func(current_key, context, source, position);
//             #[cfg(debug_assertions)]
//             {
//                 println!(
//                     "Rule: {:?}, Position: {:?}, Core Loop Last Result: {:?}, Last Key: {:?}",
//                     rule, position, last_f, last_key
//                 );
//                 println!(
//                     "Rule: {:?}, Position: {:?}, Core Loop Result: {:?}, Key: {:?}",
//                     rule, position, f, current_key
//                 )
//             }
//             context.borrow_mut().reinitialize_eval_set(position);

//             if !f.0 || (f.1 <= last_f.1) {
//                 break;
//             }

//             println!("f: {:?}, Last f: {:?}", f, last_f);
//             last_f = f;
//             last_key = current_key;
//             let mut c = context.borrow_mut();
//             c.update_publisher_entry(last_key, last_f.0, position, last_f.1);
//             // Change to only connect on success to makes things a little faster
//         }
//         context.borrow_mut().remove_head(position);
//         let memo = context.borrow().check(rule, position);
//         println!("Memo: {:?}", memo);
//         match memo{
//             None => panic!("Should not happen!"),
//             Some(memo) => {
//                     context.borrow_mut().connect(memo.2, current_key);

//                    context.borrow_mut().connect(parent, memo.2);
//                     #[cfg(debug_assertions)]
//                     {
//                         println!(
//                             "Rule: {:?}, Position: {:?}, Returning Indirect Left Recursion Result: {:?}, Key: {:?}",
//                             rule, position, last_f, last_key
//                         )
//                     }
//                     return (memo.0, memo.1);
//                     }
//     }
//     }

//     let should_run_function = context.borrow().rule_in_eval_set(position, rule);
//     if should_run_function {
//         {
//             println!(
//                 "Rule: {:?}, Position: {:?}, Function should run!",
//                 rule, position,
//             )
//         }
//         let current_key = context.borrow_mut().reserve_publisher_entry(rule);
//         context.borrow_mut().remove_from_eval_set(position, rule); // So it only runs once. Then recreate evalset in the original core loop above.

//         let f = func(current_key, context, source, position);

//         let mut c = context.borrow_mut();
//         c.create_cache_entry(rule, f.0, position, f.1, current_key);
//         c.update_publisher_entry(current_key, f.0, position, f.1);
//         // Change to only connect on success to makes things a little faster
//         c.connect(parent, current_key);
//         {
//             println!(
//                 "Rule: {:?}, Position: {:?}, Should Run: Returning Non Memoized Result: {:?}, Parent: {:?}",
//                 rule, position, f, parent
//             )
//         }
//         f
//     } else {
//         let memo = context.borrow().check(rule, position);
//         match memo {
//             None => {
//                 #[cfg(debug_assertions)]
//                 {
//                     println!(
//                         "Rule: {:?}, Position: {:?}, Returning Non Memoized Result: {:?}",
//                         rule,
//                         position,
//                         (false, 0)
//                     )
//                 }
//                 (false, 0)
//             }
//             Some((is_true, end_position, memoized_key)) => {
//                 context.borrow_mut().connect(parent, memoized_key);
//                 #[cfg(debug_assertions)]
//                 {
//                     println!(
//                         "Rule: {:?}, Position: {:?}, Returning Memoized Result: {:?}, Key: {:?}, Parent: {:?}",
//                         rule,
//                         position,
//                         (is_true, end_position),
//                         memoized_key,
//                         parent
//                     )
//                 }
//                 (is_true, end_position)
//             }
//         }
//     }
// }

pub fn _var_name_indirect_left_recursion2<'a, T: Context + 'static>(
    involved_set: &'a Vec<Rules>,
    rule: Rules,
    context: &'a RefCell<T>,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
) -> impl Fn(Key, &Source, u32) -> (bool, u32) + 'a {
    move |parent: Key, source: &Source, position: u32| {
        _var_name_kernel_indirect_left_recursion3(
            involved_set,
            rule,
            context,
            parent,
            source,
            position,
            func,
        )
    }
}

fn convert_vec_to_btree_set(involved_set: &Vec<Rules>) -> BTreeSet<Rules> {
    let mut involved: BTreeSet<Rules> = BTreeSet::new();
    for r in involved_set {
        involved.insert(*r);
    }
    involved
}

// pub fn _var_name_kernel_indirect_left_recursion2<T: Context>(
//     involved_set: &Vec<Rules>,
//     rule: Rules,
//     context: &RefCell<T>,
//     parent: Key,
//     source: &Source,
//     position: u32,
//     func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
// ) -> (bool, u32) {
//     // println!("In Indirect Left Recursion 2");
//     let head = context.borrow().check_head(position);
//     if head.is_none(){
//         println!("Entering Core Loop");
//         // println!("Active Left Recursion Rule: {:?}", rule);
//         // Sets head otherwise does nothing and just calls it again.
//         let involved_btree = convert_vec_to_btree_set(involved_set);
//         context.borrow_mut().set_head(position, rule, involved_btree);
//         let mut result: (bool, u32);
//         let mut last_result: (bool, u32) = (true, position);
//         let mut current_key: Key =  context.borrow_mut().reserve_publisher_entry(rule);

//         let mut last_key = parent;
//         let mut count = 0;
//         loop {
//             context.borrow_mut().reinitialize_eval_set(position);
//             // println!("Going into func");
//             println!("Before Func Count: {:?}, Current Key: {:?}", count, current_key);
//             // Probs need a 2nd loop where it checks for whether there's anything left in eval set.
//             result = func(current_key, context, source, position);
//             println!("After Func Count: {:?}", count);

//             // println!("Result: {:?}, Last Result: {:?}, {:?}, {:?}, {:?}", result, last_result, !result.0, (result.1 <= last_result.1), !result.0 || (result.1 <= last_result.1));
//             context.borrow_mut().update_publisher_entry(current_key, result.0, position, result.1);

//             if !result.0 || (result.1 <= last_result.1) {
//                 break;
//             }
//             context.borrow_mut().create_cache_entry(rule, result.0, position, result.1, current_key);
//             last_result = result;
//             last_key = current_key;
//             current_key = context.borrow_mut().reserve_publisher_entry(rule);
//             count += 1;
//             context.borrow_mut().connect(current_key, last_key);

//         }

//         context.borrow_mut().remove_head(position);
//         // Clear Nodes children here and then connect last node?
//         context.borrow_mut().clear_node_of_children(parent);
//         context.borrow_mut().connect(parent, last_key);
//         println!("Indirect Core Loop: Rule: {:?}: {:?}, Parent: {:?}, Current: {:?}", rule, last_result, parent, last_key);
//         return last_result;
//     }
//     else{
//         println!("Entering Alternate Path");
//         let head = head.expect("Should exist since we checked none above");
//         // println!("Head Rule is {:?}, Current Rule is {:?}, At Position: {:?}", head, rule, position);
//         // If head is some it will go here.
//         // println!("In default behaviour");
//         let is_in_eval_set = context.borrow().rule_in_eval_set(position, rule);
//         if is_in_eval_set{
//             println!("Entering Is In Eval Set: {:?}", rule);
//             // println!("{:?}, Is In Eval Set", rule);
//             context.borrow_mut().remove_from_eval_set(position, rule);
//             let current_key = context.borrow_mut().reserve_publisher_entry(rule);
//             let f = func(current_key, context, source, position);
//             let mut c = context.borrow_mut();
//             c.create_cache_entry(rule, f.0, position, f.1, current_key);
//             c.update_publisher_entry(current_key, f.0, position, f.1);
//             // Change to only connect on success to makes things a little faster
//             c.connect(parent, current_key);
//             println!("Indirect Is In Eval Set: Rule: {:?}: {:?}",rule, f);

//             f

//         }
//         else{
//             // println!("Not in Eval Set");

//             let memo = context.borrow().check(rule, position);
//             return match memo{
//                 None => {
//                     println!("Returning False, 0");
//                         (false, 0)
//                     }
//                 Some((is_true, end_position, memoized_key)) => {
//                     context.borrow_mut().connect(parent, memoized_key);
//                     println!("Indirect Memoized: Rule: {:?}: {:?}", rule, (is_true, end_position));

//                     (is_true, end_position)

//                 }
//             }
//         }

//     }
// }

pub fn _var_name_kernel_growth_function<T: Context>(
    involved_set: &Vec<Rules>,
    rule: Rules,
    context: &RefCell<T>,
    parent: Key,
    source: &Source,
    position: u32,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    println!("ENTERING GROWTH FUNCTION!");
    // If head is none, return what is stored in the memo table.
    let mut result: (bool, u32);
    let mut last_result = (false, position);
    let mut last_key: Key = context.borrow_mut().reserve_publisher_entry(rule);
    let first_last_key = last_key;
    let first_current_key: Key = context.borrow_mut().reserve_publisher_entry(rule);
    let mut current_key = first_current_key;
    {
        let involved_btree = convert_vec_to_btree_set(involved_set);

        // Creates head so on calling func again it goes to other branch
        context
            .borrow_mut()
            .set_head(position, rule, involved_btree);
        let mut count = 0;

        loop {
            println!("LOOP COUNT: {:?}", count);
            context.borrow_mut().reinitialize_eval_set(position);

            println!("Entering Func: {:?} {:?}", rule, position);
            println!("Parent: {:?} Child: {:?}", parent, current_key);

            let memo = context.borrow().check(rule, position);
            let memo = memo;
            match memo {
                Some(memo) => {
                    println!(
                        "JUSTBEFOREFUNCMEMO {:?}, Memo Entry: {:?} ",
                        last_result, memo
                    );
                }
                None => {}
            }

            result = func(current_key, context, source, position);

            println!(
                "RESULT: {:?}, Current Key: {:?}, Last Key: {:?}",
                result, current_key, last_key
            );

            println!(
                "Leaving Func: {:?} {:?} with response {:?}",
                rule, position, result
            );
            if !result.0 || (result.1 <= last_result.1) {
                context.borrow_mut().create_cache_entry(
                    rule,
                    last_result.0,
                    position,
                    last_result.1,
                    last_key,
                );
                context.borrow_mut().update_publisher_entry(
                    last_key,
                    last_result.0,
                    position,
                    last_result.1,
                );
                context.borrow_mut().connect(parent, last_key);

                // Updates the first initial sidestepping parse.
                // context
                //     .borrow_mut()
                //     .disconnect(first_current_key, first_last_key);
                // context
                //     .borrow_mut()
                //     .connect_front(first_current_key, current_key);
                // context
                //     .borrow_mut()
                //     .update_publisher_entry(current_key, true, position, result.1);

                break;
            }
            context.borrow_mut().create_cache_entry(
                rule,
                result.0,
                position,
                result.1,
                current_key,
            );
            context
                .borrow_mut()
                .update_publisher_entry(current_key, result.0, position, result.1);
            // Because of the left recursion we need to insert the new connection at the front to maintain associativity.
            // Initially last_key is Parent so we lose one layer of the tree.
            //context.borrow_mut().connect_front(current_key, last_key);

            last_result = result;
            last_key = current_key;
            count += 1;
            current_key = context.borrow_mut().reserve_publisher_entry(rule);
        }
    }
    let memo = context.borrow().check(rule, position);
    let memo = memo.expect("Should there always be a cached entry at this point?");
    println!(
        "In Growth Function, returning {:?}, Memo Entry: {:?} ",
        last_result, memo
    );
    // We need to reset the head, in that if there was a head before we need to push it back onto the stack.
    //context.borrow_mut().remove_head(position);
    context.borrow_mut().reset_head(position);

    return last_result;
}

pub fn should_go_into_growth_function<T: Context>(
    rule: Rules,
    context: &RefCell<T>,
    position: u32,
) -> bool {
    let ctx = context.borrow();
    let head = ctx.check_head(position);
    //println!("Head: {:?}", head);
    return match head {
        Some(head) => {
            if ctx.rule_in_involved_set(position, rule) {
                false
            } else {
                true
            }
        }
        None => true,
    };
}

pub fn _var_name_kernel_indirect_left_recursion3<T: Context>(
    involved_set: &Vec<Rules>,
    rule: Rules,
    context: &RefCell<T>,
    parent: Key,
    source: &Source,
    position: u32,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    let should = should_go_into_growth_function(rule, context, position);
    println!(
        "Rule: {:?}, Position: {:?} should go into growth: {:?} ",
        rule, position, should
    );
    if should {
        let result = _var_name_kernel_growth_function(
            involved_set,
            rule,
            context,
            parent,
            source,
            position,
            func,
        );
        return result;
    } else {
        // Runs if head is Some
        // Do not evaluate any rule that is not involved in this left recursion(i.e is not in the involved set.)
        let is_in_eval_set = context.borrow().rule_in_eval_set(position, rule);
        println!("{:?} is in Eval Set: {:?}", rule, is_in_eval_set);
        if is_in_eval_set {
            let memo = context.borrow().check(rule, position);
            match memo {
                None => {
                    let current_key = context.borrow_mut().reserve_publisher_entry(rule); // I need it for the keys even though they don't necessarily make sense currently/ Might need a NULL key.
                    context.borrow_mut().remove_from_eval_set(position, rule);

                    println!("RUNNING FUNCTION: {:?}", rule);
                    // The first time we evaluate we just pass throught he parent function.
                    let f = func(current_key, context, source, position);
                    context.borrow_mut().connect(parent, current_key);
                    context
                        .borrow_mut()
                        .update_publisher_entry(current_key, f.0, position, f.1);
                    println!("FIRST TIME RESULT: {:?}", f);
                    println!("Not in Growth Result(1): {:?}", (rule, f));
                    f
                }
                Some(memo) => {
                    println!("Not in Growth Result(2): {:?}", (rule, memo.0, memo.1));
                    context.borrow_mut().connect(parent, memo.2);
                    (memo.0, memo.1)
                }
            }
        } else {
            println!("{:?} NOT IN EVAL SET RETURNING {:?}", rule, position);
            let memo = context.borrow().check(rule, position);
            match memo {
                Some(memo) => {
                    println!("Returning(1) {:?}", (memo.0, memo.1));
                    (memo.0, memo.1)
                }
                None => {
                    println!("Returning(2) {:?}", (false, 0));
                    println!("RULE: {:?} in Returning(2)", rule);
                    (false, 0)
                }
            }
        }
    }
}
