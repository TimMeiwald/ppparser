use parser_core::_terminal;
use parser_core::Source;
use parser_core::_ordered_choice;

pub fn alphabet_lower(source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal('a' as u8);
    let t2 = _terminal('c' as u8);
    let oc1 = _ordered_choice(&t1, &t2);
    
    let t1 = _terminal('c' as u8);
    let t2 = _terminal('d' as u8);
    let oc2 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&oc1, &oc2);

    let t1 = _terminal('e' as u8);
    let t2 = _terminal('f' as u8);
    let oc3 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc3);

    let t1 = _terminal('g' as u8);
    let t2 = _terminal('h' as u8);
    let oc4 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc4);

    let t1 = _terminal('i' as u8);
    let t2 = _terminal('j' as u8);
    let oc5 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc5);

    let t1 = _terminal('k' as u8);
    let t2 = _terminal('l' as u8);
    let oc6 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc6);

    let t1 = _terminal('m' as u8);
    let t2 = _terminal('n' as u8);
    let oc7 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc7);

    let t1 = _terminal('o' as u8);
    let t2 = _terminal('p' as u8);
    let oc8 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc8);

    let t1 = _terminal('q' as u8);
    let t2 = _terminal('r' as u8);
    let oc9 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc9);

    let t1 = _terminal('s' as u8);
    let t2 = _terminal('t' as u8);
    let oc10 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc10);

    let t1 = _terminal('u' as u8);
    let t2 = _terminal('v' as u8);
    let oc11 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc11);

    let t1 = _terminal('w' as u8);
    let t2 = _terminal('x' as u8);
    let oc12 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc12);

    let t1 = _terminal('y' as u8);
    let t2 = _terminal('z' as u8);
    let oc13 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc13);
    
    hoc(source, position)

}



#[cfg(test)]
mod tests {
use parser_core::Source;
use super::*;
#[test]
fn test_alphabet_lower_false() {
    let string = "AAA".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = alphabet_lower(&source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_alphabet_lower_true() {
    let string = "aaa".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = alphabet_lower(&source, position);
    assert_eq!(result, (true, 1));
}
#[test]
fn test_alphabet_lower_true2() {
    let string = "zzz".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = alphabet_lower(&source, position);
    assert_eq!(result, (true, 1));
}
}
