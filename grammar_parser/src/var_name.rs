use parser_core::_ordered_choice;
use parser_core::_sequence;
use parser_core::_subexpression;
use parser_core::_terminal;
use parser_core::Source;
use parser_core::_var_name;
use parser_core::_zero_or_more;
use super::*;

pub fn var_name(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(left_angle_bracket);
    let v2 = _var_name(alphabet_lower);
    let v3 = _var_name(alphabet_upper);
    let v4 = _terminal('_' as u8);
    let v5 = _var_name(right_angle_bracket);
    
    let oc1 = _ordered_choice(&v2, &v3);
    let oc2 = _ordered_choice(&oc1, &v4);
    let sub1 = _subexpression(&oc1);
    let sub2 = _subexpression(&oc2);
    let z1 = _zero_or_more(&sub2);

    let s1 = _sequence(&v1, &sub1);
    let s2 = _sequence(&z1, &v5);
    let s3 = _sequence(&s1, &s2);
    s3(source, position)
}


#[cfg(test)]
mod tests {
use parser_core::Source;
use super::*;
#[test]
fn test_num_false() {
    let string = "_this_is_not_a_valid_var_name".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = var_name(&source, position);
    assert_eq!(result, (false, 0));
}
#[test]
fn test_num_true() {
    let string = "<this_is_a_valid_var_name>".to_string();
    let source = Source::new(string);
    let position: u32 = 0;
    let result = var_name(&source, position);
    assert_eq!(result, (true, 26));
}

}