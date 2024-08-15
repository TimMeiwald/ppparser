use anyhow::Result;
use cache::*;
use grammar_parser::{grammar, test_lr_expr};
use parser_core::Context;
use parser_core::Source;
use publisher::*;
use rules::Key;
use rules::RULES_SIZE;
use std::fs::canonicalize;
use std::fs::read_to_string;
use std::path::Path;
use std::time::Instant;

pub fn parse<T: Cache, S: Publisher>(
    src: String,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32),
) -> Result<(bool, u32)> {
    let string = src;
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<T, S>::new(src_len, RULES_SIZE);
    let now = Instant::now();
    let result = func(&context, &source, position);
    let elapsed = now.elapsed();
    //context.stack.borrow().print(Key(0), None);
    // let only_true_tree = context.stack.borrow().clear_false();
    let elapsed2 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Elapsed with Tree cleaning: {:.2?}", elapsed2);

    //only_true_tree.print(Key(0), None);
    //println!("Capacity: {:?}, Len: {:?}", context.stack.borrow().capacity(), context.stack.borrow().len());
    //println!("Capacity: {:?}, Len: {:?}", only_true_tree.capacity(), only_true_tree.len());
    // for i in &*context.publisher.borrow() {
    //     // if i[0] == 20 || i[0] == 36 || i[0] == 29 || (i[0] >= 26 && i[0] <= 32) {
    //     //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);
    //     // }
    //     //println!("{}",i[0]);
    //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);
    // }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use grammar_parser::integer;

    use super::*;
    use std::panic::catch_unwind;
    use std::path::Path;

    // #[test]
    // fn test_overflows_by_default() {
    //     // Overflows always fails. Disable if not testing LR.
    //     let src: String = "0-0-0-0".to_string();
    //     let _x = catch_unwind(move || parse::<MyCache4, Tree>(src));
    //     assert!(_x.is_err());
    //     println!("{:?}", _x);
    // }

    #[test]
    fn test_no_recursion_deny_left_recursion_cache() {
        // Should not fail.
        let src: String = "1-2-3".to_string();

        let x = parse::<DenyLeftRecursionCache, Tree>(src, integer);
        assert_eq!(x.unwrap(), (true, 1));
    }

    #[test]
    fn test_recursion_deny_left_recursion_cache() {
        let src: String = "1-2-3".to_string();

        let x = parse::<DenyLeftRecursionCache, Tree>(src, test_lr_expr)
            .expect("Should succeed on fallback Num but not on full expr");
        assert_eq!(x, (true, 3));
        println!("{:?}", x);
    }

    #[test]
    fn test_no_recursion_direct_left_recursion_cache() {
        // Should not fail.
        let src: String = "1-2-3".to_string();

        let x = parse::<DirectLeftRecursionCache, Tree>(src, integer);
        assert_eq!(x.unwrap(), (true, 1));
    }

    #[test]
    fn test_recursion_direct_left_recursion_cache() {
        let src: String = "1-2-3-7-9   ".to_string();
        let x = parse::<DirectLeftRecursionCache, Tree>(src, test_lr_expr);
        // Not it should be 5 not 3 since it should grow the seed.
        assert_eq!(x.unwrap(), (true, 5));
    }
}
