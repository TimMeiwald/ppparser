use parser_core::_terminal;
use parser_core::Source;
use parser_core::_ordered_choice;
use parser_core::_var_name;
use super::*;
use parser_core::{Context, Rules};

pub fn ascii(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let t1 = _var_name(Rules::AlphabetLower, &context,alphabet_lower);
    let t2 = _var_name(Rules::AlphabetUpper, &context,alphabet_upper);
    let oc1 = _ordered_choice(&t1, &t2);
    
    let t1 = _var_name(Rules::Num, &context,num);
    let t2 = _var_name(Rules::Spaces, &context,spaces);
    let oc2 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&oc1, &oc2);

    let t1 = _var_name(Rules::Specials, &context,specials);
    let hoc = _ordered_choice(&hoc, &t1);

    hoc(source, position)

}


#[cfg(test)]
mod tests {
use parser_core::Source;
use super::*;
#[test]
fn test_ascii_true() {
    let string = "aaa".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);

    let result = ascii(&context, &source, position);
    assert_eq!(result, (true, 1));
}
#[test]
fn test_ascii_true2() {
    let string = "~".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::new(0, 0);

    let result = ascii(&context, &source, position);
    assert_eq!(result, (true, 1));
}

}