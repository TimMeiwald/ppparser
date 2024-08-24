#![allow(clippy::type_complexity)]

// While complex still under development and the core of the entire program is here so complexity is
// Acceptable
use crate::source::Source;
use crate::Context;
use cache::{Cache, Head};
use publisher::Publisher;
use rules::{Key, Rules};

// Rename to _var_name temporarily for testing.
pub fn _var_name_no_lr<T: Cache, S: Publisher>(
    // Temporary rename from _var_name for testing.
    rule: Rules,
    context: &Context<T, S>,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> impl Fn(&Source, u32) -> (bool, u32) + '_ {
    move |source: &Source, position: u32| {
        _var_name_kernel_no_lr(rule, context, source, position, func)
    }
}

pub fn _var_name_deny_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> impl Fn(&Source, u32) -> (bool, u32) + '_ {
    move |source: &Source, position: u32| {
        _var_name_kernel_deny_lr(rule, context, source, position, func)
    }
}

pub fn _var_name<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> impl Fn(&Source, u32) -> (bool, u32) + '_ {
    move |source: &Source, position: u32| {
        _var_name_kernel_direct_lr(rule, context, source, position, func)
    }
}

// Temp rename of _var_name_indirect_lr to _var_name for test
pub fn _var_nam_indirect_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> impl Fn(&Source, u32) -> (bool, u32) + '_ {
    move |source: &Source, position: u32| {
        _var_name_kernel_indirect_lr(rule, context, source, position, func)
    }
}

pub fn _var_name_kernel_no_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    // NO LR
    let cached_val: Option<(bool, u32, Key)>;
    let temp_key: Option<Key>;
    let curr_key: Key;
    {
        let cache = context.cache.borrow();
        cached_val = cache.check(rule, position);
    }
    {
        let mut tree = context.stack.borrow_mut();
        temp_key = tree.last_node();
        curr_key = tree.add_node(rule, position, 0, temp_key, false);
        tree.set_last_node(Some(curr_key));
    }
    // If cached val exists. We don't use cache because we need to scope borrow_mut correctly
    if cached_val.is_some() {
        let cached_val = cached_val.unwrap();
        // Cached Val at Index Key
        let key = cached_val.2;
        // Make cached subtree a child of parent and current node parent of subtree
        let mut tree = context.stack.borrow_mut();
        match temp_key {
            None => {}
            Some(tkey) => {
                tree.connect(tkey, key);
            }
        }
        let result = (cached_val.0, cached_val.1);
        tree.set_node_start_position(curr_key, position);
        tree.set_node_end_position(curr_key, result.1);
        tree.set_node_result(curr_key, result.0);
        tree.set_last_node(temp_key);
        return result;
        // Return Result
    } else {
        let result = func(context, source, position);
        {
            // Cache Val - Scoping may let the compiler optimize better. May being the operative word.
            let mut cache = context.cache.borrow_mut();
            cache.push(rule, result.0, position, result.1, curr_key);
        }
        let mut tree = context.stack.borrow_mut();
        match temp_key {
            None => {}
            Some(tkey) => {
                tree.connect(tkey, curr_key);
            }
        }
        // Return Result
        tree.set_node_start_position(curr_key, position);
        tree.set_node_end_position(curr_key, result.1);
        tree.set_node_result(curr_key, result.0);
        tree.set_last_node(temp_key);
        return result;
    }
}

