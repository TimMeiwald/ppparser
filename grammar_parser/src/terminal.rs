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
    let t3 = _terminal('t' as u8);
    let t4 = _terminal('t' as u8);
    let oc3 = _ordered_choice(&t2, &t3);
    let oc4 = _ordered_choice(&oc3, &t4);
    let sub2 = _subexpression(&oc4);


    let s3 = _sequence(&apostrophe, &t1);
    let s4 = _sequence(&s3, &sub2);
    let s5 = _sequence(&s4, &apostrophe);
    let sub3 = _subexpression(&s5);

    let s6 = _sequence(&sub1, &sub3);
    let epsilon = _var_name(epsilon);
    let s7 = _sequence(&s6, &epsilon);

    s7(source, position)

}
