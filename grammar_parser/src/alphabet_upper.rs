//use parser_core::_terminal;
use parser_core::Source;
//use parser_core::_ordered_choice;
use parser_core::Context;
//Example of possible substiution optimization.
use cache::Cache;
use stack::Stack;

pub fn alphabet_upper<T: Cache, S: Stack>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let char = source.get_char(position);
    if char > Some(64) && char < Some(91) {
        (true, position + 1)
    } else {
        (false, position)
    }
}

// pub fn alphabet_upper(context: &Context,source: &Source, position: u32) -> (bool, u32){
//     let t1 = _terminal(b'A' );
//     let t2 = _terminal(b'B' );
//     let oc1 = _ordered_choice(&t1, &t2);

//     let t1 = _terminal(b'C' );
//     let t2 = _terminal(b'D' );
//     let oc2 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&oc1, &oc2);

//     let t1 = _terminal(b'E' );
//     let t2 = _terminal(b'F' );
//     let oc3 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc3);

//     let t1 = _terminal(b'G' );
//     let t2 = _terminal(b'H' );
//     let oc4 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc4);

//     let t1 = _terminal(b'I' );
//     let t2 = _terminal(b'J' );
//     let oc5 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc5);

//     let t1 = _terminal(b'K' );
//     let t2 = _terminal(b'L' );
//     let oc6 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc6);

//     let t1 = _terminal(b'M' );
//     let t2 = _terminal(b'N' );
//     let oc7 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc7);

//     let t1 = _terminal(b'O' );
//     let t2 = _terminal(b'P' );
//     let oc8 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc8);

//     let t1 = _terminal(b'Q' );
//     let t2 = _terminal(b'R' );
//     let oc9 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc9);

//     let t1 = _terminal(b'S' );
//     let t2 = _terminal(b'T' );
//     let oc10 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc10);

//     let t1 = _terminal(b'U' );
//     let t2 = _terminal(b'V' );
//     let oc11 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc11);

//     let t1 = _terminal(b'W' );
//     let t2 = _terminal(b'X' );
//     let oc12 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc12);

//     let t1 = _terminal(b'Y' );
//     let t2 = _terminal(b'Z' );
//     let oc13 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc13);

//     hoc(source, position)

// }

#[cfg(test)]
mod tests {
    use super::*;
    use cache::MyCache4;
    use parser_core::Rules;
    use parser_core::Source;
    use parser_core::_var_name;
    use stack::NoopStack;

    #[test]
    fn test_alphabet_upper_false() {
        let string = "aaa".to_string();
        let src_len = string.len();
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(src_len as u32, 43);

        let result = alphabet_upper(&context, &source, position);
        assert_eq!(result, (false, 0));
    }
    #[test]
    fn test_alphabet_upper_true() {
        let string = "AAA".to_string();
        let src_len = string.len();
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(src_len as u32, 43);

        let result = alphabet_upper(&context, &source, position);
        assert_eq!(result, (true, 1));
    }
    #[test]
    fn test_alphabet_upper_true_with_var_name() {
        let string = "AAA".to_string();
        let src_len = string.len();
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(src_len as u32, 43);

        let var_name_closure = _var_name(Rules::AlphabetUpper, &context, alphabet_upper);
        let result = var_name_closure(&source, position);
        assert_eq!(result, (true, 1));
    }
}
