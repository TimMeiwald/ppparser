//use parser_core::_terminal;
use parser_core::Source;
//use parser_core::_ordered_choice;
use cache::Cache;
use parser_core::Context;
use publisher::Publisher;
//Example of possible substiution optimization.
pub fn alphabet_lower<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let char = source.get_char(position);
    if char > Some(95) && char < Some(123) {
        (true, position + 1)
    } else {
        (false, position)
    }
}

// pub fn alphabet_lower(context: &Context,source: &Source, position: u32) -> (bool, u32){
//     let t1 = _terminal(b'a' );
//     let t2 = _terminal(b'b' );
//     let oc1 = _ordered_choice(&t1, &t2);

//     let t1 = _terminal(b'c' );
//     let t2 = _terminal(b'd' );
//     let oc2 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&oc1, &oc2);

//     let t1 = _terminal(b'e' );
//     let t2 = _terminal(b'f' );
//     let oc3 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc3);

//     let t1 = _terminal(b'g' );
//     let t2 = _terminal(b'h' );
//     let oc4 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc4);

//     let t1 = _terminal(b'i' );
//     let t2 = _terminal(b'j' );
//     let oc5 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc5);

//     let t1 = _terminal(b'k' );
//     let t2 = _terminal(b'l' );
//     let oc6 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc6);

//     let t1 = _terminal(b'm' );
//     let t2 = _terminal(b'n' );
//     let oc7 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc7);

//     let t1 = _terminal(b'o' );
//     let t2 = _terminal(b'p' );
//     let oc8 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc8);

//     let t1 = _terminal(b'q' );
//     let t2 = _terminal(b'r' );
//     let oc9 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc9);

//     let t1 = _terminal(b's' );
//     let t2 = _terminal(b't' );
//     let oc10 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc10);

//     let t1 = _terminal(b'u' );
//     let t2 = _terminal(b'v' );
//     let oc11 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc11);

//     let t1 = _terminal(b'w' );
//     let t2 = _terminal(b'x' );
//     let oc12 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc12);

//     let t1 = _terminal(b'y' );
//     let t2 = _terminal(b'z' );
//     let oc13 = _ordered_choice(&t1, &t2);
//     let hoc = _ordered_choice(&hoc, &oc13);

//     hoc(source, position)

// }

#[cfg(test)]
mod tests {
    use cache::MyCache4;
    use parser_core::Source;
    use publisher::Tree;

    use super::*;
    #[test]
    fn test_alphabet_lower_false() {
        let string = "AAA".to_string();

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(0, 44);

        let result = alphabet_lower(&context, &source, position);
        assert_eq!(result, (false, 0));
    }
    #[test]
    fn test_alphabet_lower_true() {
        let string = "aaa".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 44);

        let result = alphabet_lower(&context, &source, position);
        assert_eq!(result, (true, 1));
    }
    #[test]
    fn test_alphabet_lower_true2() {
        let string = "zzz".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 44);

        let result = alphabet_lower(&context, &source, position);
        assert_eq!(result, (true, 1));
    }
}
