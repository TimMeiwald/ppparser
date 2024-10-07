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
    return match memo {
        Some((is_true, end_position, memoized_key)) => {
            context.borrow_mut().connect(parent, memoized_key);
            (is_true, end_position)
        }
        None => {
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
    let head_rule = context.borrow().check_head(position);
    if head_rule.is_none() {
        let mut involved: BTreeSet<Rules> = BTreeSet::new();
        for r in involved_set {
            involved.insert(*r);
        }
        context.borrow_mut().set_head(position, rule, involved);
        let mut current_key: Key;
        let mut last_key: Key = parent;

        let mut f: (bool, u32) = (false, position);
        let mut last_f: (bool, u32) = (false, position);

        loop {
            {
                let mut c = context.borrow_mut();
                current_key = c.reserve_publisher_entry(rule);
                c.create_cache_entry(rule, f.0, position, f.1, current_key);
            }
            f = func(current_key, context, source, position);
            context.borrow_mut().reinitialize_eval_set(position);

            if !f.0 || (f.1 <= last_f.1) {
                f = last_f;
                break;
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

        return f;
    }

    let should_run_function = context.borrow().rule_in_eval_set(position, rule);
    if should_run_function {
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
