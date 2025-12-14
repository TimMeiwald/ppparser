#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn s<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Some whitespace is necessary for differentiating words
    let closure_1 = _terminal(b' ');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn ws_kernel<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Some whitespace are never relevant
    let closure_1 = _terminal(b' ');
    let closure_2 = _terminal(b'\t');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'\r');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b'\n');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn ws<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws_kernel(parent, context, source, position)
    };
    let closure_2 = _zero_or_more(&closure_1);
    closure_2(parent, source, position)
}
#[allow(dead_code)]
pub fn wsc<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Comments should be retained for e.g formatters
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws_kernel(parent, context, source, position)
    };
    let closure_2 = _var_name(Rules::Comment, context, comment);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Multiline_comment, context, multiline_comment);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _zero_or_more(&closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn digits<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn character<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(0, 255);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn letters<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(97, 122);
    let closure_2 = _ordered_choice_match_range(65, 90);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn integer<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        digits(parent, context, source, position)
    };
    let closure_2 = _one_or_more(&closure_1);
    closure_2(parent, source, position)
}
#[allow(dead_code)]
pub fn float<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Integer, context, integer);
    let closure_2 = _terminal(b'.');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Integer, context, integer);
    let closure_5 = _terminal(b'"');
    let closure_6 = _terminal(b'"');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _ordered_choice(&closure_4, &closure_7);
    let closure_9 = _subexpression(&closure_8);
    let closure_10 = _sequence(&closure_3, &closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn char<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\'');
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        character(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b'\'');
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn string<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'"');
    let closure_2 = _terminal(b'"');
    let closure_3 = _not_predicate(&closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        character(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _zero_or_more(&closure_6);
    let closure_8 = _sequence(&closure_1, &closure_7);
    let closure_9 = _terminal(b'"');
    let closure_10 = _sequence(&closure_8, &closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn name_kernel<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Things can be ctypes or variables etc.
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        letters(parent, context, source, position)
    };
    let closure_2 = _terminal(b'_');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = move |parent: Key, source: &Source, position: u32| {
        letters(parent, context, source, position)
    };
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        digits(parent, context, source, position)
    };
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _terminal(b'_');
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _zero_or_more(&closure_10);
    let closure_12 = _sequence(&closure_4, &closure_11);
    closure_12(parent, source, position)
}
#[allow(dead_code)]
pub fn type_name<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        name_kernel(parent, context, source, position)
    };
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn variable_name<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        name_kernel(parent, context, source, position)
    };
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn comment<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"//");
    let closure_2 = _terminal(b'\n');
    let closure_3 = _not_predicate(&closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        character(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _zero_or_more(&closure_6);
    let closure_8 = _sequence(&closure_1, &closure_7);
    let closure_9 = _terminal(b'\n');
    let closure_10 = _sequence(&closure_8, &closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn core_types<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'f');
    let closure_2 = _terminal(b'u');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'i');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _terminal(b'8');
    let closure_8 = _string_terminal_opt_ascii(b"16");
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _string_terminal_opt_ascii(b"32");
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _string_terminal_opt_ascii(b"64");
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _sequence(&closure_6, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn array<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Fixed size array
    let closure_1 = _terminal(b'[');
    let closure_2 =
        move |parent: Key, source: &Source, position: u32| s(parent, context, source, position);
    let closure_3 = _zero_or_more(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _var_name(Rules::Core_types, context, core_types);
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 =
        move |parent: Key, source: &Source, position: u32| s(parent, context, source, position);
    let closure_8 = _zero_or_more(&closure_7);
    let closure_9 = _sequence(&closure_6, &closure_8);
    let closure_10 = _terminal(b';');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 =
        move |parent: Key, source: &Source, position: u32| s(parent, context, source, position);
    let closure_13 = _zero_or_more(&closure_12);
    let closure_14 = _sequence(&closure_11, &closure_13);
    let closure_15 = _var_name(Rules::Integer, context, integer);
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 =
        move |parent: Key, source: &Source, position: u32| s(parent, context, source, position);
    let closure_18 = _zero_or_more(&closure_17);
    let closure_19 = _sequence(&closure_16, &closure_18);
    let closure_20 = _terminal(b']');
    let closure_21 = _sequence(&closure_19, &closure_20);
    closure_21(parent, source, position)
}
#[allow(dead_code)]
pub fn type_expression<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Core_types, context, core_types);
    let closure_2 = _var_name(Rules::Array, context, array);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Structure_definition, context, structure_definition);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _var_name(
        Rules::Enumeration_definition,
        context,
        enumeration_definition,
    );
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Union_definition, context, union_definition);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Function, context, function);
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _var_name(Rules::Type_name, context, type_name);
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    closure_14(parent, source, position)
}
#[allow(dead_code)]
pub fn function<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Function is very much still a work in progress
    let closure_1 = _string_terminal_opt_ascii(b"function");
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn function_type_parameters<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"<>");
    let closure_2 = _terminal(b'"');
    let closure_3 = _terminal(b'"');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _ordered_choice(&closure_1, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn function_parameters<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"()");
    let closure_2 = _terminal(b'"');
    let closure_3 = _terminal(b'"');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _ordered_choice(&closure_1, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn function_body<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'{');
    let closure_2 = _var_name(Rules::Expression, context, expression);
    let closure_3 = _zero_or_more(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _terminal(b'}');
    let closure_6 = _sequence(&closure_4, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn structure_definition<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"struct");
    let closure_2 = _terminal(b'{');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Variable_name, context, variable_name);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b':');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = move |parent: Key, source: &Source, position: u32| {
        type_expression(parent, context, source, position)
    };
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b';');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _terminal(b'}');
    let closure_13 = _sequence(&closure_11, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn enumeration_definition<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"enum");
    let closure_2 = _terminal(b'{');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Variable_name, context, variable_name);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b':');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = move |parent: Key, source: &Source, position: u32| {
        type_expression(parent, context, source, position)
    };
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b';');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _terminal(b'}');
    let closure_13 = _sequence(&closure_11, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn union_definition<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"union");
    let closure_2 = _terminal(b'{');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Variable_name, context, variable_name);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b':');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = move |parent: Key, source: &Source, position: u32| {
        type_expression(parent, context, source, position)
    };
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b';');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _terminal(b'}');
    let closure_13 = _sequence(&closure_11, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn complex_assignment<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'{');
    let closure_2 = _var_name(Rules::Variable_name, context, variable_name);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b'=');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::Expression, context, expression);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _terminal(b';');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b'}');
    let closure_11 = _sequence(&closure_9, &closure_10);
    closure_11(parent, source, position)
}
#[allow(dead_code)]
pub fn type_definition<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"type");
    let closure_2 = _var_name(Rules::Type_name, context, type_name);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b':');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        type_expression(parent, context, source, position)
    };
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _terminal(b';');
    let closure_9 = _sequence(&closure_7, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn statement<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Type_definition, context, type_definition);
    let closure_2 = _var_name(Rules::Assignment, context, assignment);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn variable_assignment<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Variable_name, context, variable_name);
    let closure_2 = _terminal(b':');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Type_name, context, type_name);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b'=');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Expression, context, expression);
    let closure_9 = _sequence(&closure_7, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn expression<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Float, context, float);
    let closure_2 = _var_name(Rules::Integer, context, integer);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Char, context, char);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::String, context, string);
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Variable_name, context, variable_name);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Complex_assignment, context, complex_assignment);
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    closure_11(parent, source, position)
}
#[allow(dead_code)]
pub fn grammar<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Type_definition, context, type_definition);
    let closure_2 = _subexpression(&closure_1);
    let closure_3 = _zero_or_more(&closure_2);
    closure_3(parent, source, position)
}
