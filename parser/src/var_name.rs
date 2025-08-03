use super::{Context, Rules, Source};
use crate::{cache::LR, Key};
use core::{cell::RefCell, panic};
use std::{collections::BTreeSet, thread::current};

fn memoized_behaviour<T: Context>(
    context: &RefCell<T>,
    _rule: Rules,
    parent: Key,
    is_true: bool,
    _start_position: u32,
    end_position: u32,
    memoized_key: Key,
) -> (bool, u32) {
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
    // TODO: Change to only connect on success to makes things a little faster
    // Unsure how it impacts correctness on LR so needs testing first.
    c.connect(parent, current_key);
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
    let memo = context.borrow().check(rule, position);
    match memo {
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
    }
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
    match memo {
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

                (is_true, end_position)
            }
            LR::Unset => (is_true, end_position),
        },
        None => {
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
    }
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

fn convert_vec_to_btree_set(involved_set: &Vec<Rules>) -> BTreeSet<Rules> {
    let mut involved: BTreeSet<Rules> = BTreeSet::new();
    for r in involved_set {
        involved.insert(*r);
    }
    involved
}

#[allow(clippy::too_many_arguments)]
pub fn _var_name_kernel_growth_function<T: Context>(
    involved_set: &Vec<Rules>,
    rule: Rules,
    context: &RefCell<T>,
    parent: Key,
    source: &Source,
    position: u32,
    func: fn(Key, &RefCell<T>, &Source, u32) -> (bool, u32),
    last_lr_position: Option<(Rules, u32)>,
) -> (bool, u32) {
    // If head is none, return what is stored in the memo table.
    let mut result: (bool, u32);
    let mut last_result = (false, position);
    let mut last_key: Key = context.borrow_mut().reserve_publisher_entry(rule);
    context
        .borrow_mut()
        .update_publisher_entry(last_key, false, 0, 0);
    let mut current_key = context.borrow_mut().reserve_publisher_entry(rule);
    let previous_active_lr_position = last_lr_position;

    {
        let involved_btree = convert_vec_to_btree_set(involved_set);

        // Creates head so on calling func again it goes to other branch
        context
            .borrow_mut()
            .set_head(position, rule, involved_btree);
        loop {
            context.borrow_mut().reinitialize_eval_set(rule, position);
            result = func(current_key, context, source, position);
            let memo_result = context.borrow_mut().check(rule, position);
            match memo_result {
                None => {}
                Some(memo_result) => {
                    current_key = memo_result.2;
                    result = (memo_result.0, memo_result.1);
                }
            }
            context
                .borrow_mut()
                .update_publisher_entry(current_key, result.0, position, result.1);
            if !result.0 || (result.1 <= last_result.1) {
                context.borrow_mut().connect(parent, last_key);
                context
                    .borrow_mut()
                    .set_current_active_lr_position(previous_active_lr_position);
                context.borrow_mut().create_cache_entry(
                    rule,
                    last_result.0,
                    position,
                    last_result.1,
                    last_key,
                );
                break;
            }
            last_result = result;
            last_key = current_key;
            current_key = context.borrow_mut().reserve_publisher_entry(rule);
        }
    }
    last_result
}

pub fn should_go_into_growth_function<T: Context>(
    rule: Rules,
    context: &RefCell<T>,
    position: u32,
) -> (bool, Option<(Rules, u32)>) {
    // Keeps triggering everytime which it should not do.
    let mut ctx = context.borrow_mut();
    let active_lr_position = ctx.get_current_active_lr_position();
    match active_lr_position {
        None => {
            let last_lr_position = ctx.get_current_active_lr_position();
            ctx.set_current_active_lr_position(Some((rule, position)));
            (true, last_lr_position)
        }
        Some(lr_position) => {
            let head = ctx.check_head(rule, position);
            if head.is_some() {
                // We don't want to grow more than once per rule, position pair.
                (false, None)
            } else if ctx.rule_in_involved_set(lr_position, rule) {
                (false, None)
            } else {
                let last_lr_position = ctx.get_current_active_lr_position();
                ctx.set_current_active_lr_position(Some((rule, position)));
                (true, last_lr_position)
            }
        }
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
    let should = should_go_into_growth_function(rule, context, position);
    if should.0 {
        _var_name_kernel_growth_function(
            involved_set,
            rule,
            context,
            parent,
            source,
            position,
            func,
            should.1,
        )
    } else {
        // Runs if head is Some
        // Do not evaluate any rule that is not involved in this left recursion(i.e is not in the eval set.)
        let active_lr_position = context
            .borrow()
            .get_current_active_lr_position()
            .expect("Active LR Should exist when in LR code.");
        let is_in_eval_set = context.borrow().rule_in_eval_set(active_lr_position, rule);
        if is_in_eval_set {
            let current_key = context.borrow_mut().reserve_publisher_entry(rule);
            context
                .borrow_mut()
                .remove_from_eval_set(active_lr_position, rule);
            let result = func(current_key, context, source, position);
            context
                .borrow_mut()
                .update_publisher_entry(current_key, result.0, position, result.1);
            context.borrow_mut().create_cache_entry(
                rule,
                result.0,
                position,
                result.1,
                current_key,
            );
            context.borrow_mut().connect(parent, current_key);
            result
        } else {
            let memo = context.borrow().check(rule, position);

            match memo {
                Some(memo) => {
                    context
                        .borrow_mut()
                        .connect_if_not_connected(parent, memo.2);
                    (memo.0, memo.1)
                }
                None => (false, position),
            }
        }
    }
}
