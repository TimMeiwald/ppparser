
use crate::source::Source;
use crate::Context;
use rules::rules::Rules;
use cache::{Cache, Index};
use stack::Stack;



pub fn _var_name_kernel<T: Cache, S: Stack>(
    rule: Rules,
    context: &Context<T, S>,
    source: &Source,
    position: u32,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> (bool, u32) {
    let cached_val: Option<(bool, u32, Index)>;
    {
        let res = &*(context.cache).borrow();
        cached_val = res.check(rule, position);
    };





    println!("Rule: {:?}", rule);
    match cached_val {
        Some(cached_val) => {
            // If True read results from stack and push back onto stack again 
            if cached_val.0
            {
                let stack = &mut *(context.stack).borrow_mut();
                let index = u32::from(cached_val.2);
                let result = stack.read_children(index);
                match result {
                    None => {}
                    Some(res) => {
                        for child_index in (res.0+1)..res.1{
                            let dets = stack.get(child_index);
                            stack.push(true, dets[0], dets[1], dets[2]);
                        }
                    }
                };

            }
            println!("Result: {:?} {:?}", cached_val.0, cached_val.1);
            (cached_val.0, cached_val.1)
        }, 
        None => {

            // Need to push before result to ensure that the order is correct, use patch to insert correct values after
            let index: u32;
            {   
                let stack = &mut *(context.stack).borrow_mut();
                index = stack.push(false, rule as u32, position, 0);
            }

            // Run function
            let result = func(context, source, position);
            // Patch in Result if True else pop to index. 
            if result.0{
                {
                    let stack = &mut *(context.stack).borrow_mut();
                    stack.patch(index, result.0, rule as u32, position, result.1);
                }

            }
            else {
                // Should pop anything where end_position doesn't get set.
                {   
                    let stack = &mut *(context.stack).borrow_mut();
                    stack.pop_to(index);
                }
            }
            {
                let cache = &mut *(context.cache).borrow_mut();
                cache.push(rule, result.0, position, result.1, Index(index));
            }
            println!("Result: {:?} {:?}", result.0, result.1);
            result
        }
    }

}


// pub fn _var_name_kernel<T: Cache, S: Stack>(
//     rule: Rules,
//     context: &Context<T, S>,
//     source: &Source,
//     position: u32,
//     func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
// ) -> (bool, u32) {
//     let cached_val: Option<(bool, u32, Index)>;
//     {
//         let res = &*(context.cache).borrow();
//         cached_val = res.check(rule as u32, position);
//     };
//     println!("Rule: {:?}", rule);
//     match cached_val {
//         Some(cached_val) => {
//             // If True read results from stack and push back onto stack again 
//             if cached_val.0
//             {
//                 let stack = &mut *(context.stack).borrow_mut();
//                 let index = u32::from(cached_val.2);
//                 let result = stack.read_children(index);
//                 match result {
//                     None => {}
//                     Some(res) => {
//                         for child_index in (res.0+1)..res.1{
//                             let dets = stack.get(child_index);
//                             stack.push(true, dets[0], dets[1], dets[2]);
//                         }
//                     }
//                 };

//             }
//             println!("Result: {:?} {:?}", cached_val.0, cached_val.1);
//             (cached_val.0, cached_val.1)
//         }, 
//         None => {

//             // Need to push before result to ensure that the order is correct, use patch to insert correct values after
//             let index: u32;
//             {   
//                 let stack = &mut *(context.stack).borrow_mut();
//                 index = stack.push(false, rule as u32, position, 0);
//             }

//             // Run function
//             let result = func(context, source, position);
//             // Patch in Result if True else pop to index. 
//             if result.0{
//                 {
//                     let stack = &mut *(context.stack).borrow_mut();
//                     stack.patch(index, result.0, rule as u32, position, result.1);
//                 }

//             }
//             else {
//                 // Should pop anything where end_position doesn't get set.
//                 {   
//                     let stack = &mut *(context.stack).borrow_mut();
//                     stack.pop_to(index);
//                 }
//             }
//             {
//                 let cache = &mut *(context.cache).borrow_mut();
//                 cache.push(rule as u32, result.0, position, result.1, Index(index));
//             }
//             println!("Result: {:?} {:?}", result.0, result.1);
//             result
//         }
//     }

// }

// Initial buggy _var_name_kernel
// pub fn _var_name_kernel<T: Cache, S: Stack>(
//     rule: Rules,
//     context: &Context<T, S>,
//     source: &Source,
//     position: u32,
//     func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
// ) -> (bool, u32) {
//     let cached_val: Option<(bool, u32)>;
//     {
//         let res = &*(context.cache).borrow();
//         cached_val = res.check(rule as u32, position);
//     };
//     let res = match cached_val {
//         Some(cached_val) => cached_val,
//         None => {
//             let result = func(context, source, position);
//             {
//                 let cache = &mut *(context.cache).borrow_mut();
//                 cache.push(rule as u32, result.0, position, result.1);
//             }
//             result
//         }
//     };
//     let end_position = res.1;
//     let is_true = res.0;
//     let stack = &mut *(context.stack).borrow_mut();
//     stack.push(is_true, rule as u32, position, end_position);
//     res
// }

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
    use rules::rules::Rules;
    use cache::{Cache, MyCache4};
    use stack::{NoopStack, Stack};
    fn test_func<T: Cache, S: Stack>(
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
        let s = Source::new(s);
        let context = Context::<MyCache4, NoopStack>::new(src_len, 44);
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
        let context = Context::<MyCache4, NoopStack>::new(src_len, 44);
        let func = _var_name(Rules::AlphabetLower, &context, test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }
}
