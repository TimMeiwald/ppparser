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
pub fn ascii<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Any ASCII char
    let closure_1 = _ordered_choice_match_range(0, 255);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn multiline_comment<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"/*");
    let closure_2 = _string_terminal_opt_ascii(b"*/");
    let closure_3 = _not_predicate(&closure_2);
    let closure_4 =
        move |parent: Key, source: &Source, position: u32| ascii(parent, context, source, position);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _zero_or_more(&closure_6);
    let closure_8 = _sequence(&closure_1, &closure_7);
    let closure_9 = _string_terminal_opt_ascii(b"*/");
    let closure_10 = _sequence(&closure_8, &closure_9);
    closure_10(parent, source, position)
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
    let closure_4 =
        move |parent: Key, source: &Source, position: u32| ascii(parent, context, source, position);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _zero_or_more(&closure_6);
    let closure_8 = _sequence(&closure_1, &closure_7);
    closure_8(parent, source, position)
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
pub fn int<T: Context + 'static>(
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
pub fn valid_thing_name<T: Context + 'static>(
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
pub fn name<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Reserved_words, context, reserved_words);
    let closure_2 = _subexpression(&closure_1);
    let closure_3 = _not_predicate(&closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        valid_thing_name(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn ctype<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        valid_thing_name(parent, context, source, position)
    };
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn reserved_words<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Not all there yet
    let closure_1 = _string_terminal_opt_ascii(b"void");
    let closure_2 = _string_terminal_opt_ascii(b"int");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _string_terminal_opt_ascii(b"float");
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _string_terminal_opt_ascii(b"double");
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _string_terminal_opt_ascii(b"return");
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn function_declaration<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Function_header, context, function_header);
    let closure_2 = _var_name(Rules::Wsc, context, wsc);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b';');
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn function_definition<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Function_header, context, function_header);
    let closure_2 = _var_name(Rules::Wsc, context, wsc);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Function_body, context, function_body);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn function_header<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Ctype, context, ctype);
    let closure_2 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Name, context, name);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Function_parameters, context, function_parameters);
    let closure_9 = _sequence(&closure_7, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn function_parameters<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'(');
    let closure_2 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Parameter, context, parameter);
    let closure_5 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _terminal(b',');
    let closure_8 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Parameter, context, parameter);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _zero_or_more(&closure_12);
    let closure_14 = _sequence(&closure_6, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _optional(&closure_15);
    let closure_17 = _sequence(&closure_3, &closure_16);
    let closure_18 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_19 = _sequence(&closure_17, &closure_18);
    let closure_20 = _terminal(b')');
    let closure_21 = _sequence(&closure_19, &closure_20);
    let closure_22 = _var_name(Rules::Wsc, context, wsc);
    let closure_23 = _sequence(&closure_21, &closure_22);
    closure_23(parent, source, position)
}
#[allow(dead_code)]
pub fn parameter<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Ctype, context, ctype);
    let closure_2 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Name, context, name);
    let closure_5 = _sequence(&closure_3, &closure_4);
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
    let closure_2 = _var_name(Rules::Wsc, context, wsc);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Statement, context, statement);
    let closure_5 = _var_name(Rules::Wsc, context, wsc);
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _zero_or_more(&closure_7);
    let closure_9 = _sequence(&closure_3, &closure_8);
    let closure_10 = _var_name(Rules::Wsc, context, wsc);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _terminal(b'}');
    let closure_13 = _sequence(&closure_11, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn function_call<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Name, context, name);
    let closure_2 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b'(');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Expression, context, expression);
    let closure_9 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _terminal(b',');
    let closure_12 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _var_name(Rules::Expression, context, expression);
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _subexpression(&closure_15);
    let closure_17 = _zero_or_more(&closure_16);
    let closure_18 = _sequence(&closure_10, &closure_17);
    let closure_19 = _subexpression(&closure_18);
    let closure_20 = _optional(&closure_19);
    let closure_21 = _sequence(&closure_7, &closure_20);
    let closure_22 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_23 = _sequence(&closure_21, &closure_22);
    let closure_24 = _terminal(b')');
    let closure_25 = _sequence(&closure_23, &closure_24);
    closure_25(parent, source, position)
}
#[allow(dead_code)]
pub fn statement<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // WIP
    let closure_1 = _var_name(Rules::Function_call, context, function_call);
    let closure_2 = _var_name(Rules::Statement_return, context, statement_return);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(
        Rules::Statement_variable_assignment,
        context,
        statement_variable_assignment,
    );
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name(Rules::Wsc, context, wsc);
    let closure_8 = _sequence(&closure_6, &closure_7);
    let closure_9 = _terminal(b';');
    let closure_10 = _sequence(&closure_8, &closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn statement_return<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"return");
    let closure_2 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Expression, context, expression);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn statement_variable_assignment<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Ctype, context, ctype);
    let closure_2 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Name, context, name);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _terminal(b'=');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 =
        move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _var_name(Rules::Expression, context, expression);
    let closure_13 = _sequence(&closure_11, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn expression<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Name, context, name);
    let closure_2 = _var_name(Rules::Int, context, int);
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
    let closure_1 = _var_name(Rules::Wsc, context, wsc);
    let closure_2 = _var_name(Rules::Function_definition, context, function_definition);
    let closure_3 = _var_name(Rules::Function_declaration, context, function_declaration);
    let closure_4 = _ordered_choice(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _zero_or_more(&closure_5);
    let closure_7 = _sequence(&closure_1, &closure_6);
    let closure_8 = _var_name(Rules::Wsc, context, wsc);
    let closure_9 = _sequence(&closure_7, &closure_8);
    closure_9(parent, source, position)
}
