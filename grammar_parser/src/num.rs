use parser_core::_terminal;
use parser_core::Source;
use parser_core::_ordered_choice;
use parser_core::{Context, Rules};

pub fn num(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal('0' as u8);
    let t2 = _terminal('1' as u8);
    let oc1 = _ordered_choice(&t1, &t2);
    
    let t1 = _terminal('2' as u8);
    let t2 = _terminal('3' as u8);
    let oc2 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&oc1, &oc2);

    let t1 = _terminal('4' as u8);
    let t2 = _terminal('5' as u8);
    let oc3 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc3);

    let t1 = _terminal('6' as u8);
    let t2 = _terminal('7' as u8);
    let oc4 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc4);

    let t1 = _terminal('8' as u8);
    let t2 = _terminal('9' as u8);
    let oc5 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc5);

    hoc(source, position)

}


#[cfg(test)]
mod tests {
use parser_core::Source;
use super::*;
#[test]
fn test_num_false() {
    let string = "aaa".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);

    let result = num(&context, &source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_num_true() {
    let string = "511".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);

    let result = num(&context, &source, position);
    assert_eq!(result, (true, 1));
}

}