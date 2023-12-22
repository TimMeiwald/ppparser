use cache::Cache;
use parser_core::{
    Context, Rules, Source, _ordered_choice, _sequence, _subexpression, _terminal, _var_name,
};
#[allow(dead_code)]
pub fn num<T: Cache>(_context: &Context<T>, source: &Source, position: u32) -> (bool, u32) {
    let char = source.get_char(position); // Optimized version is fine for testing. Known to work correctly with other caches on non-left recursion.
    if char > Some(47) && char < Some(58) {
        (true, position + 1)
    } else {
        (false, position)
    }
}
#[allow(dead_code)]
pub fn expr<T: Cache>(context: &Context<T>, source: &Source, position: u32) -> (bool, u32) {
    // Using AlphabetLower for expr and Num for Num, don't want to pollute Rules nor use a trait.
    let t1 = _terminal(b'-');
    let expr = _var_name(Rules::AlphabetLower, context, expr);
    let num_closure = _var_name(Rules::Num, context, num);
    let seq1 = _sequence(&expr, &t1);
    let seq2 = _sequence(&seq1, &num_closure);
    let subexpress = _subexpression(&seq2);
    let oc1 = _ordered_choice(&subexpress, &num_closure);
    oc1(source, position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cache::{DenyLeftRecursionCache, AllowDirectLeftRecursionCache};
    use parser_core::{Source, _zero_or_more};
    
    #[test]
    fn test_basic_basic_num(){
        let string = "111".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<AllowDirectLeftRecursionCache>::new(src_len, 42);
        //let num_closure = _var_name(Rules::AlphabetLower, &context, num);
        let result = num(&context, &source, position);
        //let result = num_closure(&source, position);
        assert_eq!(result, (true, 1))
    }
    #[test]
    fn test_basic_num(){
        let string = "111".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<AllowDirectLeftRecursionCache>::new(src_len, 42);
        let num_closure = _var_name(Rules::AlphabetLower, &context, num);
        let result = num_closure(&source, position);
        assert_eq!(result, (true, 1))
    }

    #[test]
    fn test_num() {
        // To test it doesnt panic on a valid parse
        let string = "111".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<AllowDirectLeftRecursionCache>::new(src_len, 42);
        let num_closure = _var_name(Rules::AlphabetLower, &context, num);
        let z1 = _zero_or_more(&num_closure);
        let result = z1(&source, position);
        assert_eq!(result, (true, 3));
    }

    // Zero or more this so it increments position
    // Then check position and last rule to determine if there's direct recursion happening since
    // for zero or more etc it should increment if successful so it can fail on a recursion but not
    // on zero or more etc.

    // #[test]
    // fn test_direct_left_recursion1_deny() {
    //     // Will overflow stack if using Cache that does not support LR
    //     // Won't if using Cache that does support LR.
    //     let string = "1-2".to_string();
    //     let src_len = string.len() as u32;

    //     let source = Source::new(string);
    //     let position: u32 = 0;
    //     let context = Context::<DenyLeftRecursionCache>::new(src_len, 42);

    //     let result = expr(&context, &source, position);
    //     assert_eq!(result, (true, 3));
    // }
    // #[test]
    // //#[should_panic]
    // fn test_direct_left_recursion2_deny() {
    //     // Will give this result on LR Deny cache
    //     // Will overflow stack if using Cache that does not support LR
    //     // Won't if using Cache that does support LR.
    //     let string = "1-2-3-4-5".to_string();
    //     let src_len = string.len() as u32;

    //     let source = Source::new(string);
    //     let position: u32 = 0;
    //     let context = Context::<DenyLeftRecursionCache>::new(src_len, 42);

    //     let result = expr(&context, &source, position);
    //     assert_eq!(result, (true, 3));
    // }


    
    // #[test]
    // //#[should_panic]
    // fn test_direct_left_recursion1_allow() {
    //     // Will overflow stack if using Cache that does not support LR
    //     // Won't if using Cache that does support LR.
    //     let string = "1-2".to_string();
    //     let src_len = string.len() as u32;

    //     let source = Source::new(string);
    //     let position: u32 = 0;
    //     let context = Context::<AllowDirectLeftRecursionCache>::new(src_len, 42);

    //     let result = expr(&context, &source, position);
    //     assert_eq!(result, (true, 3));
    // }
    // #[test]
    // //#[should_panic]
    // fn test_direct_left_recursion2_allow() {
    //     // Will give this result on LR Deny cache
    //     // Will overflow stack if using Cache that does not support LR
    //     // Won't if using Cache that does support LR.
    //     let string = "1-2-3-4-5".to_string();
    //     let src_len = string.len() as u32;

    //     let source = Source::new(string);
    //     let position: u32 = 0;
    //     let context = Context::<AllowDirectLeftRecursionCache>::new(src_len, 42);

    //     let result = expr(&context, &source, position);
    //     assert_eq!(result, (true, 9));
    // }


}
