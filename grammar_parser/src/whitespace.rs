use parser_core::{Source, _terminal, _ordered_choice, _subexpression, _zero_or_more};
use parser_core::{Context, Rules};

pub fn whitespace(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal(' ' as u8);
    let t2 = _terminal('\n' as u8);
    let t3 = _terminal('\r' as u8);
    let oc1 = _ordered_choice(&t1, &t2);
    let oc2 = _ordered_choice(&oc1, &t3);
    let sub1 = _subexpression(&oc2);
    let z1 = _zero_or_more(&sub1);
    z1(source, position)

}