pub fn _var_name_kernel_deny_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    println!("Rule: {:?}, Position: {:?}", rule, position);
    let cached_val: Option<(Option<bool>, u32, Key)>;
    let temp_key: Option<Key>;
    let curr_key: Key;
    {
        let cache = context.cache.borrow();
        cached_val = cache.check_LR(rule, position);
    }
    {
        let mut tree = context.stack.borrow_mut();
        temp_key = tree.last_node();
        curr_key = tree.add_node(rule, position, 0, temp_key, false);
        tree.set_last_node(Some(curr_key));
    }
    // If cached val exists. We don't use cache because we need to scope borrow_mut correctly
    if cached_val.is_some() {
        let mut cached_val = cached_val.unwrap();
        if cached_val.0.is_none() {
            cached_val.0 = Some(false);
        }

        // Cached Val at Index Key
        let key = cached_val.2;
        // Make cached subtree a child of parent and current node parent of subtree
        let mut tree = context.stack.borrow_mut();
        match temp_key {
            None => {}
            Some(tkey) => {
                tree.connect(tkey, key);
            }
        }
        let result = (
            cached_val
                .0
                .expect("By this point we should have checked for None"),
            cached_val.1,
        );
        tree.set_node_start_position(curr_key, position);
        tree.set_node_end_position(curr_key, result.1);
        tree.set_node_result(curr_key, result.0);
        tree.set_last_node(temp_key);
        return result;
        // Return Result
    } else {
        {
            println!("No Cached Value");
            // Solely for Deny LR
            let mut cache = context.cache.borrow_mut();
            println!("Pushing to Cache");
            cache.push_deny_LR(rule, None, position, position, curr_key);
        }
        println!("Prior to func call");
        let result = func(context, source, position);
        {
            // Cache Val - Scoping may let the compiler optimize better. May being the operative word.
            let mut cache = context.cache.borrow_mut();
            cache.push(rule, result.0, position, result.1, curr_key);
        }
        let mut tree = context.stack.borrow_mut();
        match temp_key {
            None => {}
            Some(tkey) => {
                tree.connect(tkey, curr_key);
            }
        }
        // Return Result
        tree.set_node_start_position(curr_key, position);
        tree.set_node_end_position(curr_key, result.1);
        tree.set_node_result(curr_key, result.0);
        tree.set_last_node(temp_key);
        return result;
    }
}

pub fn grow_direct_lr<T: Cache, S: Publisher>(
    reference: Key,
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    println!("In Grow LR with Rule: {:?}", rule);
    let cached_val: Option<(Option<bool>, u32, Key)>;
    {
        let cache = context.cache.borrow();
        cached_val = cache.check_LR(rule, position);
    }
    println!("Cached Val: {:?}", cached_val);
    let mut result: (bool, u32);
    let mut last_result: (bool, u32) = (false, position);
    // Need to break one before the last result so that the Last Sequence can execute
    println!("Reference: {:?}", reference);
    let mut temp_key: Option<Key>;
    let mut curr_key: Key;
    {
        let mut tree = context.stack.borrow_mut();
        tree.set_last_node(Some(reference));
    }

    loop {
        {
            let mut tree = context.stack.borrow_mut();
            temp_key = tree.last_node();
            curr_key = tree.add_node(rule, position, 0, temp_key, false);
            tree.set_last_node(Some(curr_key));
        }
        result = func(context, source, position);
        if result.0 == false || result.1 <= last_result.1 {
            break;
        }
        {
            let mut cache = context.cache.borrow_mut();
            cache.push(rule, result.0, position, result.1, reference);
        }
        last_result = result;
        println!("Result: {:?}", result);
        {
            let mut tree = context.stack.borrow_mut();
            match temp_key {
                None => {}
                Some(tkey) => {
                    tree.connect(tkey, curr_key);
                }
            }
            // Return Result
            tree.set_node_start_position(curr_key, position);
            tree.set_node_end_position(curr_key, result.1);
            tree.set_node_result(curr_key, result.0);
            tree.set_last_node(Some(curr_key));
        }
    }

    {
        let mut cache = context.cache.borrow_mut();
        cache.set_lr_detected(None);
    }

    println!("Returning: {:?}", last_result);
    return (last_result.0, last_result.1);
}

// Memo(R, P) -> MemoEntry in paper
// In our case Cache(R, P, ...) -> Option<(Option<bool>, u32, Key)> where Key is essentially the AST/ which in the paper can have
// a special value of FAIL, I used the option in bool instead which is wrong.
// Ideally I'd use Cache(R, P) -> Option<(bool, u32, Option<Key>)>

