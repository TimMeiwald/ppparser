#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn alphabet_upper<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // We all love commments
    let closure_1 = _ordered_choice_match_range(65, 90);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn alphabet_lower<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(97, 122);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn num<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn numnozero<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(49, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn hexval<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    let closure_2 = _ordered_choice_match_range(65, 70);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn spaces<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\n');
    let closure_2 = _terminal(b'\t');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'\r');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b' ');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn specials<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        alphabet_upper(parent, context, source, position)
    };
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        alphabet_lower(parent, context, source, position)
    };
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Num, context, num);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        spaces(parent, context, source, position)
    };
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _not_predicate(&closure_8);
    let closure_10 = _ordered_choice_match_range(0, 255);
    let closure_11 = _sequence(&closure_9, &closure_10);
    closure_11(parent, source, position)
}
#[allow(dead_code)]
pub fn ascii<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(0, 255);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn apostrophe<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'"');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn quotationmark<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\'');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn left_angle_bracket<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'<');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn right_angle_bracket<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'>');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn left_bracket<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'(');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn right_bracket<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b')');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn assignment<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'=');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn end_rule<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b';');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn ampersand<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'&');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn exclamation_mark<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'!');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn plus<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'+');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn star<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'*');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn question_mark<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'?');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn comma<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b',');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn newline<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\n');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn backslash<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'/');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn var_name<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        alphabet_lower(parent, context, source, position)
    };
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        alphabet_upper(parent, context, source, position)
    };
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = move |parent: Key, source: &Source, position: u32| {
        alphabet_lower(parent, context, source, position)
    };
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        alphabet_upper(parent, context, source, position)
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
pub fn var_name_decl<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        left_angle_bracket(parent, context, source, position)
    };
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        var_name(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        right_angle_bracket(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn var_name_ref<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        left_angle_bracket(parent, context, source, position)
    };
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        var_name(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        right_angle_bracket(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn hex<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Replace with custom code
    let closure_1 = _terminal(b'0');
    let closure_2 = _terminal(b'x');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::HexVal, context, hexval);
    let closure_5 = _one_or_more(&closure_4);
    let closure_6 = _sequence(&closure_3, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn integer<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // No negative values since that is meaningless in this context
    let closure_1 = _var_name(Rules::NumNoZero, context, numnozero);
    let closure_2 = _var_name(Rules::Num, context, num);
    let closure_3 = _zero_or_more(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    closure_4(parent, source, position)
}
#[allow(dead_code)]
pub fn orderedchoicematchrange<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_2 = _terminal(b'[');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::Terminal, context, terminal);
    let closure_7 = _var_name(Rules::Integer, context, integer);
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    let closure_9 = _var_name(Rules::Hex, context, hex);
    let closure_10 = _ordered_choice(&closure_8, &closure_9);
    let closure_11 = _subexpression(&closure_10);
    let closure_12 = _sequence(&closure_5, &closure_11);
    let closure_13 = _terminal(b'.');
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _terminal(b'.');
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 = _var_name(Rules::Terminal, context, terminal);
    let closure_18 = _var_name(Rules::Integer, context, integer);
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _var_name(Rules::Hex, context, hex);
    let closure_21 = _ordered_choice(&closure_19, &closure_20);
    let closure_22 = _subexpression(&closure_21);
    let closure_23 = _sequence(&closure_16, &closure_22);
    let closure_24 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_25 = _sequence(&closure_23, &closure_24);
    let closure_26 = _terminal(b']');
    let closure_27 = _sequence(&closure_25, &closure_26);
    let closure_28 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_29 = _sequence(&closure_27, &closure_28);
    closure_29(parent, source, position)
}
#[allow(dead_code)]
pub fn subexpression<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        left_bracket(parent, context, source, position)
    };
    let closure_2 = _var_name(Rules::RHS, context, rhs);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        right_bracket(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn epsilon<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        quotationmark(parent, context, source, position)
    };
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        quotationmark(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn stringterminal<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Multibyte matches essentially
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        apostrophe(parent, context, source, position)
    };
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        apostrophe(parent, context, source, position)
    };
    let closure_3 = _not_predicate(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _var_name(Rules::ASCII, context, ascii);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        apostrophe(parent, context, source, position)
    };
    let closure_7 = _not_predicate(&closure_6);
    let closure_8 = _var_name(Rules::ASCII, context, ascii);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _one_or_more(&closure_10);
    let closure_12 = _sequence(&closure_5, &closure_11);
    let closure_13 = _subexpression(&closure_12);
    let closure_14 = _sequence(&closure_4, &closure_13);
    let closure_15 = move |parent: Key, source: &Source, position: u32| {
        apostrophe(parent, context, source, position)
    };
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 = _subexpression(&closure_16);
    let closure_18 = _var_name(Rules::Hex, context, hex);
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _var_name(Rules::Integer, context, integer);
    let closure_21 = _ordered_choice(&closure_19, &closure_20);
    closure_21(parent, source, position)
}
#[allow(dead_code)]
pub fn terminal<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        quotationmark(parent, context, source, position)
    };
    let closure_2 = _var_name(Rules::ASCII, context, ascii);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        quotationmark(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = move |parent: Key, source: &Source, position: u32| {
        quotationmark(parent, context, source, position)
    };
    let closure_8 = _terminal(b'\\');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b'n');
    let closure_11 = _terminal(b'r');
    let closure_12 = _ordered_choice(&closure_10, &closure_11);
    let closure_13 = _terminal(b't');
    let closure_14 = _ordered_choice(&closure_12, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _sequence(&closure_9, &closure_15);
    let closure_17 = move |parent: Key, source: &Source, position: u32| {
        quotationmark(parent, context, source, position)
    };
    let closure_18 = _sequence(&closure_16, &closure_17);
    let closure_19 = _subexpression(&closure_18);
    let closure_20 = _ordered_choice(&closure_6, &closure_19);
    let closure_21 = _var_name(Rules::Epsilon, context, epsilon);
    let closure_22 = _ordered_choice(&closure_20, &closure_21);
    closure_22(parent, source, position)
}
#[allow(dead_code)]
pub fn nucleus<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        Rules::OrderedChoiceMatchRange,
        context,
        orderedchoicematchrange,
    );
    let closure_2 = _var_name(Rules::Subexpression, context, subexpression);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Terminal, context, terminal);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::StringTerminal, context, stringterminal);
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Var_Name_Ref, context, var_name_ref);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_12 = _sequence(&closure_10, &closure_11);
    closure_12(parent, source, position)
}
#[allow(dead_code)]
pub fn atom<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::And_Predicate, context, and_predicate);
    let closure_2 = _var_name(Rules::Not_Predicate, context, not_predicate);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::One_Or_More, context, one_or_more);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::Zero_Or_More, context, zero_or_more);
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Optional, context, optional);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_14 = _sequence(&closure_12, &closure_13);
    closure_14(parent, source, position)
}
#[allow(dead_code)]
pub fn and_predicate<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ampersand(parent, context, source, position)
    };
    let closure_2 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_3 = _sequence(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn not_predicate<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        exclamation_mark(parent, context, source, position)
    };
    let closure_2 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_3 = _sequence(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn sequence<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Atom, context, atom);
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 =
        move |parent: Key, source: &Source, position: u32| comma(parent, context, source, position);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Atom, context, atom);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 =
        move |parent: Key, source: &Source, position: u32| comma(parent, context, source, position);
    let closure_11 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_12 = _sequence(&closure_10, &closure_11);
    let closure_13 = _var_name(Rules::Atom, context, atom);
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _zero_or_more(&closure_15);
    let closure_17 = _sequence(&closure_9, &closure_16);
    closure_17(parent, source, position)
}
#[allow(dead_code)]
pub fn ordered_choice<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Atom, context, atom);
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        backslash(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Atom, context, atom);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = move |parent: Key, source: &Source, position: u32| {
        backslash(parent, context, source, position)
    };
    let closure_11 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_12 = _sequence(&closure_10, &closure_11);
    let closure_13 = _var_name(Rules::Atom, context, atom);
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _zero_or_more(&closure_15);
    let closure_17 = _sequence(&closure_9, &closure_16);
    closure_17(parent, source, position)
}
#[allow(dead_code)]
pub fn one_or_more<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 =
        move |parent: Key, source: &Source, position: u32| plus(parent, context, source, position);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn zero_or_more<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 =
        move |parent: Key, source: &Source, position: u32| star(parent, context, source, position);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn optional<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        question_mark(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn whitespace<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b' ');
    let closure_2 = _terminal(b'\n');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'\r');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b'\t');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _zero_or_more(&closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn rhs<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Sequence, context, sequence);
    let closure_2 = _var_name(Rules::Ordered_Choice, context, ordered_choice);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Atom, context, atom);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn lhs<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Var_Name_Decl, context, var_name_decl);
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_3 = _var_name(Rules::Semantic_Instructions, context, semantic_instructions);
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _optional(&closure_7);
    let closure_9 = _sequence(&closure_1, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn rule<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::LHS, context, lhs);
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        assignment(parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::RHS, context, rhs);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = move |parent: Key, source: &Source, position: u32| {
        end_rule(parent, context, source, position)
    };
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _var_name(Rules::Comment, context, comment);
    let closure_17 = _zero_or_more(&closure_16);
    let closure_18 = _sequence(&closure_15, &closure_17);
    closure_18(parent, source, position)
}
#[allow(dead_code)]
pub fn grammar<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Double up dem comments
    let closure_1 = _var_name(Rules::Rule, context, rule);
    let closure_2 = _one_or_more(&closure_1);
    let closure_3 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_4 = _optional(&closure_3);
    let closure_5 = _sequence(&closure_2, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn comment<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_2 = _terminal(b'#');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b'#');
    let closure_5 = _not_predicate(&closure_4);
    let closure_6 = _var_name(Rules::ASCII, context, ascii);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _zero_or_more(&closure_8);
    let closure_10 = _sequence(&closure_3, &closure_9);
    let closure_11 = _terminal(b'#');
    let closure_12 = _sequence(&closure_10, &closure_11);
    let closure_13 = move |parent: Key, source: &Source, position: u32| {
        whitespace(parent, context, source, position)
    };
    let closure_14 = _sequence(&closure_12, &closure_13);
    closure_14(parent, source, position)
}
#[allow(dead_code)]
pub fn semantic_instructions<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Delete, context, delete);
    let closure_2 = _var_name(Rules::Passthrough, context, passthrough);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Inline, context, inline);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn delete<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(&[b'D', b'E', b'L', b'E', b'T', b'E']);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn passthrough<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(&[
        b'P', b'A', b'S', b'S', b'T', b'H', b'R', b'O', b'U', b'G', b'H',
    ]);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn inline<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Comment
    let closure_1 = _string_terminal_opt_ascii(&[b'I', b'n', b'l', b'i', b'n', b'e']);
    closure_1(parent, source, position)
}

