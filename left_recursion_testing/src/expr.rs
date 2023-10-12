use parser_core::{Context, Source, _sequence, _terminal, _var_name, Rules, _ordered_choice, _subexpression};

pub fn num(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    let char = source.get_char(position); // Optimized version is fine for testing. Known to work correctly with other caches on non-left recursion.
    if char > Some(47) && char < Some(58) {
        (true, position + 1)
    } else {
        (false, position)
    }
}

pub fn expr(context: &Context, source: &Source, position: u32) -> (bool, u32){
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
    use parser_core::Source;

    #[test]
    fn test_direct_left_recursion1() {
        // Will overflow stack if using Cache that does not support LR
        // Won't if using Cache that does support LR. 
        let string = "1-2".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = expr(&context, &source, position);
        assert_eq!(result, (true, 3));
    }
    #[test]
    fn test_direct_left_recursion2() {
        // Will overflow stack if using Cache that does not support LR
        // Won't if using Cache that does support LR. 
        let string = "1-2-3-4-5".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = expr(&context, &source, position);
        assert_eq!(result, (true, 9));
    }
    
}
