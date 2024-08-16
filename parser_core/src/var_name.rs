#![allow(clippy::type_complexity)]

// While complex still under development and the core of the entire program is here so complexity is
// Acceptable
use crate::source::Source;
use crate::Context;
use cache::Cache;
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
    let mut second_last_result: (bool, u32) = (false, position);
    let mut last_result: (bool, u32) = (false, position);
    // Need to break one before the last result so that the Last Sequence can execute
    let mut temp_key: Option<Key> = Some(reference);
    {
        let mut tree = context.stack.borrow_mut();
        tree.set_last_node(temp_key);
    }
    let mut curr_key: Key;
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
        second_last_result = last_result;
        last_result = result;
        println!("Result: {:?}", result);

        {
            // Make cached subtree a child of parent and current node parent of subtree
            let mut tree = context.stack.borrow_mut();
            match temp_key {
                None => {}
                Some(tkey) => {
                    // To guard against any loops
                    if tkey != curr_key {
                        tree.connect(tkey, curr_key);
                    }
                }
            }
            let result = (second_last_result.0, second_last_result.1);
            tree.set_node_start_position(curr_key, position);
            tree.set_node_end_position(curr_key, result.1);
            tree.set_node_result(curr_key, result.0);
            tree.set_last_node(temp_key);
        }
    }

    {
        // Make cached subtree a child of parent and current node parent of subtree
        let mut tree = context.stack.borrow_mut();
        tree.connect(reference, curr_key);
        let result = (last_result.0, last_result.1);
        tree.set_node_start_position(curr_key, position);
        tree.set_node_end_position(curr_key, result.1);
        tree.set_node_result(curr_key, result.0);
        tree.set_last_node(temp_key);
    }

    {
        let mut cache = context.cache.borrow_mut();
        cache.set_lr_detected(None);
    }

    println!("Returning: {:?}", second_last_result);
    return (second_last_result.0, second_last_result.1);
}

pub fn _var_name_kernel_direct_lr<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    // println!("Rule: {:?}, Position: {:?}", rule, position);
    let cached_val: Option<(Option<bool>, u32, Key)>;
    let mut temp_key: Option<Key>;
    let mut curr_key: Key;
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
            return (false, 0);
        }

        // Make cached subtree a child of parent and current node parent of subtree
        let mut tree = context.stack.borrow_mut();
        match temp_key {
            None => {}
            Some(tkey) => {
                // To guard against any loops
                if tkey != curr_key {
                    tree.connect(tkey, curr_key);
                }
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
            let res = grow_direct_lr(curr_key, rule, context, source, position, func);
            result.0 = res.0;
            result.1 = res.1;
        }

        // println!(
        //     "In _var_name: No Cache Rule: {:?}: Result {:?}",
        //     rule, result
        // );
        let mut tree = context.stack.borrow_mut();
        match temp_key {
            None => {}
            Some(tkey) => {
                // To guard against any loops.
                if tkey != curr_key {
                    tree.connect(tkey, curr_key);
                }
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
