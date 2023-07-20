// def cache(func):
//     """Handles regular PEG"""
//     name = func.__name__
//     @wraps(func)
//     def kernel(*args):
//         obj = args[0]
//         args = args[1:]
//         position = obj.position
//         try:
//             bool, pos = obj.cache.get(name, position, args)
//             bool, pos = obj.cache.get(name, position, args)
//             if bool:
//                 obj.position = pos
//             if(func.__name__ in ["And_Predicate", "Not_Predicate", "Optional", "Ordered_Choice", "Sequence", "Var_Name","_TERMINAL", "many_A"] and bool == True):
//                 print(f"k: Token: {position}, {func.__name__} -> '{obj.src[position:obj.position]}'")
//             return bool
//         except KeyError:
//             bool = func(obj, *args)
//             obj.cache.set(name, position, args, (bool, obj.position))
//             if(func.__name__ in ["And_Predicate", "Not_Predicate", "Optional", "Ordered_Choice", "Sequence", "Var_Name", "_TERMINAL", "many_A"] and bool == True):
//                 print(f"nk: Token: {position}, {func.__name__} -> '{obj.src[position:obj.position]}'")
//             return bool
use crate::{Resolvable, output_stack::Stack};

// One per Parse
pub struct Cache {
    entries: Vec<ArgCache>, // Start Position encoded in the indexing of the Cache
}

impl Cache {
    
    pub fn new(size_of_source: i32, number_of_structs: i32) -> Self {
        let mut c = Cache {
            entries: Vec::with_capacity(size_of_source as usize),
        };
        for i in 0..size_of_source {
            // Ensures the Vector in Cache is as large as the input source
            c.entries.push(ArgCache {
                entries: Vec::with_capacity(number_of_structs as usize),
            });
            for _j in 0..number_of_structs {
                // Ensures the Vector in ArgCache is as large as the number of structs(Aka possible arguments since each struct implements resolvable, which is known at parser generation time)
                c.entries[i as usize].entries.push((false, i32::MAX));
            }
        }
        return c;
        // for every arg cache in c set size to <number_of_structs>
    }

    fn push(&mut self, position: i32, arg_key: i32, bool: bool, end_position: i32) {
        let arg_cache: &mut ArgCache = &mut self.entries[position as usize];
        arg_cache.entries[arg_key as usize] = (bool, end_position);
    }
    fn check(&self, position: i32, arg_key: i32) -> Option<(bool, i32)> {
        let ret: (bool, i32) = self.entries[position as usize].entries[arg_key as usize];
        if ret.1 != i32::MAX {
            // Result is returned to callee to unwrap
            return Some(ret);
        } else {
            // Tells callee to simply run the actual code instead of using cached value since one does not exist.
            return None;
        };
    }
}

// Create 1 per Position in Cache
pub struct ArgCache {
    entries: Vec<(bool, i32)>, // Struct type encoded in the position of the entries
}

fn grow_lr<T: Resolvable>( stack: &mut Stack,
    cache: &mut Cache,
    rule: T,
    arg_key: i32,
    position: i32,
    source: &str)-> (bool, i32)
    {
    loop{
        let loop_position = position;
        let ans = rule.resolve(stack, cache, position, source);
        if ans.0 == false || loop_position <= ans.1{
            break;
        }
        cache.push(loop_position, arg_key, ans.0, ans.1)
    }
    let ans = cache.check(position, arg_key);
    return ans.unwrap();

}

pub fn cache_struct_wrapper<T: Resolvable>(
    stack: &mut Stack,
    cache: &mut Cache,
    rule: T,
    arg_key: i32,
    position: i32,
    source: &str,
) -> (bool, i32) {
    let m = cache.check(position, arg_key);
    if m.is_none(){
        cache.push(position, arg_key, false, -1); // AKA Set to LR
        let ans = rule.resolve(stack, cache, position, source);
        cache.push(position, arg_key, ans.0, ans.1); // AKA Set to LR
        if ans.0 == false && ans.1 == -1{
            return grow_lr(stack, cache, rule, arg_key, position, source);
        }
        else{
            return ans;
        }
    }
    else{
        let m = m.unwrap();
        if m.0 == false && m.1 == -1{
            return (false, -1)
        }
        return m;
    }
    // if ret != None {
    //     let ret = ret.unwrap();
    //     if ret.0 == false && ret.1 == -1 {
    //         // AKA LR
    //         loop {
    //             let loop_position = position;
    //             println!("1 {}, {}, {}", loop_position, ret.0, ret.1);

    //             let ret = rule.resolve(stack, cache, loop_position, source);
    //             println!("2{}, {}, {}", loop_position, ret.0, ret.1);
    //             if ret.0 == false && loop_position <= ret.1  {
    //                 break;
    //             }
    //             println!(" 3{}, {}, {}", loop_position, ret.0, ret.1);
    //             cache.push(position, arg_key, ret.0, ret.1);

                
    //         }
    //         let ret = cache.check(position, arg_key).unwrap();
    //         println!("{}, {}", ret.0, ret.1);
    //         return ret;
    //     }
    //     return ret;
    // } else {
    //     cache.push(position, arg_key, false, -1); // AKA Set to LR
    //     let ret = rule.resolve(stack, cache, position, source);
    //     cache.push(position, arg_key, ret.0, ret.1);
    //     return ret;
    // }
}

