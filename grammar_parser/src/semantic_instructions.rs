use cache::Cache;
use parser_core::Context;
use parser_core::Rules;
use parser_core::{Source, _ordered_choice, _sequence, _terminal, _var_name};

pub fn semantic_instructions<T: Cache>(
    context: &Context<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::Delete, context, delete);
    let v2 = _var_name(Rules::Passthrough, context, passthrough);
    let v3 = _var_name(Rules::Collect, context, collect);
    let s1 = _ordered_choice(&v1, &v2);
    let s2 = _ordered_choice(&s1, &v3);
    s2(source, position)
}

pub fn collect<T: Cache>(_context: &Context<T>, source: &Source, position: u32) -> (bool, u32) {
    let t1 = _terminal(b'C');
    let t2 = _terminal(b'O');
    let t3 = _terminal(b'L');
    let t4 = _terminal(b'L');
    let t5 = _terminal(b'E');
    let t6 = _terminal(b'C');
    let t7 = _terminal(b'T');
    let s1 = _sequence(&t1, &t2);
    let s2 = _sequence(&s1, &t3);
    let s3 = _sequence(&s2, &t4);
    let s4 = _sequence(&s3, &t5);
    let s5 = _sequence(&s4, &t6);
    let s6 = _sequence(&s5, &t7);
    s6(source, position)
}

pub fn delete<T: Cache>(_context: &Context<T>, source: &Source, position: u32) -> (bool, u32) {
    let t1 = _terminal(b'D');
    let t2 = _terminal(b'E');
    let t3 = _terminal(b'L');
    let t4 = _terminal(b'E');
    let t5 = _terminal(b'T');
    let t6 = _terminal(b'E');
    let s1 = _sequence(&t1, &t2);
    let s2 = _sequence(&s1, &t3);
    let s3 = _sequence(&s2, &t4);
    let s4 = _sequence(&s3, &t5);
    let s5 = _sequence(&s4, &t6);
    s5(source, position)
}

pub fn passthrough<T: Cache>(_context: &Context<T>, source: &Source, position: u32) -> (bool, u32) {
    let t1 = _terminal(b'P');
    let t2 = _terminal(b'A');
    let t3 = _terminal(b'S');
    let t4 = _terminal(b'S');
    let t5 = _terminal(b'T');
    let t6 = _terminal(b'H');
    let t7 = _terminal(b'R');
    let t8 = _terminal(b'O');
    let t9 = _terminal(b'U');
    let t10 = _terminal(b'G');
    let t11 = _terminal(b'H');
    let s1 = _sequence(&t1, &t2);
    let s2 = _sequence(&s1, &t3);
    let s3 = _sequence(&s2, &t4);
    let s4 = _sequence(&s3, &t5);
    let s5 = _sequence(&s4, &t6);
    let s6 = _sequence(&s5, &t7);
    let s7 = _sequence(&s6, &t8);
    let s8 = _sequence(&s7, &t9);
    let s9 = _sequence(&s8, &t10);
    let s10 = _sequence(&s9, &t11);
    s10(source, position)
}

#[cfg(test)]
mod tests {

    use super::*;
    use cache::MyCache4;
    use parser_core::Source;

    #[test]
    fn test_semantic_instruction_true() {
        let string = "PASSTHROUGH".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);
        let result = semantic_instructions(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }
    #[test]
    fn test_semantic_instruction_true_cache() {
        let string = "PASSTHROUGH".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);
        let result = semantic_instructions(&context, &source, position);
        assert_eq!(result, (true, src_len), "1");
        let result = semantic_instructions(&context, &source, position);
        println!("{:?}, {:?}", result.0, result.1);
        assert_eq!(result, (true, src_len), "2");
    }
}