#[allow(dead_code)]
pub fn test_lr_num<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Num, context, num);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn test_lr_expr<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Should match 0-0-0-0-0-0-0-0 etc
    let closure_1 = _var_name_direct_left_recursion(Rules::test_LR_expr, context, test_lr_expr);
    let closure_2 = _terminal(b'-');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::test_LR_num, context, test_lr_num);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name(Rules::test_LR_num, context, test_lr_num);
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn test_indirect_lr_num<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> =
        [Rules::test_indirect_LR_expr, Rules::test_indirect_LR_num].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        &involved_set,
        Rules::test_indirect_LR_expr,
        context,
        test_indirect_lr_expr,
    );
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn test_indirect_lr_expr<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Should match 0-0-0-0-0-0-0-0 etc
    let involved_set: Vec<Rules> =
        [Rules::test_indirect_LR_expr, Rules::test_indirect_LR_num].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        &involved_set,
        Rules::test_indirect_LR_num,
        context,
        test_indirect_lr_num,
    );
    let closure_2 = _terminal(b'-');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::test_LR_num, context, test_lr_num);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name(Rules::test_LR_num, context, test_lr_num);
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn test_term<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name_direct_left_recursion(Rules::test_term, context, test_term);
    let closure_2 = _terminal(b'+');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name_direct_left_recursion(Rules::test_fact, context, test_fact);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name_direct_left_recursion(Rules::test_term, context, test_term);
    let closure_8 = _terminal(b'-');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name_direct_left_recursion(Rules::test_fact, context, test_fact);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _ordered_choice(&closure_6, &closure_12);
    let closure_14 = _var_name_direct_left_recursion(Rules::test_fact, context, test_fact);
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn test_fact<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name_direct_left_recursion(Rules::test_fact, context, test_fact);
    let closure_2 = _terminal(b'*');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Num, context, num);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name_direct_left_recursion(Rules::test_fact, context, test_fact);
    let closure_8 = _terminal(b'/');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Num, context, num);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _ordered_choice(&closure_6, &closure_12);
    let closure_14 = _var_name(Rules::Num, context, num);
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    closure_15(parent, source, position)
}

