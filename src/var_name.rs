use super::{Context, Rules, Source};
use crate::{cache::LR, Key};
use core::cell::RefCell;
use std::collections::BTreeSet;

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
    let memo = context.borrow().check(rule, position);
    //println!("Rule: {:?}, In Regular Recursion!", rule);
    //println!("Not left recursion: Parent Key: {:?}", parent);
    return match memo {
        Some((is_true, end_position, memoized_key)) => {
            //println!("Regular Recursion Cached");
            // Cached value -> Return the cached result
            //println!("Is Cached Value");
            // Change to only connect on success to makes things a little faster
            context.borrow_mut().connect(parent, memoized_key);
            (is_true, end_position)
        }
        None => {
            //println!("Regular Recursion Not Cached");

            //println!("Is not Cached Value");
            // No Cached Value -> Run the function
            let current_key = context.borrow_mut().reserve_publisher_entry(rule);
            let f = func(current_key, context, source, position);
            let mut c = context.borrow_mut();
            c.create_cache_entry(rule, f.0, position, f.1, current_key);
            c.update_publisher_entry(current_key, f.0, position, f.1);
            // Change to only connect on success to makes things a little faster
            c.connect(parent, current_key);
            f
        }
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
    // This function is used instead of _var_name if we know through static analysis at generation time
    // that this function is directly or indirectly left recursive.

    // Prior to running the function, we set the cached value to FAIL in the Not cached branch
    // Then if the cached branch triggers it means we already have set it to FAIL at least once
    // Then we update the result and run it again?

    let memo = context.borrow().check_lr(rule, position);
    println!(
        "Rule: {:?}, In direct Left Recursion! Parent Key: {:?}",
        rule, parent
    );
    return match memo {
        Some((is_true, end_position, memoized_key, lr)) => {
            // Cached value -> Return the cached result
            println!("Is Cached Value");
            // Change to only connect on success to makes things a little faster
            match lr {
                LR::Set => {
                    println!("In SET: {:?}", (is_true, end_position));
                    context.borrow_mut().create_cache_entry_direct_lr(
                        rule,
                        is_true,
                        position,
                        end_position,
                        memoized_key,
                        LR::Unset,
                    );

                    return (is_true, end_position);
                } // This forces an evaluation of other branches
                LR::Unset => {
                    println!("In UNSET: {:?}", (is_true, end_position));
                    return (is_true, end_position);
                }
            }
        }
        None => {
            println!("Is not Cached Value");
            // No Cached Value -> Run the function
            let mut current_key: Key;
            println!("Parent Key is: {:?}", parent);
            let mut last_key: Key = parent;

            let mut f: (bool, u32) = (false, position);
            let mut last_f: (bool, u32) = (false, position);
            loop {
                {
                    let mut c = context.borrow_mut();
                    current_key = c.reserve_publisher_entry(rule);
                    println!("Current Key: {:?}", current_key);
                    c.create_cache_entry_direct_lr(rule, f.0, position, f.1, current_key, LR::Set);
                }
                f = func(current_key, context, source, position);
                println!("Function Result: {:?}", f);
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

            println!("Last Key: {:?}", last_key);

            f
        }
    };
}

pub fn _var_name_indirect_left_recursion<'a, T: Context + 'static>(
    involved_set: &'a Vec<Rules>,
    rule: Rules,
    context: &'a RefCell<T>,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
) -> impl Fn(Key, &Source, u32) -> (bool, u32) + 'a {
    move |parent: Key, source: &Source, position: u32| {
        _var_name_kernel_indirect_left_recursion(
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

pub fn _var_name_kernel_indirect_left_recursion<T: Context>(
    involved_set: &Vec<Rules>,
    rule: Rules,
    context: &RefCell<T>,
    parent: Key,
    source: &Source,
    position: u32,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    // This function is used instead of _var_name if we know through static analysis at generation time
    // that this function is directly or indirectly left recursive.

    // The goal is to make a copy of involved_set and grow it every cycle until growth halts.
    println!("Rule in Indirect: {:?}", rule);
    let head_rule = context.borrow().check_head(position);
    println!("Head Rule: {:?}", head_rule);
    if head_rule.is_none() {
        // Create head rule if none yet exists. N.B do not create if it already exists at position
        // Since only one rule can be active at each position at any given time.
        let mut involved: BTreeSet<Rules> = BTreeSet::new();
        for r in involved_set {
            involved.insert(*r);
        }
        // Set head also sets eval set to copy of involved set.
        context.borrow_mut().set_head(position, rule, involved);
        println!("Is not Cached Value");
        // No Cached Value -> Run the function
        let mut current_key: Key;
        println!("Parent Key is: {:?}", parent);
        let mut last_key: Key = parent;

        let mut f: (bool, u32) = (false, position);
        let mut last_f: (bool, u32) = (false, position);

        loop {
            {
                let mut c = context.borrow_mut();
                current_key = c.reserve_publisher_entry(rule);
                println!("Current Key: {:?}", current_key);

                // Set Involved Set

                c.create_cache_entry(rule, f.0, position, f.1, current_key);
            }
            f = func(current_key, context, source, position);
            println!("{:?}", context.borrow().print_cache());
            context.borrow_mut().reinitialize_eval_set(position);
            println!("{:?}", context.borrow().print_cache());

            println!("Function Result: {:?}", f);
            if !f.0 || (f.1 <= last_f.1) {
                f = last_f;
                break;
            }
            // if parent != last_key {
            //     context.borrow_mut().connect_front(current_key, last_key);
            // }
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

        println!("Last Key: {:?}", last_key);

        return f;
    }

    println!("Prior to rule_in_eval_set");
    let should_run_function = context.borrow().rule_in_eval_set(position, rule);
    println!("Should run function: {:?}", should_run_function);
    if should_run_function {
        // If head is some it means we're currently actively left recursing.
        println!("Head Rule: {:?}", head_rule);
        println!(
            "Rule: {:?}, In indirect Left Recursion! Parent Key: {:?}",
            rule, parent
        );
        println!("Indirect LR Not Cached");

        //println!("Is not Cached Value");
        // No Cached Value -> Run the function
        let current_key = context.borrow_mut().reserve_publisher_entry(rule);
        context.borrow_mut().remove_from_eval_set(position, rule); // So it only runs once. Then recreate evalset in the original core loop above.

        let f = func(current_key, context, source, position);
        let mut c = context.borrow_mut();
        c.create_cache_entry(rule, f.0, position, f.1, current_key);
        c.update_publisher_entry(current_key, f.0, position, f.1);
        // Change to only connect on success to makes things a little faster
        c.connect(parent, current_key);
        f
    } else {
        let memo = context.borrow().check(rule, position);
        match memo {
            None => (false, 0),
            Some((is_true, end_position, memoized_key)) => {
                context.borrow_mut().connect(parent, memoized_key);
                (is_true, end_position)
            }
        }
    }
}