pub fn _var_name_kernel_direct_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    // println!("Rule: {:?}, Position: {:?}", rule, position);
    let cached_val: Option<(Option<bool>, u32, Key)>;
    let temp_key: Option<Key>;
    let curr_key: Key;
    {
        let cache = context.cache.borrow();
        cached_val = cache.check_LR(rule, position);
    }
    {
        let mut tree = context.stack.borrow_mut();
        temp_key = tree.last_node();
        curr_key = tree.add_node(rule, position, 0, temp_key, false);
        tree.set_last_node(Some(curr_key));
    }
    // If cached val exists. We don't use cache because we need to scope borrow_mut correctly
    if cached_val.is_some() {
        // println!("Entering Cached Function: {:?}", rule);
        let mut cached_val = cached_val.unwrap();
        if cached_val.0.is_none() {
            {
                let mut cache = context.cache.borrow_mut();
                cache.set_lr_detected(Some(rule));
                // println!("Setting LR Detected to '{:?}'", rule);
            }
            cached_val.0 = Some(false);
        }

        // Make cached subtree a child of parent and current node parent of subtree
        let mut tree = context.stack.borrow_mut();
        match temp_key {
            None => {}
            Some(tkey) => {
                tree.connect(tkey, curr_key);
            }
        }
        let result = (
            cached_val
                .0
                .expect("By this point we should have checked for None"),
            cached_val.1,
        );
        tree.set_node_start_position(curr_key, position);
        tree.set_node_end_position(curr_key, result.1);
        tree.set_node_result(curr_key, result.0);
        tree.set_last_node(temp_key);
        // println!("Exiting Cached Function: {:?}", rule);
        return result;
        // Return Result
    } else {
        // println!("Entering NonCached Function: {:?}", rule);

        {
            // println!("No Cached Value");
            // Solely for Deny LR
            let mut cache = context.cache.borrow_mut();
            // println!("Pushing to Cache");
            cache.push_deny_LR(rule, None, position, position, curr_key);
        }
        // println!("Entering, Rule: {:?}", rule);
        let mut result = func(context, source, position);
        // println!("Result: {:?}", result);
        // println!("Exiting, Rule: {:?}", rule);

        let lr_detected: bool;
        {
            // Cache Val - Scoping may let the compiler optimize better. May being the operative word.
            let mut cache = context.cache.borrow_mut();
            cache.push(rule, result.0, position, result.1, curr_key);
            lr_detected = cache.get_lr_detected(rule);
        }
        if lr_detected {
            result = grow_direct_lr(curr_key, rule, context, source, position, func);
        }

        // println!(
        //     "In _var_name: No Cache Rule: {:?}: Result {:?}",
        //     rule, result
        // );
        let mut tree = context.stack.borrow_mut();
        match temp_key {
            None => {}
            Some(tkey) => {
                tree.connect(tkey, curr_key);
            }
        }
        // Return Result
        tree.set_node_start_position(curr_key, position);
        tree.set_node_end_position(curr_key, result.1);
        tree.set_node_result(curr_key, result.0);
        tree.set_last_node(temp_key);
        // println!(
        //     "Exiting NonCached Function: {:?}: Result: {:?}",
        //     rule, result
        // );
        return result;
    }
}

// ###########
// Older Version of grow_direct_lr that maps badly to paper.

// pub fn grow_direct_lr<T: Cache, S: Publisher>(
//     reference: Key,
//     rule: Rules,
//     context: &Context<T, S>,
//     source: &Source,
//     position: u32,
//     func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
// ) -> (bool, u32) {
//     println!("In Grow LR with Rule: {:?}", rule);
//     let cached_val: Option<(Option<bool>, u32, Key)>;
//     {
//         let cache = context.cache.borrow();
//         cached_val = cache.check_LR(rule, position);
//     }
//     println!("Cached Val: {:?}", cached_val);
//     let mut result: (bool, u32);
//     let mut last_result: (bool, u32) = (false, position);
//     // Need to break one before the last result so that the Last Sequence can execute
//     println!("Reference: {:?}", reference);
//     let mut temp_key: Option<Key>;
//     let mut curr_key: Key;
//     {
//         let mut tree = context.stack.borrow_mut();
//         tree.set_last_node(Some(reference));
//     }

//     loop {
//         {
//             let mut tree = context.stack.borrow_mut();
//             temp_key = tree.last_node();
//             curr_key = tree.add_node(rule, position, 0, temp_key, false);
//             tree.set_last_node(Some(curr_key));
//         }
//         result = func(context, source, position);
//         if result.0 == false || result.1 <= last_result.1 {
//             break;
//         }
//         {
//             let mut cache = context.cache.borrow_mut();
//             cache.push(rule, result.0, position, result.1, reference);
//         }
//         last_result = result;
//         println!("Result: {:?}", result);
//         {
//             let mut tree = context.stack.borrow_mut();
//             match temp_key {
//                 None => {}
//                 Some(tkey) => {
//                     tree.connect(tkey, curr_key);
//                 }
//             }
//             // Return Result
//             tree.set_node_start_position(curr_key, position);
//             tree.set_node_end_position(curr_key, result.1);
//             tree.set_node_result(curr_key, result.0);
//             tree.set_last_node(Some(curr_key));
//         }
//     }

