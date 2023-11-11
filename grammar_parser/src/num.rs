use parser_core::Context;
use parser_core::Source;
// use parser_core::_ordered_choice;
// use parser_core::_terminal;
use cache::Cache;

pub fn num<T: Cache>(_context: &Context<T>, source: &Source, position: u32) -> (bool, u32) {
    let char = source.get_char(position);
    if char > Some(47) && char < Some(58) {
        (true, position + 1)
    } else {
        (false, position)
    }
}

// pub fn num(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
//     let t1 = _terminal(b'0');
//     let t2 = _terminal(b'1');
//     let oc1 = _ordered_choice(&t1, &t2);

//     let t1 = _terminal(b'2');
//     let t2 = _terminal(b'3');
//     let oc2 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&oc1, &oc2);

//     let t1 = _terminal(b'4');
//     let t2 = _terminal(b'5');
//     let oc3 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc3);

//     let t1 = _terminal(b'6');
//     let t2 = _terminal(b'7');
//     let oc4 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc4);

//     let t1 = _terminal(b'8');
//     let t2 = _terminal(b'9');
//     let oc5 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc5);

//     hoc(source, position)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use parser_core::Source;
    use cache::MyCache4;

    #[test]
    fn test_num_false() {
        let string = "aaa".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);

        let result = num(&context, &source, position);
        assert_eq!(result, (false, 0));
    }
    #[test]
    fn test_num_true() {
        let string = "511".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);

        let result = num(&context, &source, position);
        assert_eq!(result, (true, 1));
    }
}
