#![allow(clippy::type_complexity)]

// While complex still under development and the core of the entire program is here so complexity is
// Acceptable
use crate::source::Source;
use crate::Context;
use cache::Cache;
use publisher::Publisher;
use rules::{Key, Rules};

pub fn _var_name_kernel<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
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
        {
            // Only necessary when Cache is DenyLeftRecursionCache
            let mut cache = context.cache.borrow_mut();
            cache.push_deny_LR(rule, None, position, 0, curr_key);
        }
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

pub fn _var_name<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> impl Fn(&Source, u32) -> (bool, u32) + '_ {
    move |source: &Source, position: u32| _var_name_kernel(rule, context, source, position, func)
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