//     {
//         let mut cache = context.cache.borrow_mut();
//         cache.set_lr_detected(None);
//     }

//     println!("Returning: {:?}", last_result);
//     return (last_result.0, last_result.1);
// }

// pub fn _var_name_kernel_direct_lr<T: Cache, S: Publisher>(
//     rule: Rules,
//     context: &Context<T, S>,
//     source: &Source,
//     position: u32,
//     func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
// ) -> (bool, u32) {
//     // println!("Rule: {:?}, Position: {:?}", rule, position);
//     let cached_val: Option<(Option<bool>, u32, Key)>;
//     let temp_key: Option<Key>;
//     let curr_key: Key;
//     {
//         let cache = context.cache.borrow();
//         cached_val = cache.check_LR(rule, position);
//     }
//     {
//         let mut tree = context.stack.borrow_mut();
//         temp_key = tree.last_node();
//         curr_key = tree.add_node(rule, position, 0, temp_key, false);
//         tree.set_last_node(Some(curr_key));
//     }
//     // If cached val exists. We don't use cache because we need to scope borrow_mut correctly
//     if cached_val.is_some() {
//         // println!("Entering Cached Function: {:?}", rule);
//         let mut cached_val = cached_val.unwrap();
//         if cached_val.0.is_none() {
//             {
//                 let mut cache = context.cache.borrow_mut();
//                 cache.set_lr_detected(Some(rule));
//                 // println!("Setting LR Detected to '{:?}'", rule);
//             }
//             cached_val.0 = Some(false);
//         }

//         // Make cached subtree a child of parent and current node parent of subtree
//         let mut tree = context.stack.borrow_mut();
//         match temp_key {
//             None => {}
//             Some(tkey) => {
//                 tree.connect(tkey, curr_key);
//             }
//         }
//         let result = (
//             cached_val
//                 .0
//                 .expect("By this point we should have checked for None"),
//             cached_val.1,
//         );
//         tree.set_node_start_position(curr_key, position);
//         tree.set_node_end_position(curr_key, result.1);
//         tree.set_node_result(curr_key, result.0);
//         tree.set_last_node(temp_key);
//         // println!("Exiting Cached Function: {:?}", rule);
//         return result;
//         // Return Result
//     } else {
//         // println!("Entering NonCached Function: {:?}", rule);

//         {
//             // println!("No Cached Value");
//             // Solely for Deny LR
//             let mut cache = context.cache.borrow_mut();
//             // println!("Pushing to Cache");
//             cache.push_deny_LR(rule, None, position, position, curr_key);
//         }
//         // println!("Entering, Rule: {:?}", rule);
//         let mut result = func(context, source, position);
//         // println!("Result: {:?}", result);
//         // println!("Exiting, Rule: {:?}", rule);

//         let lr_detected: bool;
//         {
//             // Cache Val - Scoping may let the compiler optimize better. May being the operative word.
//             let mut cache = context.cache.borrow_mut();
//             cache.push(rule, result.0, position, result.1, curr_key);
//             lr_detected = cache.get_lr_detected(rule);
//         }
//         if lr_detected {
//             result = grow_direct_lr(curr_key, rule, context, source, position, func);
//         }

//         // println!(
//         //     "In _var_name: No Cache Rule: {:?}: Result {:?}",
//         //     rule, result
//         // );
//         let mut tree = context.stack.borrow_mut();
//         match temp_key {
//             None => {}
//             Some(tkey) => {
//                 tree.connect(tkey, curr_key);
//             }
//         }
//         // Return Result
//         tree.set_node_start_position(curr_key, position);
//         tree.set_node_end_position(curr_key, result.1);
//         tree.set_node_result(curr_key, result.0);
//         tree.set_last_node(temp_key);
//         // println!(
//         //     "Exiting NonCached Function: {:?}: Result: {:?}",
//         //     rule, result
//         // );
//         return result;
//     }
// }

