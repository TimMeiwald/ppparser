#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn onenine<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(49, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn digit<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn integer<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Sign, context, sign);
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        onenine(parent, context, source, position)
    };
    let closure_3 =
        move |parent: Key, source: &Source, position: u32| digit(parent, context, source, position);
    let closure_4 = _one_or_more(&closure_3);
    let closure_5 = _sequence(&closure_2, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 =
        move |parent: Key, source: &Source, position: u32| digit(parent, context, source, position);
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    let closure_9 = _subexpression(&closure_8);
    let closure_10 = _sequence(&closure_1, &closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn number<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Integer, context, integer);
    let closure_2 = _var_name(Rules::Fraction, context, fraction);
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _var_name(Rules::Exponent, context, exponent);
    let closure_6 = _optional(&closure_5);
    let closure_7 = _sequence(&closure_4, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn fraction<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'.');
    let closure_2 =
        move |parent: Key, source: &Source, position: u32| digit(parent, context, source, position);
    let closure_3 = _one_or_more(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn exponent<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'E');
    let closure_2 = _terminal(b'e');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(Rules::Sign, context, sign);
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 =
        move |parent: Key, source: &Source, position: u32| digit(parent, context, source, position);
    let closure_8 = _one_or_more(&closure_7);
    let closure_9 = _sequence(&closure_6, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn sign<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'+');
    let closure_2 = _terminal(b'-');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _optional(&closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn addition<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Addition, Rules::Expr, Rules::Subtraction].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(&involved_set, Rules::Expr, context, expr);
    let closure_2 = _terminal(b'+');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let involved_set: Vec<Rules> = [Rules::Division, Rules::Multiplication, Rules::Term].to_vec();
    let closure_4 = _var_name_indirect_left_recursion(&involved_set, Rules::Term, context, term);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn subtraction<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Addition, Rules::Expr, Rules::Subtraction].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(&involved_set, Rules::Expr, context, expr);
    let closure_2 = _terminal(b'-');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let involved_set: Vec<Rules> = [Rules::Division, Rules::Multiplication, Rules::Term].to_vec();
    let closure_4 = _var_name_indirect_left_recursion(&involved_set, Rules::Term, context, term);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn multiplication<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Division, Rules::Multiplication, Rules::Term].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(&involved_set, Rules::Term, context, term);
    let closure_2 = _terminal(b'*');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Factor, context, factor);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn division<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Division, Rules::Multiplication, Rules::Term].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(&involved_set, Rules::Term, context, term);
    let closure_2 = _terminal(b'/');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Factor, context, factor);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn parentheses<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'(');
    let involved_set: Vec<Rules> = [Rules::Addition, Rules::Expr, Rules::Subtraction].to_vec();
    let closure_2 = _var_name_indirect_left_recursion(&involved_set, Rules::Expr, context, expr);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b')');
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn expr<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Expr, Rules::Addition].to_vec();
    let closure_1 =
        _var_name_indirect_left_recursion(&involved_set, Rules::Addition, context, addition);
    let involved_set: Vec<Rules> = [Rules::Expr, Rules::Subtraction].to_vec();
    let closure_2 =
        _var_name_indirect_left_recursion(&involved_set, Rules::Subtraction, context, subtraction);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let involved_set: Vec<Rules> = [Rules::Division, Rules::Multiplication, Rules::Term].to_vec();
    let closure_4 = _var_name_indirect_left_recursion(&involved_set, Rules::Term, context, term);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn term<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Term, Rules::Multiplication].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        &involved_set,
        Rules::Multiplication,
        context,
        multiplication,
    );
    let involved_set: Vec<Rules> = [Rules::Term, Rules::Division].to_vec();
    let closure_2 =
        _var_name_indirect_left_recursion(&involved_set, Rules::Division, context, division);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Factor, context, factor);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn factor<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Parentheses, context, parentheses);
    let closure_2 = _var_name(Rules::Number, context, number);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn grammar<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Addition, Rules::Expr, Rules::Subtraction].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(&involved_set, Rules::Expr, context, expr);
    closure_1(parent, source, position)
}
