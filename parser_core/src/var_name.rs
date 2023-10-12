use crate::source::Source;
use crate::Context;
use crate::Rules;
use cache::Cache;
pub fn _var_name_kernel(
    rule: Rules,
    context: &Context,
    source: &Source,
    position: u32,
    func: fn(&Context, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    let cached_val: Option<(bool, u32)>;
    {
        let res = &mut *(context.cache).borrow_mut();
        cached_val = res.check(rule as u32, position);
    };
    match cached_val {
        Some(cached_val) => {
            //println!("Cached");
            // if cached_val.0 == true{
            //     println!("{:?} {:?} {:?}", rule, position, cached_val.1);
            // }
            cached_val
        }
        None => {
            //println!("Not cached");
            let result = func(context, source, position);
            {
                let cache = &mut *(context.cache).borrow_mut();
                cache.push(rule as u32, result.0, position, result.1);
            }
            // if result.0 == true{
            //     println!("{:?} {:?} {:?}", rule, position, result.1);
            // }
            result
        }
    }
}
// }
// pub fn _var_name(func: fn(&Source, u32) -> (bool, u32), cache: &mut impl Cache) -> impl FnMut(&Source, u32) -> (bool, u32) + '_ {
//     move |source: &Source, position: u32| _var_name_kernel(source, position, func, cache)
// }
// pub fn _var_name_kernel(
//     context: &Context,
//     source: &Source,
//     position: u32,
//     func: fn(&Context, &Source, u32) -> (bool, u32),
// ) -> (bool, u32) {
//     let result = func(context, source, position);
//     let cache = &mut *(context.cache).borrow_mut();
//     cache.push(0, true, 0, 0);
//     println!("Result: {:?}, {:?}", result.0, result.1);
//     result

// }

pub fn _var_name(
    rule: Rules,
    context: &Context,
    func: fn(&Context, &Source, u32) -> (bool, u32),
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
    fn test_func(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        x(source, position)
    }
    #[test]
    fn test_var_name() {
        let s = "aaa".to_string();
        let src_len: u32 = s.len() as u32;
        let s = Source::new(s);
        let context = Context::new(src_len, 42);
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
        let context = Context::new(src_len, 42);
        let func = _var_name(Rules::AlphabetLower, &context, test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }
}