pub fn grow_indirect_lr<T: Cache, S: Publisher>(
    reference: Key,
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    println!("In Indirect Grow LR with Rule: {:?}", rule);
    let cached_val: Option<(Option<bool>, u32, Key)>;
    {
        let cache = context.cache.borrow();
        cached_val = cache.check_LR(rule, position);
    }
    println!("Cached Val: {:?}", cached_val);
    let mut result: (bool, u32);
    let mut last_result: (bool, u32) = (false, position);
    // Need to break one before the last result so that the Last Sequence can execute
    println!("Reference: {:?}", reference);
    let mut temp_key: Option<Key>;
    let mut curr_key: Key;
    {
        let mut tree = context.stack.borrow_mut();
        tree.set_last_node(Some(reference));
    }

    loop {
        {
            let mut tree = context.stack.borrow_mut();
            temp_key = tree.last_node();
            curr_key = tree.add_node(rule, position, 0, temp_key, false);
            tree.set_last_node(Some(curr_key));
        }
        {
            let mut cache = context.cache.borrow_mut();
            let head: Option<&mut Head> = cache.get_indirect_lr_detected(position);
        }
        result = func(context, source, position);
        if result.0 == false || result.1 <= last_result.1 {
            break;
        }
        {
            let mut cache = context.cache.borrow_mut();
            cache.push(rule, result.0, position, result.1, reference);
        }
        last_result = result;
        println!("Result: {:?}", result);
        {
            let mut tree = context.stack.borrow_mut();
            match temp_key {
                None => {}
                Some(tkey) => {
                    tree.connect(tkey, curr_key);
                }
            }
            // Return Result
            tree.set_node_start_position(curr_key, position);
            tree.set_node_end_position(curr_key, result.1);
            tree.set_node_result(curr_key, result.0);
            tree.set_last_node(Some(curr_key));
        }
    }

    {
        let mut cache = context.cache.borrow_mut();
        cache.set_lr_detected(None);
    }

    println!("Returning: {:?}", last_result);
    return (last_result.0, last_result.1);
}

