use parser_core::_terminal;
use parser_core::Source;
use parser_core::_ordered_choice;
use parser_core::{Context, Rules};

pub fn alphabet_upper(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal('A' as u8);
    let t2 = _terminal('B' as u8);
    let oc1 = _ordered_choice(&t1, &t2);
    
    let t1 = _terminal('C' as u8);
    let t2 = _terminal('D' as u8);
    let oc2 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&oc1, &oc2);

    let t1 = _terminal('E' as u8);
    let t2 = _terminal('F' as u8);
    let oc3 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc3);

    let t1 = _terminal('G' as u8);
    let t2 = _terminal('H' as u8);
    let oc4 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc4);

    let t1 = _terminal('I' as u8);
    let t2 = _terminal('J' as u8);
    let oc5 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc5);

    let t1 = _terminal('K' as u8);
    let t2 = _terminal('L' as u8);
    let oc6 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc6);

    let t1 = _terminal('M' as u8);
    let t2 = _terminal('N' as u8);
    let oc7 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc7);

    let t1 = _terminal('O' as u8);
    let t2 = _terminal('P' as u8);
    let oc8 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc8);

    let t1 = _terminal('Q' as u8);
    let t2 = _terminal('R' as u8);
    let oc9 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc9);

    let t1 = _terminal('S' as u8);
    let t2 = _terminal('T' as u8);
    let oc10 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc10);

    let t1 = _terminal('U' as u8);
    let t2 = _terminal('V' as u8);
    let oc11 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc11);

    let t1 = _terminal('W' as u8);
    let t2 = _terminal('X' as u8);
    let oc12 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc12);

    let t1 = _terminal('Y' as u8);
    let t2 = _terminal('Z' as u8);
    let oc13 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc13);
    
    hoc(source, position)

}



#[cfg(test)]
mod tests {
use parser_core::Source;
use super::*;
use parser_core::_var_name;
#[test]
fn test_alphabet_upper_false() {
    let string = "aaa".to_string();
    let src_len = string.len();
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(src_len as u32, 42);

    let result = alphabet_upper(&context, &source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_alphabet_upper_true() {
    let string = "AAA".to_string();
    let src_len = string.len();
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(src_len as u32, 42);

    let result = alphabet_upper(&context, &source, position);
    assert_eq!(result, (true, 1));
}
#[test]
fn test_alphabet_upper_true_with_var_name() {
    let string = "AAA".to_string();
    let src_len = string.len();
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(src_len as u32, 42);

    let var_name_closure = _var_name(Rules::AlphabetUpper, &context, alphabet_upper);
    let result = var_name_closure(&source, position);
    assert_eq!(result, (true, 1));
}
}
