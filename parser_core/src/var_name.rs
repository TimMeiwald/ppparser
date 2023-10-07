use cache::Cache;

use crate::source::Source;
pub fn _var_name_kernel(
    source: &Source,
    position: u32,
    func: fn(&Source, u32) -> (bool, u32),
) -> (bool, u32) {
    let result = func(source, position);
    result

}

// pub fn _var_name_kernel(
//     source: &Source,
//     position: u32,
//     func: fn(&Source, u32) -> (bool, u32),
//     cache: &mut impl Cache
// ) -> (bool, u32) {
//     let cached_val = cache.check(0, position);
//     match cached_val{
//         Some(cached_val) => {
//             println!("Cached");

//             return *cached_val;
//         },
//         None => {
//             println!("Not cached");
//             let result = func(source, position);
//             cache.push(0, result.0, position, result.1);
//             return result
//         }
//     }

// }
// pub fn _var_name(func: fn(&Source, u32) -> (bool, u32), cache: &mut impl Cache) -> impl FnMut(&Source, u32) -> (bool, u32) + '_ {
//     move |source: &Source, position: u32| _var_name_kernel(source, position, func, cache)
// }
pub fn _var_name(func: fn(&Source, u32) -> (bool, u32)) -> impl Fn(&Source, u32) -> (bool, u32){
    move |source: &Source, position: u32| _var_name_kernel(source, position, func)
}

#[cfg(test)]
mod tests {
    use cache::{Cache, BTreeCache};

    use super::_var_name;
    use crate::source::Source;
    use crate::terminal::_terminal;
    fn test_func(source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        x(source, position)
    }
    #[test]
    fn test_var_name() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        //let mut c = BTreeCache::new(0,0);
        let mut func = _var_name(test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }

    #[test]
    fn test_var_name_caching() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        //let mut c = BTreeCache::new(0,0);
        let mut func = _var_name(test_func);
        let x = func(&s, 0);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }
    
}
