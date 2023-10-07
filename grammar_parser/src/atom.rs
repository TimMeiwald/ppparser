use parser_core::{_subexpression, _var_name, _ordered_choice, Source
};

use crate::{and_predicate, not_predicate, nucleus, whitespace, one_or_more, zero_or_more, optional, ordered_choice::ordered_choice};

pub fn atom(source: &Source, position: u32) -> (bool, u32){
    let v1 = _var_name(and_predicate);
    let v2 = _var_name(not_predicate);
    let v3 = _var_name(one_or_more);
    let v4 = _var_name(zero_or_more);
    let v5 = _var_name(optional);
    let v6 = _var_name(nucleus);

    let oc1 = _ordered_choice(&v1, &v2);
    let oc2 = _ordered_choice(&oc1, &v3);
    let oc3 = _ordered_choice(&oc2, &v4);
    let oc4 = _ordered_choice(&oc3, &v5);
    let oc5 = _ordered_choice(&oc4, &v6);

    let sub1 = _subexpression(&oc5);

    let v7 = _var_name(whitespace);
    let oc6 = _ordered_choice(&sub1, &v7);

    oc6(source, position)

}

#[cfg(test)]
mod tests {
use parser_core::Source;
use super::*;

#[test]
fn test_atom_true() {
    let string = "\"A\"/\"B\"/\"C\"/\"D\"/\"E\"/\"F\"/\"G\"/\"H\"/\"I\"/\"J\"/\"K\"/\"L\"/\"M\"/\"N\"/\"O\"/\"P\"/\"Q\"/\"R\"/\"S\"/\"T\"/\"U\"/\"V\"/\"W\"/\"X\"/\"Y\"/\"Z\"".to_string();
    let str_len =string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let result = atom(&source, position);
    assert_eq!(result, (true, str_len));
}




}