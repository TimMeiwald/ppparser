use crate::source::Source;
use crate::Context;
use crate::Rules;
use cache::Cache;
use stack::Stack;
pub fn _var_name_kernel<T: Cache, S: Stack>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    let cached_val: Option<(bool, u32)>;
    {
        let res = &mut *(context.cache).borrow_mut();
        cached_val = res.check(rule as u32, position);
    };
    let res = match cached_val {
        Some(cached_val) => {
            cached_val
        }
        None => {
            let result = func(context, source, position);
            {
                let cache = &mut *(context.cache).borrow_mut();
                cache.push(rule as u32, result.0, position, result.1);
            }
            result  
        }
    };
    // Want to cache whether bool is true or false since a sub rule can have false
    let stack = &mut *(context.stack).borrow_mut();
    let end_position = res.1;
    println!("The boolean of line below was : {}", res.0);
    stack.push(rule as u32, position, end_position);
    res
    
}



// Without stack
// pub fn _var_name_kernel<T: Cache>(
//     rule: Rules,
//     context: &Context<T>,
//     source: &Source,
//     position: u32,
//     func: fn(&Context<T>, &Source, u32) -> (bool, u32),
// ) -> (bool, u32) {
//     let cached_val: Option<(bool, u32)>;
//     {
//         let res = &mut *(context.cache).borrow_mut();
//         cached_val = res.check(rule as u32, position);
//     };
//     match cached_val {
//         Some(cached_val) => {
//             //println!("Cached");
//             println!("Not Cached {:?}", cached_val);
//             // {
//             //     let cache = &mut *(context.cache).borrow_mut();
//             //     cache.push(rule as u32, cached_val.0, position, cached_val.1); // Not sure if needed maybe for left recursion but unnecessary for no LR
//             // }
//             cached_val
//         }
//         None => {
//             //println!("Not cached");
//             let result = func(context, source, position);
//             {
//                 let cache = &mut *(context.cache).borrow_mut();
//                 cache.push(rule as u32, result.0, position, result.1);
//             }
//             println!("Cached {:?}", result);

//             result
//         }
//     }
// }











pub fn _var_name<T: Cache, S: Stack>(
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
    use crate::Rules;
    use cache::{Cache, MyCache4};
    use stack::{Stack, NoopStack};
    fn test_func<T: Cache, S: Stack>(_context: &Context<T, S>, source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        x(source, position)
    }
    #[test]
    fn test_var_name() {
        let s = "aaa".to_string();
        let src_len: u32 = s.len() as u32;
        let s = Source::new(s);
        let context = Context::<MyCache4, NoopStack>::new(src_len, 42);
        let func = _var_name(Rules::AlphabetLower, &context, test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }

    #[test]
    fn test_var_name_caching() {
        let s = "aaa".to_string();
        let src_len: u32 = s.len() as u32;

        let s = Source::new(s);
        //let mut c = BTreeCache::new(0,0);
        let context = Context::<MyCache4, NoopStack>::new(src_len, 42);
        let func = _var_name(Rules::AlphabetLower, &context, test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }
}