pub fn cache_fn_wrapper(
    stack: &mut Stack,
    cache: &mut Cache,
    rule_function: fn(&mut Stack, &mut Cache, i32, &str) -> (bool, i32),
    arg_key: i32,
    position: i32,
    source: &str,
) -> (bool, i32) {
    /*
       Use this to wrap functions, i.e if using handwritten functions to improve performance.
    */
    let ret = cache.check(position, arg_key);
    if ret != None {
        let ret = ret.unwrap();
        // HERE
        return ret;
    } else {
        cache.push(position, arg_key, false, -1);
        let ret = rule_function(stack, cache, position, source);
        cache.push(position, arg_key, ret.0, ret.1);
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_Terminal;

    #[test]
    fn test_cache_nothing_cached() {
        // Simulating what would happen after a rule that consumes one character parses and returns true
        let arg_key: i32 = 1;
        let start_position = 0;
        let f = Cache::new(10, 10); // 10 just cos it's a test no particular meaning
        let s: Option<(bool, i32)> = f.check(start_position, arg_key);
        assert!(s.is_none());
    }

    #[test]
    fn test_cache() {
        // Simulating what would happen after a rule that consumes one character parses and returns true
        let arg_key: i32 = 1;
        let start_position = 0;
        let result = true;
        let end_position = 1;
        let mut f = Cache::new(10, 10); // 10 just cos it's a test no particular meaning
        f.push(start_position, arg_key, result, end_position);
        let (b, p) = f.check(start_position, arg_key).unwrap();
        println!("{:?}, {:?}", b, p);
        assert_eq!(b, true);
        assert_eq!(p, 1);
    }

    #[test]
    fn test_sample_struct() {
        let src = "Hello World";
        let position = 0;
        let mut cache = Cache::new(11, 1);
        let arg_key = 0;
        let s = cache.check(position, arg_key);
        assert_eq!(s, None);
        let rule = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let mut stack = Stack::new(100,100);
        let ret = cache_struct_wrapper(&mut stack , &mut cache, rule, arg_key, position, src);
        assert_eq!(ret.0, true);
        assert_eq!(ret.1, 1);
        let v = cache.entries[position as usize].entries[arg_key as usize];
        assert_eq!(v.0, true);
        assert_eq!(v.1, 1);
        let ret = cache_struct_wrapper(&mut stack, &mut cache, rule, arg_key, position, src);
        assert_eq!(ret.0, true);
        assert_eq!(ret.1, 1);
    }

    #[test]
    fn test_sample_function() {
        let src = "Hello World";
        let position = 0;
        let mut cache = Cache::new(11, 1);
        let arg_key = 0;
        let s = cache.check(position, arg_key);
        assert_eq!(s, None);
        fn thing(_stack: &mut Stack, _cache: &mut Cache, _position: i32, _source: &str) -> (bool, i32) {
            return (true, 1);
        }
        let mut stack = Stack::new(100,100);
        let ret = cache_fn_wrapper(&mut stack, &mut cache, thing, arg_key, position, src);
        assert_eq!(ret.0, true);
        assert_eq!(ret.1, 1);
        let v = cache.entries[position as usize].entries[arg_key as usize];
        assert_eq!(v.0, true);
        assert_eq!(v.1, 1);
        let ret = cache_fn_wrapper(&mut stack, &mut cache, thing, arg_key, position, src);
        assert_eq!(ret.0, true);
        assert_eq!(ret.1, 1);
    }
}