#[allow(dead_code)]
pub fn test_term_indirect<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set = vec![Rules::test_term_indirect];
    let involved_set_for_test_fact = vec![Rules::test_fact_indirect];

    let closure_1 = _var_name_indirect_left_recursion(
        &involved_set,
        Rules::test_term_indirect,
        context,
        test_term_indirect,
    );
    let closure_2 = _terminal(b'+');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name_indirect_left_recursion(
        &involved_set_for_test_fact,
        Rules::test_fact_indirect,
        context,
        test_fact_indirect,
    );
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name_indirect_left_recursion(
        &involved_set,
        Rules::test_term_indirect,
        context,
        test_term_indirect,
    );
    let closure_8 = _terminal(b'-');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name_indirect_left_recursion(
        &involved_set_for_test_fact,
        Rules::test_fact_indirect,
        context,
        test_fact_indirect,
    );
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _ordered_choice(&closure_6, &closure_12);
    let closure_14 = _var_name_indirect_left_recursion(
        &involved_set_for_test_fact,
        Rules::test_fact_indirect,
        context,
        test_fact_indirect,
    );
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn test_fact_indirect<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set = vec![Rules::test_fact_indirect];
    let closure_1 = _var_name_indirect_left_recursion(
        &involved_set,
        Rules::test_fact_indirect,
        context,
        test_fact_indirect,
    );
    let closure_2 = _terminal(b'*');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Num, context, num);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name_indirect_left_recursion(
        &involved_set,
        Rules::test_fact_indirect,
        context,
        test_fact_indirect,
    );
    let closure_8 = _terminal(b'/');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Num, context, num);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _ordered_choice(&closure_6, &closure_12);
    let closure_14 = _var_name(Rules::Num, context, num);
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn test_indirect_three_level_a<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        Rules::test_indirect_three_level_B,
        context,
        test_indirect_three_level_b,
    );
    let closure_2 = _terminal(b'-');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::test_LR_num, context, test_lr_num);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name(Rules::test_LR_num, context, test_lr_num);
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn test_indirect_three_level_b<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        Rules::test_indirect_three_level_C,
        context,
        test_indirect_three_level_c,
    );
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn test_indirect_three_level_c<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        Rules::test_indirect_three_level_A,
        context,
        test_indirect_three_level_a,
    );
    closure_1(parent, source, position)
}
