use super::*;
use parser_core::{Source, _var_name, _subexpression, _ordered_choice, _terminal, _sequence};


pub fn terminal(source: &Source, position: u32) -> (bool, u32){
    let apostrophe = _var_name(apostrophe);
    let ascii = _var_name(ascii);
    let s1 = _sequence(&apostrophe, &ascii);
    let s2 = _sequence(&s1, &apostrophe);
    let sub1 = _subexpression(&s2);


    let t1 = _terminal('\\' as u8);
    let t2 = _terminal('n' as u8);
    let t3 = _terminal('r' as u8);
    let t4 = _terminal('t' as u8);
    let oc3 = _ordered_choice(&t2, &t3);
    let oc4 = _ordered_choice(&oc3, &t4);
    let sub2 = _subexpression(&oc4);


    let s3 = _sequence(&apostrophe, &t1);
    let s4 = _sequence(&s3, &sub2);
    let s5 = _sequence(&s4, &apostrophe);
    let sub3 = _subexpression(&s5);

    let s6 = _ordered_choice(&sub1, &sub3);
    let epsilon = _var_name(epsilon);
    let s7 = _ordered_choice(&s6, &epsilon);

    s7(source, position)

}
#[cfg(test)]
mod tests {
use parser_core::Source;
use super::*;
#[test]
fn test_terminal_false() {
    let string = "\"a".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = terminal(&source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_terminal_true() {
    let string = "\"a\"".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = terminal(&source, position);
    assert_eq!(result, (true, 3));
}
#[test]
fn test_terminal_true1() {
    let string = "\"\n\"".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = terminal(&source, position);
    assert_eq!(result, (true, 3));
}
#[test]
fn test_terminal_true2() {
    let string = "\"\t\"".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = terminal(&source, position);
    assert_eq!(result, (true, 3));
}
#[test]
fn test_terminal_true3() {
    let string = "\"\r\"".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = terminal(&source, position);
    assert_eq!(result, (true, 3));
}

#[test]
fn test_terminal_true4() {
    let string = "\" \"".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = terminal(&source, position);
    assert_eq!(result, (true, 3));
}



}