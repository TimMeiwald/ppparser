use parser_core::_terminal;
use parser_core::Source;
use parser_core::_ordered_choice;
use parser_core::{Context, Rules};

pub fn spaces(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal('\n' as u8);
    let t2 = _terminal('\t' as u8);
    let oc1 = _ordered_choice(&t1, &t2);
    
    let t1 = _terminal('\r' as u8);
    let t2 = _terminal(' ' as u8);
    let oc2 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&oc1, &oc2);

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

    let result = spaces(&context, &source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_num_true() {
    let string = "\n".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);

    let result = spaces(&context, &source, position);
    assert_eq!(result, (true, 1));
}

}