pub fn recall<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> Option<(Option<bool>, u32, Key)> {
    let mut cached_val: Option<(Option<bool>, u32, Key)>;
    let mut should_evaluate = false;
    {
        let mut cache = context.cache.borrow_mut();
        cached_val = cache.check_LR(rule, position);
        let head = cache.get_indirect_lr_detected(position);
        if head.is_none() {
            return cached_val;
        }
        let head: &mut Head = head.unwrap();
        if cached_val.is_none() && head.is_not_head_or_involved_set(rule) {
            // MEMO ENtry Fail
            return Some((None, position, Key(0)));
        }
        if head.is_in_eval_set_remove_if_true(rule) {
            should_evaluate = true;
        }
    }
    if should_evaluate {
        let result = func(context, source, position); // We don't need the result.
        cached_val = Some((Some(result.0), result.1, Key(0)));
    }

    return cached_val;
}
pub fn _var_name_kernel_indirect_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    // println!("Rule: {:?}, Position: {:?}", rule, position);
    let cached_val: Option<(Option<bool>, u32, Key)>;
    let temp_key: Option<Key>;
    let curr_key: Key;
    let detected_setup_lr: bool;
    {
        let cache = context.cache.borrow();
        detected_setup_lr = cache.get_lr_detected(rule);
    }
    if detected_setup_lr {
        let already_in_set: bool;
        {
            let mut cache = context.cache.borrow_mut();
            let head = cache
                .get_indirect_lr_detected(position)
                .expect("Should exist here");
            already_in_set = !head.push_involved_set(rule);
        }
        if already_in_set {
            return (false, position);
        }
        // Basically keep running subfunction until we try to evaluate the same rule that was already detected
        // As Left Recursive.
        let result = func(context, source, position);
    }
    // Real start

    cached_val = recall(rule, context, source, position, func);

    {
        let mut tree = context.stack.borrow_mut();
        temp_key = tree.last_node();
        curr_key = tree.add_node(rule, position, 0, temp_key, false);
        tree.set_last_node(Some(curr_key));
    }
    // If cached val exists. We don't use cache because we need to scope borrow_mut correctly
    if cached_val.is_some() {
        println!("Entering Cached Function: {:?}", rule);
        let mut cached_val = cached_val.unwrap();
        if cached_val.0.is_none() {
            {
                let mut cache = context.cache.borrow_mut();
                cache.set_indirect_lr_detected(rule, position);
                println!("Setting LR Detected to '{:?}'", rule);
            }
            cached_val.0 = Some(false);
        }

        // Make cached subtree a child of parent and current node parent of subtree
        let mut tree = context.stack.borrow_mut();
        match temp_key {
            None => {}
            Some(tkey) => {
                tree.connect(tkey, curr_key);
            }
        }
        let result = (
            cached_val
                .0
                .expect("By this point we should have checked for None"),
            cached_val.1,
        );
        tree.set_node_start_position(curr_key, position);
        tree.set_node_end_position(curr_key, result.1);
        tree.set_node_result(curr_key, result.0);
        tree.set_last_node(temp_key);
        println!("Exiting Cached Function: {:?}", rule);
        return result;
        // Return Result
    } else {
        println!("Entering NonCached Function: {:?}", rule);

        {
            // println!("No Cached Value");
            // Solely for Deny LR
            let mut cache = context.cache.borrow_mut();
            // println!("Pushing to Cache");
            cache.push_deny_LR(rule, None, position, position, curr_key);
        }
        // println!("Entering, Rule: {:?}", rule);
        let mut result = func(context, source, position);
        // println!("Result: {:?}", result);
        // println!("Exiting, Rule: {:?}", rule);

        let lr_detected: bool;
        {
            // Cache Val - Scoping may let the compiler optimize better. May being the operative word.
            let mut cache = context.cache.borrow_mut();
            cache.push(rule, result.0, position, result.1, curr_key);
            let head = cache.get_indirect_lr_detected(position);
            lr_detected = head.is_some();
        }
        // If lr_detected is_some then we need to trigger setup.
        if lr_detected {
            let is_head_rule: bool;
            {
                let mut cache = context.cache.borrow_mut();
                let head: &mut Head = cache
                    .get_indirect_lr_detected(position)
                    .expect("Should exist here");
                if head.get_head_rule() == rule {
                    is_head_rule = true;
                } else {
                    is_head_rule = false;
                }
            }
            if !is_head_rule {
                println!("Stepping up");
                return (false, 0); // To break up a level to be at the right place.
            } else {
                // On Entry current rule is never head rule.
                {
                    let mut cache = context.cache.borrow_mut();
                    let head: &mut Head = cache
                        .get_indirect_lr_detected(position)
                        .expect("Should exist here");
                    cache.set_lr_detected(Some(rule)); // Reusing this to say whether we just want to map which rules are involved in the left recursion.
                                                       // Detected setup_lr at the top
                }
                func(context, source, position); // We don't need the result.

                {
                    let mut cache = context.cache.borrow_mut();
                    let head: &mut Head = cache
                        .get_indirect_lr_detected(position)
                        .expect("Should exist here");
                    println!(
                        "Curr_Rule: {:?}, Position: {:?}, {:?}",
                        rule, position, head
                    );

                    cache.set_lr_detected(None);
                }
                result = grow_indirect_lr(curr_key, rule, context, source, position, func);
                //panic!("Stop");
            }

            //result = lr_answer(curr_key, rule, context, source, position, func);
        }

        println!(
            "In _var_name: No Cache Rule: {:?}: Result {:?}",
            rule, result
        );
        let mut tree = context.stack.borrow_mut();
        match temp_key {
            None => {}
            Some(tkey) => {
                tree.connect(tkey, curr_key);
            }
        }
        // Return Result
        tree.set_node_start_position(curr_key, position);
        tree.set_node_end_position(curr_key, result.1);
        tree.set_node_result(curr_key, result.0);
        tree.set_last_node(temp_key);
        println!(
            "Exiting NonCached Function: {:?}: Result: {:?}",
            rule, result
        );
        return result;
    }
}

#[cfg(test)]
mod tests {

    use super::_var_name;
    use crate::context::Context;
    use crate::source::Source;
    use crate::terminal::_terminal;
    use cache::{Cache, MyCache4};
    use publisher::{Publisher, Tree};
    use rules::rules::Rules;
    fn test_func<T: Cache, S: Publisher>(
        _context: &Context<T, S>,
        source: &Source,
        position: u32,
    ) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        x(source, position)
    }
    #[test]
    fn test_var_name() {
        let s = "aaa".to_string();
        let src_len: u32 = s.len() as u32;
        let s = Source::new(&s);
        let context = Context::<MyCache4, Tree>::new(src_len, 51);
        let func = _var_name(Rules::Alphabet_Lower, &context, test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }

    #[test]
    fn test_var_name_caching() {
        let s = "aaa".to_string();
        let src_len: u32 = s.len() as u32;

        let s = Source::new(&s);
        //let mut c = BTreeCache::new(0,0);
        let context = Context::<MyCache4, Tree>::new(src_len, 51);
        let func = _var_name(Rules::Alphabet_Lower, &context, test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }
}
