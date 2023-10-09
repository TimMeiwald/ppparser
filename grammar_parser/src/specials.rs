use parser_core::_terminal;
use parser_core::Source;
use parser_core::_ordered_choice;
use parser_core::{Context, Rules};

pub fn specials(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal('+' as u8);
    let t2 = _terminal('*' as u8);
    let oc1 = _ordered_choice(&t1, &t2);
    
    let t1 = _terminal('-' as u8);
    let t2 = _terminal('&' as u8);
    let oc2 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&oc1, &oc2);

    let t1 = _terminal('!' as u8);
    let t2 = _terminal('?' as u8);
    let oc3 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc3);

    let t1 = _terminal('<' as u8);
    let t2 = _terminal('>' as u8);
    let oc4 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc4);

    let t1 = _terminal('"' as u8);
    let t2 = _terminal('(' as u8);
    let oc5 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc5);

    let t1 = _terminal(')' as u8);
    let t2 = _terminal('_' as u8);
    let oc6 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc6);

    let t1 = _terminal(',' as u8);
    let t2 = _terminal('/' as u8);
    let oc7 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc7);

    let t1 = _terminal(';' as u8);
    let t2 = _terminal('=' as u8);
    let oc8 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc8);

    let t1 = _terminal('\\' as u8);
    let t2 = _terminal('#' as u8);
    let oc9 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc9);

    let t1 = _terminal(':' as u8);
    let t2 = _terminal('|' as u8);
    let oc10 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc10);

    let t1 = _terminal('.' as u8);
    let t2 = _terminal('{' as u8);
    let oc11 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc11);

    let t1 = _terminal('}' as u8);
    let t2 = _terminal('[' as u8);
    let oc12 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc12);

    let t1 = _terminal(']' as u8);
    let t2 = _terminal('%' as u8);
    let oc13 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc13);

    let t1 = _terminal('\'' as u8);
    let t2 = _terminal('^' as u8);
    let oc14 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&hoc, &oc14);
    
    let t1 = _terminal('~' as u8);
    let hoc = _ordered_choice(&hoc, &t1);
    
    hoc(source, position)

}

#[cfg(test)]
mod tests {
use parser_core::Source;
use super::*;
#[test]
fn test_specials_false() {
    let string = "aaa".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);

    let result = specials(&context, &source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_specials_true() {
    let string = '~'.to_string();
    let context = Context::new(0, 0);

    let source = Source::new(string);
    let position: u32 = 0;
    let result = specials(&context, &source, position);
    assert_eq!(result, (true, 1));
}

}