#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::hooked_calls::{declared_new_typedef, is_typedef};
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn ascii<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Any ASCII char

    let closure_1 = _ordered_choice_match_range(0, 255);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn s<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Some whitespace is necessary for differentiating words

    let closure_1 = _terminal(b' ');
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn ws_kernel<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Some whitespace are never relevant

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
pub fn ws<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws_kernel(user_state, parent, context, source, position)
    };
    let closure_2 = _zero_or_more(&closure_1);
    closure_2(parent, source, position)
}
#[allow(dead_code)]
pub fn wsc<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Comments should be retained for e.g formatters

    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws_kernel(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(user_state, Rules::Comment, context, comment);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(
        user_state,
        Rules::Multiline_comment,
        context,
        multiline_comment,
    );
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _zero_or_more(&closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn multiline_comment<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"/*");
    let closure_2 = _string_terminal_opt_ascii(b"*/");
    let closure_3 = _not_predicate(&closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ascii(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _zero_or_more(&closure_6);
    let closure_8 = _sequence(&closure_1, &closure_7);
    let closure_9 = _string_terminal_opt_ascii(b"*/");
    let closure_10 = _sequence(&closure_8, &closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn comment<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"//");
    let closure_2 = _terminal(b'\n');
    let closure_3 = _not_predicate(&closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ascii(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _zero_or_more(&closure_6);
    let closure_8 = _sequence(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn keyword<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"one");
    let closure_2 = _string_terminal_opt_ascii(b"of");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _string_terminal_opt_ascii(b"auto");
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _string_terminal_opt_ascii(b"break");
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _string_terminal_opt_ascii(b"case");
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _string_terminal_opt_ascii(b"char");
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _string_terminal_opt_ascii(b"const");
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _string_terminal_opt_ascii(b"continue");
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    let closure_16 = _string_terminal_opt_ascii(b"default");
    let closure_17 = _ordered_choice(&closure_15, &closure_16);
    let closure_18 = _string_terminal_opt_ascii(b"do");
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _string_terminal_opt_ascii(b"double");
    let closure_21 = _ordered_choice(&closure_19, &closure_20);
    let closure_22 = _string_terminal_opt_ascii(b"else");
    let closure_23 = _ordered_choice(&closure_21, &closure_22);
    let closure_24 = _string_terminal_opt_ascii(b"enum");
    let closure_25 = _ordered_choice(&closure_23, &closure_24);
    let closure_26 = _string_terminal_opt_ascii(b"extern");
    let closure_27 = _ordered_choice(&closure_25, &closure_26);
    let closure_28 = _string_terminal_opt_ascii(b"float");
    let closure_29 = _ordered_choice(&closure_27, &closure_28);
    let closure_30 = _string_terminal_opt_ascii(b"for");
    let closure_31 = _ordered_choice(&closure_29, &closure_30);
    let closure_32 = _string_terminal_opt_ascii(b"goto");
    let closure_33 = _ordered_choice(&closure_31, &closure_32);
    let closure_34 = _string_terminal_opt_ascii(b"if");
    let closure_35 = _ordered_choice(&closure_33, &closure_34);
    let closure_36 = _string_terminal_opt_ascii(b"inline");
    let closure_37 = _ordered_choice(&closure_35, &closure_36);
    let closure_38 = _string_terminal_opt_ascii(b"int");
    let closure_39 = _ordered_choice(&closure_37, &closure_38);
    let closure_40 = _string_terminal_opt_ascii(b"long");
    let closure_41 = _ordered_choice(&closure_39, &closure_40);
    let closure_42 = _string_terminal_opt_ascii(b"register");
    let closure_43 = _ordered_choice(&closure_41, &closure_42);
    let closure_44 = _string_terminal_opt_ascii(b"restrict");
    let closure_45 = _ordered_choice(&closure_43, &closure_44);
    let closure_46 = _string_terminal_opt_ascii(b"return");
    let closure_47 = _ordered_choice(&closure_45, &closure_46);
    let closure_48 = _string_terminal_opt_ascii(b"short");
    let closure_49 = _ordered_choice(&closure_47, &closure_48);
    let closure_50 = _string_terminal_opt_ascii(b"signed");
    let closure_51 = _ordered_choice(&closure_49, &closure_50);
    let closure_52 = _string_terminal_opt_ascii(b"sizeof");
    let closure_53 = _ordered_choice(&closure_51, &closure_52);
    let closure_54 = _string_terminal_opt_ascii(b"static");
    let closure_55 = _ordered_choice(&closure_53, &closure_54);
    let closure_56 = _string_terminal_opt_ascii(b"struct");
    let closure_57 = _ordered_choice(&closure_55, &closure_56);
    let closure_58 = _string_terminal_opt_ascii(b"switch");
    let closure_59 = _ordered_choice(&closure_57, &closure_58);
    let closure_60 = _string_terminal_opt_ascii(b"typedef");
    let closure_61 = _ordered_choice(&closure_59, &closure_60);
    let closure_62 = _string_terminal_opt_ascii(b"union");
    let closure_63 = _ordered_choice(&closure_61, &closure_62);
    let closure_64 = _string_terminal_opt_ascii(b"unsigned");
    let closure_65 = _ordered_choice(&closure_63, &closure_64);
    let closure_66 = _string_terminal_opt_ascii(b"void");
    let closure_67 = _ordered_choice(&closure_65, &closure_66);
    let closure_68 = _string_terminal_opt_ascii(b"volatile");
    let closure_69 = _ordered_choice(&closure_67, &closure_68);
    let closure_70 = _string_terminal_opt_ascii(b"while");
    let closure_71 = _ordered_choice(&closure_69, &closure_70);
    let closure_72 = _string_terminal_opt_ascii(b"_Alignas");
    let closure_73 = _ordered_choice(&closure_71, &closure_72);
    let closure_74 = _string_terminal_opt_ascii(b"_Alignof");
    let closure_75 = _ordered_choice(&closure_73, &closure_74);
    let closure_76 = _string_terminal_opt_ascii(b"_Atomic");
    let closure_77 = _ordered_choice(&closure_75, &closure_76);
    let closure_78 = _string_terminal_opt_ascii(b"_Bool");
    let closure_79 = _ordered_choice(&closure_77, &closure_78);
    let closure_80 = _string_terminal_opt_ascii(b"_Complex");
    let closure_81 = _ordered_choice(&closure_79, &closure_80);
    let closure_82 = _string_terminal_opt_ascii(b"_Generic");
    let closure_83 = _ordered_choice(&closure_81, &closure_82);
    let closure_84 = _string_terminal_opt_ascii(b" _Imaginary");
    let closure_85 = _ordered_choice(&closure_83, &closure_84);
    let closure_86 = _string_terminal_opt_ascii(b"_Noreturn");
    let closure_87 = _ordered_choice(&closure_85, &closure_86);
    let closure_88 = _string_terminal_opt_ascii(b"_Static_assert");
    let closure_89 = _ordered_choice(&closure_87, &closure_88);
    let closure_90 = _string_terminal_opt_ascii(b"_Thread_local");
    let closure_91 = _ordered_choice(&closure_89, &closure_90);
    closure_91(parent, source, position)
}
#[allow(dead_code)]
pub fn identifier<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // <identifier> =  (<identifier>, <identifier_nondigit>)
    // /(<identifier>, <digit>)
    // /<identifier_nondigit>;

    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        identifier_nondigit(user_state, parent, context, source, position)
    };
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        identifier_nondigit(user_state, parent, context, source, position)
    };
    let closure_3 = _var_name(user_state, Rules::Digit, context, digit);
    let closure_4 = _ordered_choice(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _zero_or_more(&closure_5);
    let closure_7 = _sequence(&closure_1, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn identifier_nondigit<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Nondigit, context, nondigit);
    let closure_2 = _var_name(
        user_state,
        Rules::Universal_character_name,
        context,
        universal_character_name,
    );
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn nondigit<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(65, 90);
    let closure_2 = _ordered_choice_match_range(97, 122);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'_');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn digit<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn universal_character_name<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"\\u");
    let closure_2 = _var_name(user_state, Rules::Hex_quad, context, hex_quad);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _string_terminal_opt_ascii(b"\\U");
    let closure_6 = _var_name(user_state, Rules::Hex_quad, context, hex_quad);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(user_state, Rules::Hex_quad, context, hex_quad);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _ordered_choice(&closure_4, &closure_10);
    closure_11(parent, source, position)
}
#[allow(dead_code)]
pub fn hex_quad<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Hexadecimal_digit,
        context,
        hexadecimal_digit,
    );
    let closure_2 = _var_name(
        user_state,
        Rules::Hexadecimal_digit,
        context,
        hexadecimal_digit,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(
        user_state,
        Rules::Hexadecimal_digit,
        context,
        hexadecimal_digit,
    );
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(
        user_state,
        Rules::Hexadecimal_digit,
        context,
        hexadecimal_digit,
    );
    let closure_7 = _sequence(&closure_5, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Floating_constant,
        context,
        floating_constant,
    );
    let involved_set: Vec<Rules> = [
        Rules::Binary_constant,
        Rules::Decimal_constant,
        Rules::Integer_constant,
        Rules::Octal_constant,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Integer_constant,
        context,
        integer_constant,
    );
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(
        user_state,
        Rules::Enumeration_constant,
        context,
        enumeration_constant,
    );
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _var_name(
        user_state,
        Rules::Character_constant,
        context,
        character_constant,
    );
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn integer_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Decimal_constant].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Decimal_constant,
        context,
        decimal_constant,
    );
    let closure_2 = _var_name(user_state, Rules::Integer_suffix, context, integer_suffix);
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let involved_set: Vec<Rules> = [Rules::Binary_constant].to_vec();
    let closure_6 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Binary_constant,
        context,
        binary_constant,
    );
    let closure_7 = _var_name(user_state, Rules::Integer_suffix, context, integer_suffix);
    let closure_8 = _optional(&closure_7);
    let closure_9 = _sequence(&closure_6, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _ordered_choice(&closure_5, &closure_10);
    let involved_set: Vec<Rules> = [Rules::Octal_constant].to_vec();
    let closure_12 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Octal_constant,
        context,
        octal_constant,
    );
    let closure_13 = _var_name(user_state, Rules::Integer_suffix, context, integer_suffix);
    let closure_14 = _optional(&closure_13);
    let closure_15 = _sequence(&closure_12, &closure_14);
    let closure_16 = _subexpression(&closure_15);
    let closure_17 = _ordered_choice(&closure_11, &closure_16);
    let involved_set: Vec<Rules> = [Rules::Hexadecimal_constant].to_vec();
    let closure_18 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Hexadecimal_constant,
        context,
        hexadecimal_constant,
    );
    let closure_19 = _var_name(user_state, Rules::Integer_suffix, context, integer_suffix);
    let closure_20 = _optional(&closure_19);
    let closure_21 = _sequence(&closure_18, &closure_20);
    let closure_22 = _subexpression(&closure_21);
    let closure_23 = _ordered_choice(&closure_17, &closure_22);
    closure_23(parent, source, position)
}
#[allow(dead_code)]
pub fn decimal_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Decimal_constant].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Decimal_constant,
        context,
        decimal_constant,
    );
    let closure_2 = _var_name(user_state, Rules::Digit, context, digit);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(user_state, Rules::Nonzero_digit, context, nonzero_digit);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _ordered_choice(&closure_4, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn binary_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Binary_constant].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Binary_constant,
        context,
        binary_constant,
    );
    let closure_2 = _var_name(user_state, Rules::Binary_digit, context, binary_digit);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(user_state, Rules::Binary_prefix, context, binary_prefix);
    let closure_6 = _var_name(user_state, Rules::Binary_digit, context, binary_digit);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _ordered_choice(&closure_4, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn binary_prefix<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"0b");
    let closure_2 = _string_terminal_opt_ascii(b"0B");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn binary_digit<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'0');
    let closure_2 = _terminal(b'1');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn octal_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Octal_constant].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Octal_constant,
        context,
        octal_constant,
    );
    let closure_2 = _var_name(user_state, Rules::Octal_digit, context, octal_digit);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _terminal(b'0');
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn hexadecimal_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Hexadecimal_constant].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Hexadecimal_constant,
        context,
        hexadecimal_constant,
    );
    let closure_2 = _var_name(
        user_state,
        Rules::Hexadecimal_digit,
        context,
        hexadecimal_digit,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(
        user_state,
        Rules::Hexadecimal_prefix,
        context,
        hexadecimal_prefix,
    );
    let closure_6 = _var_name(
        user_state,
        Rules::Hexadecimal_digit,
        context,
        hexadecimal_digit,
    );
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _ordered_choice(&closure_4, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn hexadecimal_prefix<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"0x");
    let closure_2 = _string_terminal_opt_ascii(b"0X");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn nonzero_digit<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(49, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn octal_digit<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 55);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn hexadecimal_digit<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    let closure_2 = _ordered_choice_match_range(65, 70);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _ordered_choice_match_range(97, 102);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn integer_suffix<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Unsigned_suffix, context, unsigned_suffix);
    let closure_2 = _var_name(user_state, Rules::Long_suffix, context, long_suffix);
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _var_name(user_state, Rules::Unsigned_suffix, context, unsigned_suffix);
    let closure_7 = _var_name(
        user_state,
        Rules::Long_long_suffix,
        context,
        long_long_suffix,
    );
    let closure_8 = _optional(&closure_7);
    let closure_9 = _sequence(&closure_6, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _ordered_choice(&closure_5, &closure_10);
    let closure_12 = _var_name(user_state, Rules::Long_suffix, context, long_suffix);
    let closure_13 = _var_name(user_state, Rules::Unsigned_suffix, context, unsigned_suffix);
    let closure_14 = _optional(&closure_13);
    let closure_15 = _sequence(&closure_12, &closure_14);
    let closure_16 = _subexpression(&closure_15);
    let closure_17 = _ordered_choice(&closure_11, &closure_16);
    let closure_18 = _var_name(
        user_state,
        Rules::Long_long_suffix,
        context,
        long_long_suffix,
    );
    let closure_19 = _var_name(user_state, Rules::Unsigned_suffix, context, unsigned_suffix);
    let closure_20 = _optional(&closure_19);
    let closure_21 = _sequence(&closure_18, &closure_20);
    let closure_22 = _subexpression(&closure_21);
    let closure_23 = _ordered_choice(&closure_17, &closure_22);
    closure_23(parent, source, position)
}
#[allow(dead_code)]
pub fn unsigned_suffix<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'u');
    let closure_2 = _terminal(b'U');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn long_suffix<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'l');
    let closure_2 = _terminal(b'L');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn long_long_suffix<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"ll");
    let closure_2 = _string_terminal_opt_ascii(b"LL");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn floating_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Decimal_floating_constant,
        context,
        decimal_floating_constant,
    );
    let closure_2 = _var_name(
        user_state,
        Rules::Hexadecimal_floating_constant,
        context,
        hexadecimal_floating_constant,
    );
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn decimal_floating_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Fractional_constant,
        context,
        fractional_constant,
    );
    let closure_2 = _var_name(user_state, Rules::Exponent_part, context, exponent_part);
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _var_name(user_state, Rules::Floating_suffix, context, floating_suffix);
    let closure_6 = _optional(&closure_5);
    let closure_7 = _sequence(&closure_4, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _var_name(user_state, Rules::Digit_sequence, context, digit_sequence);
    let closure_10 = _var_name(user_state, Rules::Exponent_part, context, exponent_part);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _var_name(user_state, Rules::Floating_suffix, context, floating_suffix);
    let closure_13 = _optional(&closure_12);
    let closure_14 = _sequence(&closure_11, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _ordered_choice(&closure_8, &closure_15);
    closure_16(parent, source, position)
}
#[allow(dead_code)]
pub fn hexadecimal_floating_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Hexadecimal_prefix,
        context,
        hexadecimal_prefix,
    );
    let closure_2 = _var_name(
        user_state,
        Rules::Hexadecimal_fractional_constant,
        context,
        hexadecimal_fractional_constant,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(
        user_state,
        Rules::Binary_exponent_part,
        context,
        binary_exponent_part,
    );
    let closure_5 = _optional(&closure_4);
    let closure_6 = _sequence(&closure_3, &closure_5);
    let closure_7 = _var_name(user_state, Rules::Floating_suffix, context, floating_suffix);
    let closure_8 = _optional(&closure_7);
    let closure_9 = _sequence(&closure_6, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _var_name(
        user_state,
        Rules::Hexadecimal_prefix,
        context,
        hexadecimal_prefix,
    );
    let closure_12 = _var_name(
        user_state,
        Rules::Hexadecimal_digit_sequence,
        context,
        hexadecimal_digit_sequence,
    );
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _var_name(
        user_state,
        Rules::Binary_exponent_part,
        context,
        binary_exponent_part,
    );
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _var_name(user_state, Rules::Floating_suffix, context, floating_suffix);
    let closure_17 = _optional(&closure_16);
    let closure_18 = _sequence(&closure_15, &closure_17);
    let closure_19 = _subexpression(&closure_18);
    let closure_20 = _ordered_choice(&closure_10, &closure_19);
    closure_20(parent, source, position)
}
#[allow(dead_code)]
pub fn fractional_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Digit_sequence, context, digit_sequence);
    let closure_2 = _optional(&closure_1);
    let closure_3 = _terminal(b'.');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _var_name(user_state, Rules::Digit_sequence, context, digit_sequence);
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _var_name(user_state, Rules::Digit_sequence, context, digit_sequence);
    let closure_9 = _terminal(b'.');
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _subexpression(&closure_10);
    let closure_12 = _ordered_choice(&closure_7, &closure_11);
    closure_12(parent, source, position)
}
#[allow(dead_code)]
pub fn exponent_part<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'e');
    let closure_2 = _var_name(user_state, Rules::Sign, context, sign);
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _var_name(user_state, Rules::Digit_sequence, context, digit_sequence);
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _terminal(b'E');
    let closure_9 = _var_name(user_state, Rules::Sign, context, sign);
    let closure_10 = _optional(&closure_9);
    let closure_11 = _sequence(&closure_8, &closure_10);
    let closure_12 = _var_name(user_state, Rules::Digit_sequence, context, digit_sequence);
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_7, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn sign<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'+');
    let closure_2 = _terminal(b'-');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn digit_sequence<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Digit, context, digit);
    let closure_2 = _zero_or_more(&closure_1);
    closure_2(parent, source, position)
}
#[allow(dead_code)]
pub fn hexadecimal_fractional_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Hexadecimal_digit_sequence,
        context,
        hexadecimal_digit_sequence,
    );
    let closure_2 = _optional(&closure_1);
    let closure_3 = _terminal(b'.');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _var_name(
        user_state,
        Rules::Hexadecimal_digit_sequence,
        context,
        hexadecimal_digit_sequence,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _var_name(
        user_state,
        Rules::Hexadecimal_digit_sequence,
        context,
        hexadecimal_digit_sequence,
    );
    let closure_9 = _terminal(b'.');
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _subexpression(&closure_10);
    let closure_12 = _ordered_choice(&closure_7, &closure_11);
    closure_12(parent, source, position)
}
#[allow(dead_code)]
pub fn binary_exponent_part<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'p');
    let closure_2 = _var_name(user_state, Rules::Sign, context, sign);
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _var_name(user_state, Rules::Digit_sequence, context, digit_sequence);
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _terminal(b'P');
    let closure_9 = _var_name(user_state, Rules::Sign, context, sign);
    let closure_10 = _optional(&closure_9);
    let closure_11 = _sequence(&closure_8, &closure_10);
    let closure_12 = _var_name(user_state, Rules::Digit_sequence, context, digit_sequence);
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_7, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn hexadecimal_digit_sequence<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Hexadecimal_digit,
        context,
        hexadecimal_digit,
    );
    let closure_2 = _var_name(
        user_state,
        Rules::Hexadecimal_digit_sequence,
        context,
        hexadecimal_digit_sequence,
    );
    let closure_3 = _var_name(
        user_state,
        Rules::Hexadecimal_digit,
        context,
        hexadecimal_digit,
    );
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _ordered_choice(&closure_1, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn floating_suffix<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'f');
    let closure_2 = _terminal(b'l');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'F');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b'L');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn enumeration_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Identifier, context, identifier);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn character_constant<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\'');
    let involved_set: Vec<Rules> = [Rules::C_char_sequence].to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::C_char_sequence,
        context,
        c_char_sequence,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b'\'');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _terminal(b'L');
    let closure_8 = _terminal(b'\'');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let involved_set: Vec<Rules> = [Rules::C_char_sequence].to_vec();
    let closure_10 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::C_char_sequence,
        context,
        c_char_sequence,
    );
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _terminal(b'\'');
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_6, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn c_char_sequence<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::C_char_sequence].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::C_char_sequence,
        context,
        c_char_sequence,
    );
    let closure_2 = _var_name(user_state, Rules::C_char, context, c_char);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(user_state, Rules::C_char, context, c_char);
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn c_char<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\'');
    let closure_2 = _not_predicate(&closure_1);
    let closure_3 = _terminal(b'\\');
    let closure_4 = _not_predicate(&closure_3);
    let closure_5 = _sequence(&closure_2, &closure_4);
    let closure_6 = _terminal(b'\n');
    let closure_7 = _not_predicate(&closure_6);
    let closure_8 = _sequence(&closure_5, &closure_7);
    let closure_9 = move |parent: Key, source: &Source, position: u32| {
        ascii(user_state, parent, context, source, position)
    };
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _subexpression(&closure_10);
    let closure_12 = _var_name(user_state, Rules::Escape_sequence, context, escape_sequence);
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn escape_sequence<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Removed /<hexadecimal_escape_sequence> temporarily as breaking stuff

    let closure_1 = _var_name(
        user_state,
        Rules::Simple_escape_sequence,
        context,
        simple_escape_sequence,
    );
    let closure_2 = _var_name(
        user_state,
        Rules::Octal_escape_sequence,
        context,
        octal_escape_sequence,
    );
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(
        user_state,
        Rules::Universal_character_name,
        context,
        universal_character_name,
    );
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn simple_escape_sequence<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // PLACEHOLDER: HAVENT ADDED EVERYTHING YET, "\a"/ "\b"/ "\f"/ "\n"/ "\r"/ "\t"/ "\v"/ "\'"/ "\"" /"\\" /"\?";

    let closure_1 = _string_terminal_opt_ascii(b"PLACEHOLDER");
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn octal_escape_sequence<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\\');
    let closure_2 = _var_name(user_state, Rules::Octal_digit, context, octal_digit);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _terminal(b'\\');
    let closure_6 = _var_name(user_state, Rules::Octal_digit, context, octal_digit);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(user_state, Rules::Octal_digit, context, octal_digit);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _ordered_choice(&closure_4, &closure_10);
    let closure_12 = _terminal(b'\\');
    let closure_13 = _var_name(user_state, Rules::Octal_digit, context, octal_digit);
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _var_name(user_state, Rules::Octal_digit, context, octal_digit);
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 = _var_name(user_state, Rules::Octal_digit, context, octal_digit);
    let closure_18 = _sequence(&closure_16, &closure_17);
    let closure_19 = _subexpression(&closure_18);
    let closure_20 = _ordered_choice(&closure_11, &closure_19);
    closure_20(parent, source, position)
}
#[allow(dead_code)]
pub fn hexadecimal_escape_sequence<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"\\x");
    let closure_2 = _var_name(
        user_state,
        Rules::Hexadecimal_digit,
        context,
        hexadecimal_digit,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(
        user_state,
        Rules::Hexadecimal_escape_sequence,
        context,
        hexadecimal_escape_sequence,
    );
    let closure_6 = _var_name(
        user_state,
        Rules::Hexadecimal_digit,
        context,
        hexadecimal_digit,
    );
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _ordered_choice(&closure_4, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn string_literal<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Encoding_prefix, context, encoding_prefix);
    let closure_2 = _optional(&closure_1);
    let closure_3 = _terminal(b'"');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [Rules::S_char_sequence].to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::S_char_sequence,
        context,
        s_char_sequence,
    );
    let closure_6 = _optional(&closure_5);
    let closure_7 = _sequence(&closure_4, &closure_6);
    let closure_8 = _terminal(b'"');
    let closure_9 = _sequence(&closure_7, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn encoding_prefix<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"u8");
    let closure_2 = _terminal(b'u');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'U');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b'L');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn s_char_sequence<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::S_char_sequence].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::S_char_sequence,
        context,
        s_char_sequence,
    );
    let closure_2 = _var_name(user_state, Rules::S_char, context, s_char);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(user_state, Rules::S_char, context, s_char);
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn s_char<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'"');
    let closure_2 = _not_predicate(&closure_1);
    let closure_3 = _terminal(b'\\');
    let closure_4 = _not_predicate(&closure_3);
    let closure_5 = _sequence(&closure_2, &closure_4);
    let closure_6 = _terminal(b'\n');
    let closure_7 = _not_predicate(&closure_6);
    let closure_8 = _sequence(&closure_5, &closure_7);
    let closure_9 = move |parent: Key, source: &Source, position: u32| {
        ascii(user_state, parent, context, source, position)
    };
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _subexpression(&closure_10);
    let closure_12 = _var_name(user_state, Rules::Escape_sequence, context, escape_sequence);
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn punctuator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'[');
    let closure_2 = _terminal(b']');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'(');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b')');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _terminal(b'{');
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _terminal(b'}');
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _terminal(b'.');
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _string_terminal_opt_ascii(b"->");
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    let closure_16 = _string_terminal_opt_ascii(b"++");
    let closure_17 = _ordered_choice(&closure_15, &closure_16);
    let closure_18 = _string_terminal_opt_ascii(b"--");
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _terminal(b'&');
    let closure_21 = _ordered_choice(&closure_19, &closure_20);
    let closure_22 = _terminal(b'*');
    let closure_23 = _ordered_choice(&closure_21, &closure_22);
    let closure_24 = _terminal(b'+');
    let closure_25 = _ordered_choice(&closure_23, &closure_24);
    let closure_26 = _terminal(b'-');
    let closure_27 = _ordered_choice(&closure_25, &closure_26);
    let closure_28 = _terminal(b'~');
    let closure_29 = _ordered_choice(&closure_27, &closure_28);
    let closure_30 = _terminal(b'!');
    let closure_31 = _ordered_choice(&closure_29, &closure_30);
    let closure_32 = _terminal(b'/');
    let closure_33 = _ordered_choice(&closure_31, &closure_32);
    let closure_34 = _terminal(b'%');
    let closure_35 = _ordered_choice(&closure_33, &closure_34);
    let closure_36 = _string_terminal_opt_ascii(b"<<");
    let closure_37 = _ordered_choice(&closure_35, &closure_36);
    let closure_38 = _string_terminal_opt_ascii(b">>");
    let closure_39 = _ordered_choice(&closure_37, &closure_38);
    let closure_40 = _terminal(b'<');
    let closure_41 = _ordered_choice(&closure_39, &closure_40);
    let closure_42 = _terminal(b'>');
    let closure_43 = _ordered_choice(&closure_41, &closure_42);
    let closure_44 = _string_terminal_opt_ascii(b"<=");
    let closure_45 = _ordered_choice(&closure_43, &closure_44);
    let closure_46 = _string_terminal_opt_ascii(b">=");
    let closure_47 = _ordered_choice(&closure_45, &closure_46);
    let closure_48 = _string_terminal_opt_ascii(b"==");
    let closure_49 = _ordered_choice(&closure_47, &closure_48);
    let closure_50 = _string_terminal_opt_ascii(b"!=");
    let closure_51 = _ordered_choice(&closure_49, &closure_50);
    let closure_52 = _terminal(b'^');
    let closure_53 = _ordered_choice(&closure_51, &closure_52);
    let closure_54 = _terminal(b'|');
    let closure_55 = _ordered_choice(&closure_53, &closure_54);
    let closure_56 = _string_terminal_opt_ascii(b"&&");
    let closure_57 = _ordered_choice(&closure_55, &closure_56);
    let closure_58 = _string_terminal_opt_ascii(b"||");
    let closure_59 = _ordered_choice(&closure_57, &closure_58);
    let closure_60 = _terminal(b'?');
    let closure_61 = _ordered_choice(&closure_59, &closure_60);
    let closure_62 = _terminal(b':');
    let closure_63 = _ordered_choice(&closure_61, &closure_62);
    let closure_64 = _terminal(b';');
    let closure_65 = _ordered_choice(&closure_63, &closure_64);
    let closure_66 = _string_terminal_opt_ascii(b"...");
    let closure_67 = _ordered_choice(&closure_65, &closure_66);
    let closure_68 = _terminal(b'=');
    let closure_69 = _ordered_choice(&closure_67, &closure_68);
    let closure_70 = _string_terminal_opt_ascii(b"*=");
    let closure_71 = _ordered_choice(&closure_69, &closure_70);
    let closure_72 = _string_terminal_opt_ascii(b"/=");
    let closure_73 = _ordered_choice(&closure_71, &closure_72);
    let closure_74 = _string_terminal_opt_ascii(b"%=");
    let closure_75 = _ordered_choice(&closure_73, &closure_74);
    let closure_76 = _string_terminal_opt_ascii(b"+=");
    let closure_77 = _ordered_choice(&closure_75, &closure_76);
    let closure_78 = _string_terminal_opt_ascii(b"-=");
    let closure_79 = _ordered_choice(&closure_77, &closure_78);
    let closure_80 = _string_terminal_opt_ascii(b"<<=");
    let closure_81 = _ordered_choice(&closure_79, &closure_80);
    let closure_82 = _string_terminal_opt_ascii(b">>=");
    let closure_83 = _ordered_choice(&closure_81, &closure_82);
    let closure_84 = _string_terminal_opt_ascii(b"&=");
    let closure_85 = _ordered_choice(&closure_83, &closure_84);
    let closure_86 = _string_terminal_opt_ascii(b"^=");
    let closure_87 = _ordered_choice(&closure_85, &closure_86);
    let closure_88 = _string_terminal_opt_ascii(b"|=");
    let closure_89 = _ordered_choice(&closure_87, &closure_88);
    let closure_90 = _terminal(b',');
    let closure_91 = _ordered_choice(&closure_89, &closure_90);
    let closure_92 = _terminal(b'#');
    let closure_93 = _ordered_choice(&closure_91, &closure_92);
    let closure_94 = _string_terminal_opt_ascii(b"##");
    let closure_95 = _ordered_choice(&closure_93, &closure_94);
    let closure_96 = _string_terminal_opt_ascii(b"<:");
    let closure_97 = _ordered_choice(&closure_95, &closure_96);
    let closure_98 = _string_terminal_opt_ascii(b":>");
    let closure_99 = _ordered_choice(&closure_97, &closure_98);
    let closure_100 = _string_terminal_opt_ascii(b"<%");
    let closure_101 = _ordered_choice(&closure_99, &closure_100);
    let closure_102 = _string_terminal_opt_ascii(b"%>");
    let closure_103 = _ordered_choice(&closure_101, &closure_102);
    let closure_104 = _string_terminal_opt_ascii(b"%:");
    let closure_105 = _ordered_choice(&closure_103, &closure_104);
    let closure_106 = _string_terminal_opt_ascii(b"%:%:");
    let closure_107 = _ordered_choice(&closure_105, &closure_106);
    closure_107(parent, source, position)
}
#[allow(dead_code)]
pub fn header_name<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'<');
    let closure_2 = _var_name(user_state, Rules::H_char_sequence, context, h_char_sequence);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b'>');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _terminal(b'"');
    let closure_8 = _var_name(user_state, Rules::Q_char_sequence, context, q_char_sequence);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b'"');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _ordered_choice(&closure_6, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn h_char_sequence<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::H_char, context, h_char);
    let closure_2 = _var_name(user_state, Rules::H_char_sequence, context, h_char_sequence);
    let closure_3 = _var_name(user_state, Rules::H_char, context, h_char);
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _ordered_choice(&closure_1, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn h_char<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\n');
    let closure_2 = _not_predicate(&closure_1);
    let closure_3 = _terminal(b'<');
    let closure_4 = _not_predicate(&closure_3);
    let closure_5 = _sequence(&closure_2, &closure_4);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        ascii(user_state, parent, context, source, position)
    };
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn q_char_sequence<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Q_char, context, q_char);
    let closure_2 = _var_name(user_state, Rules::Q_char_sequence, context, q_char_sequence);
    let closure_3 = _var_name(user_state, Rules::Q_char, context, q_char);
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _ordered_choice(&closure_1, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn q_char<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\n');
    let closure_2 = _not_predicate(&closure_1);
    let closure_3 = _terminal(b'"');
    let closure_4 = _not_predicate(&closure_3);
    let closure_5 = _sequence(&closure_2, &closure_4);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        ascii(user_state, parent, context, source, position)
    };
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn pp_number<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Digit, context, digit);
    let closure_2 = _terminal(b'.');
    let closure_3 = _var_name(user_state, Rules::Digit, context, digit);
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _ordered_choice(&closure_1, &closure_5);
    let involved_set: Vec<Rules> = [Rules::Pp_number].to_vec();
    let closure_7 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Pp_number,
        context,
        pp_number,
    );
    let closure_8 = _var_name(user_state, Rules::Digit, context, digit);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _ordered_choice(&closure_6, &closure_10);
    let involved_set: Vec<Rules> = [Rules::Pp_number].to_vec();
    let closure_12 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Pp_number,
        context,
        pp_number,
    );
    let closure_13 = move |parent: Key, source: &Source, position: u32| {
        identifier_nondigit(user_state, parent, context, source, position)
    };
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _ordered_choice(&closure_11, &closure_15);
    let involved_set: Vec<Rules> = [Rules::Pp_number].to_vec();
    let closure_17 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Pp_number,
        context,
        pp_number,
    );
    let closure_18 = _terminal(b'e');
    let closure_19 = _sequence(&closure_17, &closure_18);
    let closure_20 = _var_name(user_state, Rules::Sign, context, sign);
    let closure_21 = _sequence(&closure_19, &closure_20);
    let closure_22 = _subexpression(&closure_21);
    let closure_23 = _ordered_choice(&closure_16, &closure_22);
    let involved_set: Vec<Rules> = [Rules::Pp_number].to_vec();
    let closure_24 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Pp_number,
        context,
        pp_number,
    );
    let closure_25 = _terminal(b'E');
    let closure_26 = _sequence(&closure_24, &closure_25);
    let closure_27 = _var_name(user_state, Rules::Sign, context, sign);
    let closure_28 = _sequence(&closure_26, &closure_27);
    let closure_29 = _subexpression(&closure_28);
    let closure_30 = _ordered_choice(&closure_23, &closure_29);
    let involved_set: Vec<Rules> = [Rules::Pp_number].to_vec();
    let closure_31 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Pp_number,
        context,
        pp_number,
    );
    let closure_32 = _terminal(b'p');
    let closure_33 = _sequence(&closure_31, &closure_32);
    let closure_34 = _var_name(user_state, Rules::Sign, context, sign);
    let closure_35 = _sequence(&closure_33, &closure_34);
    let closure_36 = _subexpression(&closure_35);
    let closure_37 = _ordered_choice(&closure_30, &closure_36);
    let involved_set: Vec<Rules> = [Rules::Pp_number].to_vec();
    let closure_38 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Pp_number,
        context,
        pp_number,
    );
    let closure_39 = _terminal(b'P');
    let closure_40 = _sequence(&closure_38, &closure_39);
    let closure_41 = _var_name(user_state, Rules::Sign, context, sign);
    let closure_42 = _sequence(&closure_40, &closure_41);
    let closure_43 = _subexpression(&closure_42);
    let closure_44 = _ordered_choice(&closure_37, &closure_43);
    let involved_set: Vec<Rules> = [Rules::Pp_number].to_vec();
    let closure_45 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Pp_number,
        context,
        pp_number,
    );
    let closure_46 = _terminal(b'.');
    let closure_47 = _sequence(&closure_45, &closure_46);
    let closure_48 = _subexpression(&closure_47);
    let closure_49 = _ordered_choice(&closure_44, &closure_48);
    closure_49(parent, source, position)
}
#[allow(dead_code)]
pub fn primary_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(user_state, Rules::Identifier, context, identifier);
    let involved_set: Vec<Rules> = [
        Rules::Binary_constant,
        Rules::Constant,
        Rules::Decimal_constant,
        Rules::Integer_constant,
        Rules::Octal_constant,
    ]
    .to_vec();
    let closure_3 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Constant,
        context,
        constant,
    );
    let closure_4 = _ordered_choice(&closure_2, &closure_3);
    let closure_5 = _var_name(user_state, Rules::String_literal, context, string_literal);
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    let closure_7 = _terminal(b'(');
    let closure_8 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_9 = _sequence(&closure_7, &closure_8);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_10 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _terminal(b')');
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _subexpression(&closure_15);
    let closure_17 = _ordered_choice(&closure_6, &closure_16);
    let closure_18 = _var_name(
        user_state,
        Rules::Generic_selection,
        context,
        generic_selection,
    );
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _subexpression(&closure_19);
    let closure_21 = _sequence(&closure_1, &closure_20);
    let closure_22 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_23 = _sequence(&closure_21, &closure_22);
    closure_23(parent, source, position)
}
#[allow(dead_code)]
pub fn generic_selection<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"_Generic");
    let closure_2 = _terminal(b'(');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_4 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b',');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(
        user_state,
        Rules::Generic_assoc_list,
        context,
        generic_assoc_list,
    );
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b')');
    let closure_11 = _sequence(&closure_9, &closure_10);
    closure_11(parent, source, position)
}
#[allow(dead_code)]
pub fn generic_assoc_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Generic_association,
        context,
        generic_association,
    );
    let closure_2 = _var_name(
        user_state,
        Rules::Generic_assoc_list,
        context,
        generic_assoc_list,
    );
    let closure_3 = _terminal(b',');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _var_name(
        user_state,
        Rules::Generic_association,
        context,
        generic_association,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn generic_association<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Type_name, context, type_name);
    let closure_2 = _terminal(b':');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_4 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _string_terminal_opt_ascii(b"default");
    let closure_8 = _terminal(b':');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_10 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _ordered_choice(&closure_6, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn postfix_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Primary_expression,
        context,
        primary_expression,
    );
    let involved_set: Vec<Rules> = [Rules::Postfix_expression].to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Postfix_expression,
        context,
        postfix_expression,
    );
    let closure_3 = _terminal(b'[');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _terminal(b']');
    let closure_8 = _sequence(&closure_6, &closure_7);
    let closure_9 = _subexpression(&closure_8);
    let closure_10 = _ordered_choice(&closure_1, &closure_9);
    let involved_set: Vec<Rules> = [Rules::Postfix_expression].to_vec();
    let closure_11 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Postfix_expression,
        context,
        postfix_expression,
    );
    let closure_12 = _terminal(b'(');
    let closure_13 = _sequence(&closure_11, &closure_12);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Argument_expression_list,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_14 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Argument_expression_list,
        context,
        argument_expression_list,
    );
    let closure_15 = _optional(&closure_14);
    let closure_16 = _sequence(&closure_13, &closure_15);
    let closure_17 = _terminal(b')');
    let closure_18 = _sequence(&closure_16, &closure_17);
    let closure_19 = _subexpression(&closure_18);
    let closure_20 = _ordered_choice(&closure_10, &closure_19);
    let involved_set: Vec<Rules> = [Rules::Postfix_expression].to_vec();
    let closure_21 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Postfix_expression,
        context,
        postfix_expression,
    );
    let closure_22 = _terminal(b'.');
    let closure_23 = _sequence(&closure_21, &closure_22);
    let closure_24 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_25 = _sequence(&closure_23, &closure_24);
    let closure_26 = _subexpression(&closure_25);
    let closure_27 = _ordered_choice(&closure_20, &closure_26);
    let involved_set: Vec<Rules> = [Rules::Postfix_expression].to_vec();
    let closure_28 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Postfix_expression,
        context,
        postfix_expression,
    );
    let closure_29 = _string_terminal_opt_ascii(b"->");
    let closure_30 = _sequence(&closure_28, &closure_29);
    let closure_31 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_32 = _sequence(&closure_30, &closure_31);
    let closure_33 = _subexpression(&closure_32);
    let closure_34 = _ordered_choice(&closure_27, &closure_33);
    let involved_set: Vec<Rules> = [Rules::Postfix_expression].to_vec();
    let closure_35 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Postfix_expression,
        context,
        postfix_expression,
    );
    let closure_36 = _string_terminal_opt_ascii(b"++");
    let closure_37 = _sequence(&closure_35, &closure_36);
    let closure_38 = _subexpression(&closure_37);
    let closure_39 = _ordered_choice(&closure_34, &closure_38);
    let involved_set: Vec<Rules> = [Rules::Postfix_expression].to_vec();
    let closure_40 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Postfix_expression,
        context,
        postfix_expression,
    );
    let closure_41 = _string_terminal_opt_ascii(b"--");
    let closure_42 = _sequence(&closure_40, &closure_41);
    let closure_43 = _subexpression(&closure_42);
    let closure_44 = _ordered_choice(&closure_39, &closure_43);
    let closure_45 = _terminal(b'(');
    let closure_46 = _var_name(user_state, Rules::Type_name, context, type_name);
    let closure_47 = _sequence(&closure_45, &closure_46);
    let closure_48 = _terminal(b')');
    let closure_49 = _sequence(&closure_47, &closure_48);
    let closure_50 = _terminal(b'{');
    let closure_51 = _sequence(&closure_49, &closure_50);
    let closure_52 = _var_name(
        user_state,
        Rules::Initializer_list,
        context,
        initializer_list,
    );
    let closure_53 = _sequence(&closure_51, &closure_52);
    let closure_54 = _terminal(b'}');
    let closure_55 = _sequence(&closure_53, &closure_54);
    let closure_56 = _subexpression(&closure_55);
    let closure_57 = _ordered_choice(&closure_44, &closure_56);
    let closure_58 = _terminal(b'(');
    let closure_59 = _var_name(user_state, Rules::Type_name, context, type_name);
    let closure_60 = _sequence(&closure_58, &closure_59);
    let closure_61 = _terminal(b')');
    let closure_62 = _sequence(&closure_60, &closure_61);
    let closure_63 = _terminal(b'{');
    let closure_64 = _sequence(&closure_62, &closure_63);
    let closure_65 = _var_name(
        user_state,
        Rules::Initializer_list,
        context,
        initializer_list,
    );
    let closure_66 = _sequence(&closure_64, &closure_65);
    let closure_67 = _terminal(b',');
    let closure_68 = _sequence(&closure_66, &closure_67);
    let closure_69 = _terminal(b'}');
    let closure_70 = _sequence(&closure_68, &closure_69);
    let closure_71 = _subexpression(&closure_70);
    let closure_72 = _ordered_choice(&closure_57, &closure_71);
    closure_72(parent, source, position)
}
#[allow(dead_code)]
pub fn argument_expression_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Argument_expression_list,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Argument_expression_list,
        context,
        argument_expression_list,
    );
    let closure_3 = _terminal(b',');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn unary_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Postfix_expression].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Postfix_expression,
        context,
        postfix_expression,
    );
    let closure_2 = _string_terminal_opt_ascii(b"++");
    let involved_set: Vec<Rules> = [Rules::Postfix_expression, Rules::Unary_expression].to_vec();
    let closure_3 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Unary_expression,
        context,
        unary_expression,
    );
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _ordered_choice(&closure_1, &closure_5);
    let closure_7 = _string_terminal_opt_ascii(b"--");
    let involved_set: Vec<Rules> = [Rules::Postfix_expression, Rules::Unary_expression].to_vec();
    let closure_8 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Unary_expression,
        context,
        unary_expression,
    );
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _ordered_choice(&closure_6, &closure_10);
    let closure_12 = _var_name(user_state, Rules::Unary_operator, context, unary_operator);
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_13 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Cast_expression,
        context,
        cast_expression,
    );
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _ordered_choice(&closure_11, &closure_15);
    let closure_17 = _string_terminal_opt_ascii(b"sizeof");
    let involved_set: Vec<Rules> = [Rules::Postfix_expression, Rules::Unary_expression].to_vec();
    let closure_18 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Unary_expression,
        context,
        unary_expression,
    );
    let closure_19 = _sequence(&closure_17, &closure_18);
    let closure_20 = _subexpression(&closure_19);
    let closure_21 = _ordered_choice(&closure_16, &closure_20);
    let closure_22 = _string_terminal_opt_ascii(b"sizeof");
    let closure_23 = _terminal(b'(');
    let closure_24 = _sequence(&closure_22, &closure_23);
    let closure_25 = _var_name(user_state, Rules::Type_name, context, type_name);
    let closure_26 = _sequence(&closure_24, &closure_25);
    let closure_27 = _terminal(b')');
    let closure_28 = _sequence(&closure_26, &closure_27);
    let closure_29 = _subexpression(&closure_28);
    let closure_30 = _ordered_choice(&closure_21, &closure_29);
    let closure_31 = _string_terminal_opt_ascii(b"_Alignof");
    let closure_32 = _terminal(b'(');
    let closure_33 = _sequence(&closure_31, &closure_32);
    let closure_34 = _var_name(user_state, Rules::Type_name, context, type_name);
    let closure_35 = _sequence(&closure_33, &closure_34);
    let closure_36 = _terminal(b')');
    let closure_37 = _sequence(&closure_35, &closure_36);
    let closure_38 = _subexpression(&closure_37);
    let closure_39 = _ordered_choice(&closure_30, &closure_38);
    closure_39(parent, source, position)
}
#[allow(dead_code)]
pub fn unary_operator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'&');
    let closure_2 = _terminal(b'*');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'+');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b'-');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _terminal(b'~');
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _terminal(b'!');
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    closure_11(parent, source, position)
}
#[allow(dead_code)]
pub fn cast_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Postfix_expression, Rules::Unary_expression].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Unary_expression,
        context,
        unary_expression,
    );
    let closure_2 = _terminal(b'(');
    let closure_3 = _var_name(user_state, Rules::Type_name, context, type_name);
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _terminal(b')');
    let closure_6 = _sequence(&closure_4, &closure_5);
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_7 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Cast_expression,
        context,
        cast_expression,
    );
    let closure_8 = _sequence(&closure_6, &closure_7);
    let closure_9 = _subexpression(&closure_8);
    let closure_10 = _ordered_choice(&closure_1, &closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn multiplicative_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Cast_expression,
        context,
        cast_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Multiplicative_expression,
        context,
        multiplicative_expression,
    );
    let closure_3 = _terminal(b'*');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Cast_expression,
        context,
        cast_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_9 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Multiplicative_expression,
        context,
        multiplicative_expression,
    );
    let closure_10 = _terminal(b'/');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_12 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Cast_expression,
        context,
        cast_expression,
    );
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_8, &closure_14);
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_16 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Multiplicative_expression,
        context,
        multiplicative_expression,
    );
    let closure_17 = _terminal(b'%');
    let closure_18 = _sequence(&closure_16, &closure_17);
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_19 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Cast_expression,
        context,
        cast_expression,
    );
    let closure_20 = _sequence(&closure_18, &closure_19);
    let closure_21 = _subexpression(&closure_20);
    let closure_22 = _ordered_choice(&closure_15, &closure_21);
    closure_22(parent, source, position)
}
#[allow(dead_code)]
pub fn additive_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Multiplicative_expression,
        context,
        multiplicative_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Additive_expression,
        context,
        additive_expression,
    );
    let closure_3 = _terminal(b'+');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Multiplicative_expression,
        context,
        multiplicative_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_9 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Additive_expression,
        context,
        additive_expression,
    );
    let closure_10 = _terminal(b'-');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let involved_set: Vec<Rules> = [
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_12 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Multiplicative_expression,
        context,
        multiplicative_expression,
    );
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_8, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn shift_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Additive_expression,
        context,
        additive_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Shift_expression,
        context,
        shift_expression,
    );
    let closure_3 = _string_terminal_opt_ascii(b"<<");
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Additive_expression,
        context,
        additive_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_9 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Shift_expression,
        context,
        shift_expression,
    );
    let closure_10 = _string_terminal_opt_ascii(b">>");
    let closure_11 = _sequence(&closure_9, &closure_10);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_12 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Additive_expression,
        context,
        additive_expression,
    );
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_8, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn relational_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Shift_expression,
        context,
        shift_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Relational_expression,
        context,
        relational_expression,
    );
    let closure_3 = _terminal(b'<');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Shift_expression,
        context,
        shift_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_9 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Relational_expression,
        context,
        relational_expression,
    );
    let closure_10 = _terminal(b'>');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_12 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Shift_expression,
        context,
        shift_expression,
    );
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_8, &closure_14);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_16 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Relational_expression,
        context,
        relational_expression,
    );
    let closure_17 = _string_terminal_opt_ascii(b"<=");
    let closure_18 = _sequence(&closure_16, &closure_17);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_19 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Shift_expression,
        context,
        shift_expression,
    );
    let closure_20 = _sequence(&closure_18, &closure_19);
    let closure_21 = _subexpression(&closure_20);
    let closure_22 = _ordered_choice(&closure_15, &closure_21);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_23 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Relational_expression,
        context,
        relational_expression,
    );
    let closure_24 = _string_terminal_opt_ascii(b">=");
    let closure_25 = _sequence(&closure_23, &closure_24);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_26 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Shift_expression,
        context,
        shift_expression,
    );
    let closure_27 = _sequence(&closure_25, &closure_26);
    let closure_28 = _subexpression(&closure_27);
    let closure_29 = _ordered_choice(&closure_22, &closure_28);
    closure_29(parent, source, position)
}
#[allow(dead_code)]
pub fn equality_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Relational_expression,
        context,
        relational_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Equality_expression,
        context,
        equality_expression,
    );
    let closure_3 = _string_terminal_opt_ascii(b"==");
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Relational_expression,
        context,
        relational_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_9 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Equality_expression,
        context,
        equality_expression,
    );
    let closure_10 = _string_terminal_opt_ascii(b"!=");
    let closure_11 = _sequence(&closure_9, &closure_10);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_12 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Relational_expression,
        context,
        relational_expression,
    );
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_8, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn and_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Equality_expression,
        context,
        equality_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::AND_expression,
        context,
        and_expression,
    );
    let closure_3 = _terminal(b'&');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Equality_expression,
        context,
        equality_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn exclusive_or_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::AND_expression,
        context,
        and_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Exclusive_OR_expression,
        context,
        exclusive_or_expression,
    );
    let closure_3 = _terminal(b'^');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::AND_expression,
        context,
        and_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn inclusive_or_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Exclusive_OR_expression,
        context,
        exclusive_or_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Inclusive_OR_expression,
        context,
        inclusive_or_expression,
    );
    let closure_3 = _terminal(b'|');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Exclusive_OR_expression,
        context,
        exclusive_or_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn logical_and_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Inclusive_OR_expression,
        context,
        inclusive_or_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Logical_AND_expression,
        context,
        logical_and_expression,
    );
    let closure_3 = _string_terminal_opt_ascii(b"&&");
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Inclusive_OR_expression,
        context,
        inclusive_or_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn logical_or_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Logical_AND_expression,
        context,
        logical_and_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Logical_OR_expression,
        context,
        logical_or_expression,
    );
    let closure_3 = _string_terminal_opt_ascii(b"||");
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Logical_AND_expression,
        context,
        logical_and_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn conditional_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Logical_OR_expression,
        context,
        logical_or_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Logical_OR_expression,
        context,
        logical_or_expression,
    );
    let closure_3 = _terminal(b'?');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _terminal(b':');
    let closure_8 = _sequence(&closure_6, &closure_7);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_9 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Conditional_expression,
        context,
        conditional_expression,
    );
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _subexpression(&closure_10);
    let closure_12 = _ordered_choice(&closure_1, &closure_11);
    closure_12(parent, source, position)
}
#[allow(dead_code)]
pub fn assignment_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Conditional_expression,
        context,
        conditional_expression,
    );
    let involved_set: Vec<Rules> = [Rules::Postfix_expression, Rules::Unary_expression].to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Unary_expression,
        context,
        unary_expression,
    );
    let closure_3 = _var_name(
        user_state,
        Rules::Assignment_operator,
        context,
        assignment_operator,
    );
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn assignment_operator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'=');
    let closure_2 = _string_terminal_opt_ascii(b"*=");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _string_terminal_opt_ascii(b"/=");
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _string_terminal_opt_ascii(b"%=");
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _string_terminal_opt_ascii(b"+=");
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _string_terminal_opt_ascii(b"-=");
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _string_terminal_opt_ascii(b"<<=");
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _string_terminal_opt_ascii(b">>=");
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    let closure_16 = _string_terminal_opt_ascii(b"&=");
    let closure_17 = _ordered_choice(&closure_15, &closure_16);
    let closure_18 = _string_terminal_opt_ascii(b"^=");
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _string_terminal_opt_ascii(b"|=");
    let closure_21 = _ordered_choice(&closure_19, &closure_20);
    closure_21(parent, source, position)
}
#[allow(dead_code)]
pub fn expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_3 = _terminal(b',');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn constant_expression<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Conditional_expression,
        context,
        conditional_expression,
    );
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn declaration<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(
        user_state,
        Rules::Declaration_specifiers,
        context,
        declaration_specifiers,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(user_state, Rules::Attribute_seq, context, attribute_seq);
    let closure_7 = _optional(&closure_6);
    let closure_8 = _sequence(&closure_5, &closure_7);
    let closure_9 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _var_name(
        user_state,
        Rules::Init_declarator_list,
        context,
        init_declarator_list,
    );
    let closure_12 = _optional(&closure_11);
    let closure_13 = _sequence(&closure_10, &closure_12);
    let closure_14 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _terminal(b';');
    let closure_17 = _sequence(&closure_15, &closure_16);
    let closure_18 = _subexpression(&closure_17);
    let closure_19 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_20 = _var_name(
        user_state,
        Rules::Static_assert_declaration,
        context,
        static_assert_declaration,
    );
    let closure_21 = _sequence(&closure_19, &closure_20);
    let closure_22 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_23 = _sequence(&closure_21, &closure_22);
    let closure_24 = _subexpression(&closure_23);
    let closure_25 = _ordered_choice(&closure_18, &closure_24);
    closure_25(parent, source, position)
}
#[allow(dead_code)]
pub fn declaration_specifiers<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(
        user_state,
        Rules::Storage_class_specifier,
        context,
        storage_class_specifier,
    );
    let closure_3 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _var_name(
        user_state,
        Rules::Declaration_specifiers,
        context,
        declaration_specifiers,
    );
    let closure_6 = _optional(&closure_5);
    let closure_7 = _sequence(&closure_4, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _var_name(user_state, Rules::Type_specifier, context, type_specifier);
    let closure_10 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _var_name(
        user_state,
        Rules::Declaration_specifiers,
        context,
        declaration_specifiers,
    );
    let closure_13 = _optional(&closure_12);
    let closure_14 = _sequence(&closure_11, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _ordered_choice(&closure_8, &closure_15);
    let closure_17 = _var_name(user_state, Rules::Type_qualifier, context, type_qualifier);
    let closure_18 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_19 = _sequence(&closure_17, &closure_18);
    let closure_20 = _var_name(
        user_state,
        Rules::Declaration_specifiers,
        context,
        declaration_specifiers,
    );
    let closure_21 = _optional(&closure_20);
    let closure_22 = _sequence(&closure_19, &closure_21);
    let closure_23 = _subexpression(&closure_22);
    let closure_24 = _ordered_choice(&closure_16, &closure_23);
    let closure_25 = _var_name(
        user_state,
        Rules::Function_specifier,
        context,
        function_specifier,
    );
    let closure_26 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_27 = _sequence(&closure_25, &closure_26);
    let closure_28 = _var_name(
        user_state,
        Rules::Declaration_specifiers,
        context,
        declaration_specifiers,
    );
    let closure_29 = _optional(&closure_28);
    let closure_30 = _sequence(&closure_27, &closure_29);
    let closure_31 = _subexpression(&closure_30);
    let closure_32 = _ordered_choice(&closure_24, &closure_31);
    let closure_33 = _var_name(
        user_state,
        Rules::Alignment_specifier,
        context,
        alignment_specifier,
    );
    let closure_34 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_35 = _sequence(&closure_33, &closure_34);
    let closure_36 = _var_name(
        user_state,
        Rules::Declaration_specifiers,
        context,
        declaration_specifiers,
    );
    let closure_37 = _optional(&closure_36);
    let closure_38 = _sequence(&closure_35, &closure_37);
    let closure_39 = _subexpression(&closure_38);
    let closure_40 = _ordered_choice(&closure_32, &closure_39);
    let closure_41 = _subexpression(&closure_40);
    let closure_42 = _sequence(&closure_1, &closure_41);
    let closure_43 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_44 = _sequence(&closure_42, &closure_43);
    let closure_45 = _subexpression(&closure_44);
    let closure_46 =
        declared_new_typedef(user_state, parent, context, source, position, &closure_45);
    closure_46(parent, source, position)
}
#[allow(dead_code)]
pub fn attribute_seq<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Attribute, context, attribute);
    let closure_2 = _var_name(user_state, Rules::Attribute, context, attribute);
    let closure_3 = _var_name(user_state, Rules::Attribute_seq, context, attribute_seq);
    let closure_4 = _optional(&closure_3);
    let closure_5 = _sequence(&closure_2, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _ordered_choice(&closure_1, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn attribute<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"__asm");
    let closure_2 = _string_terminal_opt_ascii(b"__based");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _string_terminal_opt_ascii(b"__cdecl");
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _string_terminal_opt_ascii(b"__clrcall");
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _string_terminal_opt_ascii(b"__fastcall");
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _string_terminal_opt_ascii(b"__inline");
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _string_terminal_opt_ascii(b"__stdcall");
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _string_terminal_opt_ascii(b"__thiscall");
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    let closure_16 = _string_terminal_opt_ascii(b"__vectorcall");
    let closure_17 = _ordered_choice(&closure_15, &closure_16);
    closure_17(parent, source, position)
}
#[allow(dead_code)]
pub fn init_declarator_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Init_declarator, context, init_declarator);
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_3 = _terminal(b',');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _var_name(user_state, Rules::Init_declarator, context, init_declarator);
    let closure_8 = _sequence(&closure_6, &closure_7);
    let closure_9 = _subexpression(&closure_8);
    let closure_10 = _zero_or_more(&closure_9);
    let closure_11 = _sequence(&closure_1, &closure_10);
    let closure_12 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_13 = _sequence(&closure_11, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn init_declarator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(user_state, Rules::Declarator, context, declarator);
    let closure_3 = _var_name(user_state, Rules::Declarator, context, declarator);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b'=');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_9 = _sequence(&closure_7, &closure_8);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Initializer,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_10 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Initializer,
        context,
        initializer,
    );
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _ordered_choice(&closure_2, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _sequence(&closure_1, &closure_14);
    let closure_16 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_17 = _sequence(&closure_15, &closure_16);
    closure_17(parent, source, position)
}
#[allow(dead_code)]
pub fn storage_class_specifier<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"auto");
    let closure_2 = _string_terminal_opt_ascii(b"extern");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _string_terminal_opt_ascii(b"register");
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _string_terminal_opt_ascii(b"static");
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _string_terminal_opt_ascii(b"_Thread_local");
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _string_terminal_opt_ascii(b"typedef");
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _string_terminal_opt_ascii(b"__declspec");
    let closure_13 = _terminal(b'(');
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _var_name(
        user_state,
        Rules::Extended_decl_modifier_seq,
        context,
        extended_decl_modifier_seq,
    );
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 = _terminal(b')');
    let closure_18 = _sequence(&closure_16, &closure_17);
    let closure_19 = _subexpression(&closure_18);
    let closure_20 = _ordered_choice(&closure_11, &closure_19);
    closure_20(parent, source, position)
}
#[allow(dead_code)]
pub fn extended_decl_modifier_seq<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Extended_decl_modifier,
        context,
        extended_decl_modifier,
    );
    let closure_2 = _var_name(
        user_state,
        Rules::Extended_decl_modifier_seq,
        context,
        extended_decl_modifier_seq,
    );
    let closure_3 = _var_name(
        user_state,
        Rules::Extended_decl_modifier,
        context,
        extended_decl_modifier,
    );
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _ordered_choice(&closure_1, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn extended_decl_modifier<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"thread");
    let closure_2 = _string_terminal_opt_ascii(b"naked");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _string_terminal_opt_ascii(b"dllimport");
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _string_terminal_opt_ascii(b"dllexport");
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn type_specifier<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"void");
    let closure_2 = _string_terminal_opt_ascii(b"char");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _string_terminal_opt_ascii(b"short");
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _string_terminal_opt_ascii(b"int");
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _string_terminal_opt_ascii(b"__int8");
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _string_terminal_opt_ascii(b"__int16");
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _string_terminal_opt_ascii(b"__int32");
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _string_terminal_opt_ascii(b"__int64");
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    let closure_16 = _string_terminal_opt_ascii(b"long");
    let closure_17 = _ordered_choice(&closure_15, &closure_16);
    let closure_18 = _string_terminal_opt_ascii(b"float");
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _string_terminal_opt_ascii(b"double");
    let closure_21 = _ordered_choice(&closure_19, &closure_20);
    let closure_22 = _string_terminal_opt_ascii(b"signed");
    let closure_23 = _ordered_choice(&closure_21, &closure_22);
    let closure_24 = _string_terminal_opt_ascii(b"unsigned");
    let closure_25 = _ordered_choice(&closure_23, &closure_24);
    let closure_26 = _string_terminal_opt_ascii(b"_Bool");
    let closure_27 = _ordered_choice(&closure_25, &closure_26);
    let closure_28 = _string_terminal_opt_ascii(b"_Complex");
    let closure_29 = _ordered_choice(&closure_27, &closure_28);
    let closure_30 = _var_name(
        user_state,
        Rules::Atomic_type_specifier,
        context,
        atomic_type_specifier,
    );
    let closure_31 = _ordered_choice(&closure_29, &closure_30);
    let closure_32 = _var_name(
        user_state,
        Rules::Struct_or_union_specifier,
        context,
        struct_or_union_specifier,
    );
    let closure_33 = _ordered_choice(&closure_31, &closure_32);
    let closure_34 = _var_name(user_state, Rules::Enum_specifier, context, enum_specifier);
    let closure_35 = _ordered_choice(&closure_33, &closure_34);
    let closure_36 = _var_name(user_state, Rules::Typedef_name, context, typedef_name);
    let closure_37 = _ordered_choice(&closure_35, &closure_36);
    closure_37(parent, source, position)
}
#[allow(dead_code)]
pub fn struct_or_union_specifier<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(user_state, Rules::Struct_or_union, context, struct_or_union);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_7 = _optional(&closure_6);
    let closure_8 = _sequence(&closure_5, &closure_7);
    let closure_9 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _terminal(b'{');
    let closure_12 = _sequence(&closure_10, &closure_11);
    let closure_13 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_14 = _sequence(&closure_12, &closure_13);
    let involved_set: Vec<Rules> = [Rules::Struct_declaration_list].to_vec();
    let closure_15 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Struct_declaration_list,
        context,
        struct_declaration_list,
    );
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_18 = _sequence(&closure_16, &closure_17);
    let closure_19 = _terminal(b'}');
    let closure_20 = _sequence(&closure_18, &closure_19);
    let closure_21 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_22 = _sequence(&closure_20, &closure_21);
    let closure_23 = _subexpression(&closure_22);
    let closure_24 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_25 = _var_name(user_state, Rules::Struct_or_union, context, struct_or_union);
    let closure_26 = _sequence(&closure_24, &closure_25);
    let closure_27 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_28 = _sequence(&closure_26, &closure_27);
    let closure_29 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_30 = _sequence(&closure_28, &closure_29);
    let closure_31 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_32 = _sequence(&closure_30, &closure_31);
    let closure_33 = _subexpression(&closure_32);
    let closure_34 = _ordered_choice(&closure_23, &closure_33);
    closure_34(parent, source, position)
}
#[allow(dead_code)]
pub fn struct_or_union<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"struct");
    let closure_2 = _string_terminal_opt_ascii(b"union");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn struct_declaration_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Struct_declaration_list].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Struct_declaration_list,
        context,
        struct_declaration_list,
    );
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(
        user_state,
        Rules::Struct_declaration,
        context,
        struct_declaration,
    );
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_10 = _var_name(
        user_state,
        Rules::Struct_declaration,
        context,
        struct_declaration,
    );
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_8, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn struct_declaration<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(
        user_state,
        Rules::Specifier_qualifier_list,
        context,
        specifier_qualifier_list,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let involved_set: Vec<Rules> = [Rules::Struct_declarator_list].to_vec();
    let closure_6 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Struct_declarator_list,
        context,
        struct_declarator_list,
    );
    let closure_7 = _optional(&closure_6);
    let closure_8 = _sequence(&closure_5, &closure_7);
    let closure_9 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _terminal(b';');
    let closure_12 = _sequence(&closure_10, &closure_11);
    let closure_13 = _subexpression(&closure_12);
    let closure_14 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_15 = _var_name(
        user_state,
        Rules::Static_assert_declaration,
        context,
        static_assert_declaration,
    );
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_18 = _sequence(&closure_16, &closure_17);
    let closure_19 = _subexpression(&closure_18);
    let closure_20 = _ordered_choice(&closure_13, &closure_19);
    closure_20(parent, source, position)
}
#[allow(dead_code)]
pub fn specifier_qualifier_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(user_state, Rules::Type_specifier, context, type_specifier);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(
        user_state,
        Rules::Specifier_qualifier_list,
        context,
        specifier_qualifier_list,
    );
    let closure_7 = _optional(&closure_6);
    let closure_8 = _sequence(&closure_5, &closure_7);
    let closure_9 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _subexpression(&closure_10);
    let closure_12 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_13 = _var_name(user_state, Rules::Type_qualifier, context, type_qualifier);
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 = _var_name(
        user_state,
        Rules::Specifier_qualifier_list,
        context,
        specifier_qualifier_list,
    );
    let closure_18 = _optional(&closure_17);
    let closure_19 = _sequence(&closure_16, &closure_18);
    let closure_20 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_21 = _sequence(&closure_19, &closure_20);
    let closure_22 = _subexpression(&closure_21);
    let closure_23 = _ordered_choice(&closure_11, &closure_22);
    let closure_24 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_25 = _var_name(
        user_state,
        Rules::Alignment_specifier,
        context,
        alignment_specifier,
    );
    let closure_26 = _sequence(&closure_24, &closure_25);
    let closure_27 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_28 = _sequence(&closure_26, &closure_27);
    let closure_29 = _var_name(
        user_state,
        Rules::Specifier_qualifier_list,
        context,
        specifier_qualifier_list,
    );
    let closure_30 = _optional(&closure_29);
    let closure_31 = _sequence(&closure_28, &closure_30);
    let closure_32 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_33 = _sequence(&closure_31, &closure_32);
    let closure_34 = _subexpression(&closure_33);
    let closure_35 = _ordered_choice(&closure_23, &closure_34);
    closure_35(parent, source, position)
}
#[allow(dead_code)]
pub fn struct_declarator_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Struct_declarator_list].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Struct_declarator_list,
        context,
        struct_declarator_list,
    );
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b',');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(
        user_state,
        Rules::Struct_declarator,
        context,
        struct_declarator,
    );
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_14 = _var_name(
        user_state,
        Rules::Struct_declarator,
        context,
        struct_declarator,
    );
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_17 = _sequence(&closure_15, &closure_16);
    let closure_18 = _subexpression(&closure_17);
    let closure_19 = _ordered_choice(&closure_12, &closure_18);
    closure_19(parent, source, position)
}
#[allow(dead_code)]
pub fn struct_declarator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Declarator, context, declarator);
    let closure_2 = _optional(&closure_1);
    let closure_3 = _terminal(b':');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Constant_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Constant_expression,
        context,
        constant_expression,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _var_name(user_state, Rules::Declarator, context, declarator);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    closure_9(parent, source, position)
}
#[allow(dead_code)]
pub fn enum_specifier<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _string_terminal_opt_ascii(b"enum");
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_7 = _optional(&closure_6);
    let closure_8 = _sequence(&closure_5, &closure_7);
    let closure_9 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _terminal(b'{');
    let closure_12 = _sequence(&closure_10, &closure_11);
    let closure_13 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _var_name(user_state, Rules::Enumerator_list, context, enumerator_list);
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_18 = _sequence(&closure_16, &closure_17);
    let closure_19 = _terminal(b'}');
    let closure_20 = _sequence(&closure_18, &closure_19);
    let closure_21 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_22 = _sequence(&closure_20, &closure_21);
    let closure_23 = _subexpression(&closure_22);
    let closure_24 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_25 = _string_terminal_opt_ascii(b"enum");
    let closure_26 = _sequence(&closure_24, &closure_25);
    let closure_27 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_28 = _sequence(&closure_26, &closure_27);
    let closure_29 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_30 = _optional(&closure_29);
    let closure_31 = _sequence(&closure_28, &closure_30);
    let closure_32 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_33 = _sequence(&closure_31, &closure_32);
    let closure_34 = _terminal(b'{');
    let closure_35 = _sequence(&closure_33, &closure_34);
    let closure_36 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_37 = _sequence(&closure_35, &closure_36);
    let closure_38 = _var_name(user_state, Rules::Enumerator_list, context, enumerator_list);
    let closure_39 = _sequence(&closure_37, &closure_38);
    let closure_40 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_41 = _sequence(&closure_39, &closure_40);
    let closure_42 = _terminal(b',');
    let closure_43 = _sequence(&closure_41, &closure_42);
    let closure_44 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_45 = _sequence(&closure_43, &closure_44);
    let closure_46 = _terminal(b'}');
    let closure_47 = _sequence(&closure_45, &closure_46);
    let closure_48 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_49 = _sequence(&closure_47, &closure_48);
    let closure_50 = _subexpression(&closure_49);
    let closure_51 = _ordered_choice(&closure_23, &closure_50);
    let closure_52 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_53 = _string_terminal_opt_ascii(b"enum");
    let closure_54 = _sequence(&closure_52, &closure_53);
    let closure_55 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_56 = _sequence(&closure_54, &closure_55);
    let closure_57 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_58 = _sequence(&closure_56, &closure_57);
    let closure_59 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_60 = _sequence(&closure_58, &closure_59);
    let closure_61 = _subexpression(&closure_60);
    let closure_62 = _ordered_choice(&closure_51, &closure_61);
    closure_62(parent, source, position)
}
#[allow(dead_code)]
pub fn enumerator_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(user_state, Rules::Enumerator, context, enumerator);
    let closure_3 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _zero_or_more(&closure_5);
    let closure_7 = _sequence(&closure_1, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn enumerator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(
        user_state,
        Rules::Enumeration_constant,
        context,
        enumeration_constant,
    );
    let closure_3 = _var_name(
        user_state,
        Rules::Enumeration_constant,
        context,
        enumeration_constant,
    );
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b'=');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_9 = _sequence(&closure_7, &closure_8);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Constant_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_10 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Constant_expression,
        context,
        constant_expression,
    );
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _ordered_choice(&closure_2, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _sequence(&closure_1, &closure_14);
    let closure_16 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_17 = _sequence(&closure_15, &closure_16);
    closure_17(parent, source, position)
}
#[allow(dead_code)]
pub fn atomic_type_specifier<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _string_terminal_opt_ascii(b"_Atomic");
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b'(');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name(user_state, Rules::Type_name, context, type_name);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _terminal(b')');
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_17 = _sequence(&closure_15, &closure_16);
    closure_17(parent, source, position)
}
#[allow(dead_code)]
pub fn type_qualifier<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"const");
    let closure_2 = _string_terminal_opt_ascii(b"restrict");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _string_terminal_opt_ascii(b"volatile");
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _string_terminal_opt_ascii(b"_Atomic");
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn function_specifier<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"inline");
    let closure_2 = _string_terminal_opt_ascii(b"_Noreturn");
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn alignment_specifier<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"_Alignas");
    let closure_2 = _terminal(b'(');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(user_state, Rules::Type_name, context, type_name);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b')');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _string_terminal_opt_ascii(b"_Alignas");
    let closure_10 = _terminal(b'(');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Constant_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_12 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Constant_expression,
        context,
        constant_expression,
    );
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _terminal(b')');
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _subexpression(&closure_15);
    let closure_17 = _ordered_choice(&closure_8, &closure_16);
    closure_17(parent, source, position)
}
#[allow(dead_code)]
pub fn declarator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(user_state, Rules::Pointer, context, pointer);
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let involved_set: Vec<Rules> = [Rules::Direct_declarator].to_vec();
    let closure_5 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_declarator,
        context,
        direct_declarator,
    );
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_8 = _sequence(&closure_6, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn direct_declarator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'(');
    let closure_2 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(user_state, Rules::Declarator, context, declarator);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _terminal(b')');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let involved_set: Vec<Rules> = [Rules::Direct_declarator].to_vec();
    let closure_11 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_declarator,
        context,
        direct_declarator,
    );
    let closure_12 = _terminal(b'[');
    let closure_13 = _sequence(&closure_11, &closure_12);
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_14 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_15 = _optional(&closure_14);
    let closure_16 = _sequence(&closure_13, &closure_15);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_17 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_18 = _optional(&closure_17);
    let closure_19 = _sequence(&closure_16, &closure_18);
    let closure_20 = _terminal(b']');
    let closure_21 = _sequence(&closure_19, &closure_20);
    let closure_22 = _subexpression(&closure_21);
    let closure_23 = _ordered_choice(&closure_10, &closure_22);
    let involved_set: Vec<Rules> = [Rules::Direct_declarator].to_vec();
    let closure_24 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_declarator,
        context,
        direct_declarator,
    );
    let closure_25 = _terminal(b'[');
    let closure_26 = _sequence(&closure_24, &closure_25);
    let closure_27 = _string_terminal_opt_ascii(b"static");
    let closure_28 = _sequence(&closure_26, &closure_27);
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_29 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_30 = _optional(&closure_29);
    let closure_31 = _sequence(&closure_28, &closure_30);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_32 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_33 = _sequence(&closure_31, &closure_32);
    let closure_34 = _terminal(b']');
    let closure_35 = _sequence(&closure_33, &closure_34);
    let closure_36 = _subexpression(&closure_35);
    let closure_37 = _ordered_choice(&closure_23, &closure_36);
    let involved_set: Vec<Rules> = [Rules::Direct_declarator].to_vec();
    let closure_38 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_declarator,
        context,
        direct_declarator,
    );
    let closure_39 = _terminal(b'[');
    let closure_40 = _sequence(&closure_38, &closure_39);
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_41 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_42 = _sequence(&closure_40, &closure_41);
    let closure_43 = _string_terminal_opt_ascii(b"static");
    let closure_44 = _sequence(&closure_42, &closure_43);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_45 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_46 = _sequence(&closure_44, &closure_45);
    let closure_47 = _terminal(b']');
    let closure_48 = _sequence(&closure_46, &closure_47);
    let closure_49 = _subexpression(&closure_48);
    let closure_50 = _ordered_choice(&closure_37, &closure_49);
    let involved_set: Vec<Rules> = [Rules::Direct_declarator].to_vec();
    let closure_51 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_declarator,
        context,
        direct_declarator,
    );
    let closure_52 = _terminal(b'[');
    let closure_53 = _sequence(&closure_51, &closure_52);
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_54 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_55 = _optional(&closure_54);
    let closure_56 = _sequence(&closure_53, &closure_55);
    let closure_57 = _terminal(b'*');
    let closure_58 = _sequence(&closure_56, &closure_57);
    let closure_59 = _terminal(b']');
    let closure_60 = _sequence(&closure_58, &closure_59);
    let closure_61 = _subexpression(&closure_60);
    let closure_62 = _ordered_choice(&closure_50, &closure_61);
    let involved_set: Vec<Rules> = [Rules::Direct_declarator].to_vec();
    let closure_63 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_declarator,
        context,
        direct_declarator,
    );
    let closure_64 = _terminal(b'(');
    let closure_65 = _sequence(&closure_63, &closure_64);
    let involved_set: Vec<Rules> = [Rules::Parameter_list, Rules::Parameter_type_list].to_vec();
    let closure_66 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Parameter_type_list,
        context,
        parameter_type_list,
    );
    let closure_67 = _sequence(&closure_65, &closure_66);
    let closure_68 = _terminal(b')');
    let closure_69 = _sequence(&closure_67, &closure_68);
    let closure_70 = _subexpression(&closure_69);
    let closure_71 = _ordered_choice(&closure_62, &closure_70);
    let involved_set: Vec<Rules> = [Rules::Direct_declarator].to_vec();
    let closure_72 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_declarator,
        context,
        direct_declarator,
    );
    let closure_73 = _terminal(b'(');
    let closure_74 = _sequence(&closure_72, &closure_73);
    let involved_set: Vec<Rules> = [Rules::Identifier_list].to_vec();
    let closure_75 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Identifier_list,
        context,
        identifier_list,
    );
    let closure_76 = _optional(&closure_75);
    let closure_77 = _sequence(&closure_74, &closure_76);
    let closure_78 = _terminal(b')');
    let closure_79 = _sequence(&closure_77, &closure_78);
    let closure_80 = _subexpression(&closure_79);
    let closure_81 = _ordered_choice(&closure_71, &closure_80);
    let closure_82 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_83 = _ordered_choice(&closure_81, &closure_82);
    closure_83(parent, source, position)
}
#[allow(dead_code)]
pub fn pointer<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'*');
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _terminal(b'*');
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_7 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_8 = _optional(&closure_7);
    let closure_9 = _sequence(&closure_6, &closure_8);
    let closure_10 = _var_name(user_state, Rules::Pointer, context, pointer);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _ordered_choice(&closure_5, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn type_qualifier_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_2 = _var_name(user_state, Rules::Type_qualifier, context, type_qualifier);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(user_state, Rules::Type_qualifier, context, type_qualifier);
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn parameter_type_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Parameter_list].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Parameter_list,
        context,
        parameter_list,
    );
    let involved_set: Vec<Rules> = [Rules::Parameter_list].to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Parameter_list,
        context,
        parameter_list,
    );
    let closure_3 = _terminal(b',');
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _string_terminal_opt_ascii(b"...");
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn parameter_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Parameter_list].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Parameter_list,
        context,
        parameter_list,
    );
    let closure_2 = _terminal(b',');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(
        user_state,
        Rules::Parameter_declaration,
        context,
        parameter_declaration,
    );
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name(
        user_state,
        Rules::Parameter_declaration,
        context,
        parameter_declaration,
    );
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn parameter_declaration<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(
        user_state,
        Rules::Declaration_specifiers,
        context,
        declaration_specifiers,
    );
    let closure_3 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _var_name(user_state, Rules::Declarator, context, declarator);
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _var_name(
        user_state,
        Rules::Declaration_specifiers,
        context,
        declaration_specifiers,
    );
    let closure_9 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _var_name(
        user_state,
        Rules::Abstract_declarator,
        context,
        abstract_declarator,
    );
    let closure_12 = _optional(&closure_11);
    let closure_13 = _sequence(&closure_10, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_7, &closure_14);
    let closure_16 = _subexpression(&closure_15);
    let closure_17 = _sequence(&closure_1, &closure_16);
    let closure_18 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_19 = _sequence(&closure_17, &closure_18);
    closure_19(parent, source, position)
}
#[allow(dead_code)]
pub fn identifier_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Identifier_list].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Identifier_list,
        context,
        identifier_list,
    );
    let closure_2 = _terminal(b',');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn type_name<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(
        user_state,
        Rules::Specifier_qualifier_list,
        context,
        specifier_qualifier_list,
    );
    let closure_2 = _var_name(
        user_state,
        Rules::Abstract_declarator,
        context,
        abstract_declarator,
    );
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    closure_4(parent, source, position)
}
#[allow(dead_code)]
pub fn abstract_declarator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Pointer, context, pointer);
    let closure_2 = _var_name(user_state, Rules::Pointer, context, pointer);
    let closure_3 = _optional(&closure_2);
    let involved_set: Vec<Rules> = [Rules::Direct_abstract_declarator].to_vec();
    let closure_4 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_abstract_declarator,
        context,
        direct_abstract_declarator,
    );
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _ordered_choice(&closure_1, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn direct_abstract_declarator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'(');
    let closure_2 = _var_name(
        user_state,
        Rules::Abstract_declarator,
        context,
        abstract_declarator,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b')');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let involved_set: Vec<Rules> = [Rules::Direct_abstract_declarator].to_vec();
    let closure_7 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_abstract_declarator,
        context,
        direct_abstract_declarator,
    );
    let closure_8 = _terminal(b'[');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_10 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_11 = _optional(&closure_10);
    let closure_12 = _sequence(&closure_9, &closure_11);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_13 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_14 = _optional(&closure_13);
    let closure_15 = _sequence(&closure_12, &closure_14);
    let closure_16 = _terminal(b']');
    let closure_17 = _sequence(&closure_15, &closure_16);
    let closure_18 = _subexpression(&closure_17);
    let closure_19 = _ordered_choice(&closure_6, &closure_18);
    let involved_set: Vec<Rules> = [Rules::Direct_abstract_declarator].to_vec();
    let closure_20 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_abstract_declarator,
        context,
        direct_abstract_declarator,
    );
    let closure_21 = _terminal(b'[');
    let closure_22 = _sequence(&closure_20, &closure_21);
    let closure_23 = _string_terminal_opt_ascii(b"static");
    let closure_24 = _sequence(&closure_22, &closure_23);
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_25 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_26 = _optional(&closure_25);
    let closure_27 = _sequence(&closure_24, &closure_26);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_28 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_29 = _sequence(&closure_27, &closure_28);
    let closure_30 = _terminal(b']');
    let closure_31 = _sequence(&closure_29, &closure_30);
    let closure_32 = _subexpression(&closure_31);
    let closure_33 = _ordered_choice(&closure_19, &closure_32);
    let involved_set: Vec<Rules> = [Rules::Direct_abstract_declarator].to_vec();
    let closure_34 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_abstract_declarator,
        context,
        direct_abstract_declarator,
    );
    let closure_35 = _terminal(b'[');
    let closure_36 = _sequence(&closure_34, &closure_35);
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_37 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_38 = _sequence(&closure_36, &closure_37);
    let closure_39 = _string_terminal_opt_ascii(b"static");
    let closure_40 = _sequence(&closure_38, &closure_39);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_41 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_42 = _sequence(&closure_40, &closure_41);
    let closure_43 = _terminal(b']');
    let closure_44 = _sequence(&closure_42, &closure_43);
    let closure_45 = _subexpression(&closure_44);
    let closure_46 = _ordered_choice(&closure_33, &closure_45);
    let involved_set: Vec<Rules> = [Rules::Direct_abstract_declarator].to_vec();
    let closure_47 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_abstract_declarator,
        context,
        direct_abstract_declarator,
    );
    let closure_48 = _terminal(b'[');
    let closure_49 = _sequence(&closure_47, &closure_48);
    let involved_set: Vec<Rules> = [Rules::Type_qualifier_list].to_vec();
    let closure_50 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Type_qualifier_list,
        context,
        type_qualifier_list,
    );
    let closure_51 = _optional(&closure_50);
    let closure_52 = _sequence(&closure_49, &closure_51);
    let closure_53 = _terminal(b'*');
    let closure_54 = _sequence(&closure_52, &closure_53);
    let closure_55 = _terminal(b']');
    let closure_56 = _sequence(&closure_54, &closure_55);
    let closure_57 = _subexpression(&closure_56);
    let closure_58 = _ordered_choice(&closure_46, &closure_57);
    let involved_set: Vec<Rules> = [Rules::Direct_abstract_declarator].to_vec();
    let closure_59 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Direct_abstract_declarator,
        context,
        direct_abstract_declarator,
    );
    let closure_60 = _optional(&closure_59);
    let closure_61 = _terminal(b'(');
    let closure_62 = _sequence(&closure_60, &closure_61);
    let involved_set: Vec<Rules> = [Rules::Parameter_list, Rules::Parameter_type_list].to_vec();
    let closure_63 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Parameter_type_list,
        context,
        parameter_type_list,
    );
    let closure_64 = _optional(&closure_63);
    let closure_65 = _sequence(&closure_62, &closure_64);
    let closure_66 = _terminal(b')');
    let closure_67 = _sequence(&closure_65, &closure_66);
    let closure_68 = _subexpression(&closure_67);
    let closure_69 = _ordered_choice(&closure_58, &closure_68);
    closure_69(parent, source, position)
}
#[allow(dead_code)]
pub fn typedef_name<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // This call gets hooked. It should only return true if there is a typedef of that name as C is context sensitive

    let closure_1 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_2 = _subexpression(&closure_1);
    let closure_3 = is_typedef(user_state, parent, context, source, position, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn initializer<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Assignment_expression,
        context,
        assignment_expression,
    );
    let closure_2 = _terminal(b'{');
    let closure_3 = _var_name(
        user_state,
        Rules::Initializer_list,
        context,
        initializer_list,
    );
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _terminal(b'}');
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _ordered_choice(&closure_1, &closure_7);
    let closure_9 = _terminal(b'{');
    let closure_10 = _var_name(
        user_state,
        Rules::Initializer_list,
        context,
        initializer_list,
    );
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _terminal(b',');
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _terminal(b'}');
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _subexpression(&closure_15);
    let closure_17 = _ordered_choice(&closure_8, &closure_16);
    closure_17(parent, source, position)
}
#[allow(dead_code)]
pub fn initializer_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Designation, context, designation);
    let closure_2 = _optional(&closure_1);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Initializer,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_3 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Initializer,
        context,
        initializer,
    );
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _var_name(
        user_state,
        Rules::Initializer_list,
        context,
        initializer_list,
    );
    let closure_7 = _terminal(b',');
    let closure_8 = _sequence(&closure_6, &closure_7);
    let closure_9 = _var_name(user_state, Rules::Designation, context, designation);
    let closure_10 = _optional(&closure_9);
    let closure_11 = _sequence(&closure_8, &closure_10);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Initializer,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_12 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Initializer,
        context,
        initializer,
    );
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_5, &closure_14);
    closure_15(parent, source, position)
}
#[allow(dead_code)]
pub fn designation<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Designator_list, context, designator_list);
    let closure_2 = _terminal(b'=');
    let closure_3 = _sequence(&closure_1, &closure_2);
    closure_3(parent, source, position)
}
#[allow(dead_code)]
pub fn designator_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Designator, context, designator);
    let closure_2 = _var_name(user_state, Rules::Designator_list, context, designator_list);
    let closure_3 = _var_name(user_state, Rules::Designator, context, designator);
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _ordered_choice(&closure_1, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn designator<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'[');
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Constant_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Constant_expression,
        context,
        constant_expression,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b']');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _terminal(b'.');
    let closure_8 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _ordered_choice(&closure_6, &closure_10);
    closure_11(parent, source, position)
}
#[allow(dead_code)]
pub fn static_assert_declaration<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"_Static_assert");
    let closure_2 = _terminal(b'(');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Constant_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_4 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Constant_expression,
        context,
        constant_expression,
    );
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b',');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(user_state, Rules::String_literal, context, string_literal);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b')');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _terminal(b';');
    let closure_13 = _sequence(&closure_11, &closure_12);
    closure_13(parent, source, position)
}
#[allow(dead_code)]
pub fn statement<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(
        user_state,
        Rules::Labeled_statement,
        context,
        labeled_statement,
    );
    let closure_3 = _var_name(
        user_state,
        Rules::Compound_statement,
        context,
        compound_statement,
    );
    let closure_4 = _ordered_choice(&closure_2, &closure_3);
    let closure_5 = _var_name(
        user_state,
        Rules::Expression_statement,
        context,
        expression_statement,
    );
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    let closure_7 = _var_name(
        user_state,
        Rules::Selection_statement,
        context,
        selection_statement,
    );
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    let closure_9 = _var_name(
        user_state,
        Rules::Iteration_statement,
        context,
        iteration_statement,
    );
    let closure_10 = _ordered_choice(&closure_8, &closure_9);
    let closure_11 = _var_name(user_state, Rules::Jump_statement, context, jump_statement);
    let closure_12 = _ordered_choice(&closure_10, &closure_11);
    let closure_13 = _var_name(
        user_state,
        Rules::Try_except_statement,
        context,
        try_except_statement,
    );
    let closure_14 = _ordered_choice(&closure_12, &closure_13);
    let closure_15 = _var_name(
        user_state,
        Rules::Try_finally_statement,
        context,
        try_finally_statement,
    );
    let closure_16 = _ordered_choice(&closure_14, &closure_15);
    let closure_17 = _subexpression(&closure_16);
    let closure_18 = _sequence(&closure_1, &closure_17);
    let closure_19 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_20 = _sequence(&closure_18, &closure_19);
    closure_20(parent, source, position)
}
#[allow(dead_code)]
pub fn jump_statement<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _string_terminal_opt_ascii(b"goto");
    let closure_3 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _terminal(b';');
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _string_terminal_opt_ascii(b"continue");
    let closure_9 = _terminal(b';');
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _subexpression(&closure_10);
    let closure_12 = _ordered_choice(&closure_7, &closure_11);
    let closure_13 = _string_terminal_opt_ascii(b"break");
    let closure_14 = _terminal(b';');
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _subexpression(&closure_15);
    let closure_17 = _ordered_choice(&closure_12, &closure_16);
    let closure_18 = _string_terminal_opt_ascii(b"return");
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_19 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_20 = _optional(&closure_19);
    let closure_21 = _sequence(&closure_18, &closure_20);
    let closure_22 = _terminal(b';');
    let closure_23 = _sequence(&closure_21, &closure_22);
    let closure_24 = _subexpression(&closure_23);
    let closure_25 = _ordered_choice(&closure_17, &closure_24);
    let closure_26 = _string_terminal_opt_ascii(b"__leave");
    let closure_27 = _terminal(b';');
    let closure_28 = _sequence(&closure_26, &closure_27);
    let closure_29 = _subexpression(&closure_28);
    let closure_30 = _ordered_choice(&closure_25, &closure_29);
    let closure_31 = _subexpression(&closure_30);
    let closure_32 = _sequence(&closure_1, &closure_31);
    let closure_33 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_34 = _sequence(&closure_32, &closure_33);
    closure_34(parent, source, position)
}
#[allow(dead_code)]
pub fn compound_statement<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _terminal(b'{');
    let closure_3 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _var_name(
        user_state,
        Rules::Declaration_list,
        context,
        declaration_list,
    );
    let closure_6 = _optional(&closure_5);
    let closure_7 = _sequence(&closure_4, &closure_6);
    let closure_8 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_9 = _sequence(&closure_7, &closure_8);
    let involved_set: Vec<Rules> = [Rules::Statement_list].to_vec();
    let closure_10 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Statement_list,
        context,
        statement_list,
    );
    let closure_11 = _optional(&closure_10);
    let closure_12 = _sequence(&closure_9, &closure_11);
    let closure_13 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _terminal(b'}');
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 = _subexpression(&closure_16);
    let closure_18 = _sequence(&closure_1, &closure_17);
    let closure_19 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_20 = _sequence(&closure_18, &closure_19);
    closure_20(parent, source, position)
}
#[allow(dead_code)]
pub fn declaration_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(user_state, Rules::Declaration, context, declaration);
    let closure_3 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _one_or_more(&closure_5);
    let closure_7 = _sequence(&closure_1, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn statement_list<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Statement_list].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Statement_list,
        context,
        statement_list,
    );
    let closure_2 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn expression_statement<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_2 = _optional(&closure_1);
    let closure_3 = _terminal(b';');
    let closure_4 = _sequence(&closure_2, &closure_3);
    closure_4(parent, source, position)
}
#[allow(dead_code)]
pub fn iteration_statement<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"while");
    let closure_2 = _terminal(b'(');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_4 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b')');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _string_terminal_opt_ascii(b"do");
    let closure_12 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _string_terminal_opt_ascii(b"while");
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _terminal(b'(');
    let closure_17 = _sequence(&closure_15, &closure_16);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_18 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_19 = _sequence(&closure_17, &closure_18);
    let closure_20 = _terminal(b')');
    let closure_21 = _sequence(&closure_19, &closure_20);
    let closure_22 = _terminal(b';');
    let closure_23 = _sequence(&closure_21, &closure_22);
    let closure_24 = _subexpression(&closure_23);
    let closure_25 = _ordered_choice(&closure_10, &closure_24);
    let closure_26 = _string_terminal_opt_ascii(b"for");
    let closure_27 = _terminal(b'(');
    let closure_28 = _sequence(&closure_26, &closure_27);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_29 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_30 = _optional(&closure_29);
    let closure_31 = _sequence(&closure_28, &closure_30);
    let closure_32 = _terminal(b';');
    let closure_33 = _sequence(&closure_31, &closure_32);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_34 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_35 = _optional(&closure_34);
    let closure_36 = _sequence(&closure_33, &closure_35);
    let closure_37 = _terminal(b';');
    let closure_38 = _sequence(&closure_36, &closure_37);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_39 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_40 = _optional(&closure_39);
    let closure_41 = _sequence(&closure_38, &closure_40);
    let closure_42 = _terminal(b')');
    let closure_43 = _sequence(&closure_41, &closure_42);
    let closure_44 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_45 = _sequence(&closure_43, &closure_44);
    let closure_46 = _subexpression(&closure_45);
    let closure_47 = _ordered_choice(&closure_25, &closure_46);
    closure_47(parent, source, position)
}
#[allow(dead_code)]
pub fn selection_statement<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"if");
    let closure_2 = _terminal(b'(');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_4 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b')');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _string_terminal_opt_ascii(b"if");
    let closure_12 = _terminal(b'(');
    let closure_13 = _sequence(&closure_11, &closure_12);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_14 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _terminal(b')');
    let closure_17 = _sequence(&closure_15, &closure_16);
    let closure_18 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_19 = _sequence(&closure_17, &closure_18);
    let closure_20 = _string_terminal_opt_ascii(b"else");
    let closure_21 = _sequence(&closure_19, &closure_20);
    let closure_22 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_23 = _sequence(&closure_21, &closure_22);
    let closure_24 = _subexpression(&closure_23);
    let closure_25 = _ordered_choice(&closure_10, &closure_24);
    let closure_26 = _string_terminal_opt_ascii(b"switch");
    let closure_27 = _terminal(b'(');
    let closure_28 = _sequence(&closure_26, &closure_27);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_29 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_30 = _sequence(&closure_28, &closure_29);
    let closure_31 = _terminal(b')');
    let closure_32 = _sequence(&closure_30, &closure_31);
    let closure_33 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_34 = _sequence(&closure_32, &closure_33);
    let closure_35 = _subexpression(&closure_34);
    let closure_36 = _ordered_choice(&closure_25, &closure_35);
    closure_36(parent, source, position)
}
#[allow(dead_code)]
pub fn labeled_statement<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(user_state, Rules::Identifier, context, identifier);
    let closure_2 = _terminal(b':');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _string_terminal_opt_ascii(b"case");
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Constant_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_8 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Constant_expression,
        context,
        constant_expression,
    );
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b':');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    let closure_15 = _ordered_choice(&closure_6, &closure_14);
    let closure_16 = _string_terminal_opt_ascii(b"default");
    let closure_17 = _terminal(b':');
    let closure_18 = _sequence(&closure_16, &closure_17);
    let closure_19 = _var_name(user_state, Rules::Statement, context, statement);
    let closure_20 = _sequence(&closure_18, &closure_19);
    let closure_21 = _subexpression(&closure_20);
    let closure_22 = _ordered_choice(&closure_15, &closure_21);
    closure_22(parent, source, position)
}
#[allow(dead_code)]
pub fn try_except_statement<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"__try");
    let closure_2 = _var_name(
        user_state,
        Rules::Compound_statement,
        context,
        compound_statement,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _string_terminal_opt_ascii(b"__except");
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b'(');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let involved_set: Vec<Rules> = [
        Rules::AND_expression,
        Rules::Additive_expression,
        Rules::Assignment_expression,
        Rules::Cast_expression,
        Rules::Conditional_expression,
        Rules::Equality_expression,
        Rules::Exclusive_OR_expression,
        Rules::Expression,
        Rules::Inclusive_OR_expression,
        Rules::Logical_AND_expression,
        Rules::Logical_OR_expression,
        Rules::Multiplicative_expression,
        Rules::Postfix_expression,
        Rules::Relational_expression,
        Rules::Shift_expression,
        Rules::Unary_expression,
    ]
    .to_vec();
    let closure_8 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Expression,
        context,
        expression,
    );
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b')');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _var_name(
        user_state,
        Rules::Compound_statement,
        context,
        compound_statement,
    );
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _subexpression(&closure_13);
    closure_14(parent, source, position)
}
#[allow(dead_code)]
pub fn try_finally_statement<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(b"__try");
    let closure_2 = _var_name(
        user_state,
        Rules::Compound_statement,
        context,
        compound_statement,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _string_terminal_opt_ascii(b"__finally");
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(
        user_state,
        Rules::Compound_statement,
        context,
        compound_statement,
    );
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn translation_unit<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set: Vec<Rules> = [Rules::Translation_unit].to_vec();
    let closure_1 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Translation_unit,
        context,
        translation_unit,
    );
    let closure_2 = _var_name(
        user_state,
        Rules::External_declaration,
        context,
        external_declaration,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(
        user_state,
        Rules::External_declaration,
        context,
        external_declaration,
    );
    let closure_6 = _ordered_choice(&closure_4, &closure_5);
    closure_6(parent, source, position)
}
#[allow(dead_code)]
pub fn external_declaration<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(
        user_state,
        Rules::Function_definition,
        context,
        function_definition,
    );
    let closure_3 = _var_name(user_state, Rules::Declaration, context, declaration);
    let closure_4 = _ordered_choice(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _sequence(&closure_1, &closure_5);
    let closure_7 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_8 = _sequence(&closure_6, &closure_7);
    closure_8(parent, source, position)
}
#[allow(dead_code)]
pub fn function_definition<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_2 = _var_name(
        user_state,
        Rules::Declaration_specifiers,
        context,
        declaration_specifiers,
    );
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _var_name(user_state, Rules::Declarator, context, declarator);
    let closure_8 = _sequence(&closure_6, &closure_7);
    let closure_9 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_10 = _sequence(&closure_8, &closure_9);
    let closure_11 = _var_name(
        user_state,
        Rules::Declaration_list,
        context,
        declaration_list,
    );
    let closure_12 = _optional(&closure_11);
    let closure_13 = _sequence(&closure_10, &closure_12);
    let closure_14 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _var_name(
        user_state,
        Rules::Compound_statement,
        context,
        compound_statement,
    );
    let closure_17 = _sequence(&closure_15, &closure_16);
    let closure_18 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_19 = _sequence(&closure_17, &closure_18);
    closure_19(parent, source, position)
}
#[allow(dead_code)]
pub fn grammar<T: Context>(
    user_state: &RefCell<UserState>,
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let involved_set: Vec<Rules> = [Rules::Translation_unit].to_vec();
    let closure_2 = _var_name_indirect_left_recursion(
        user_state,
        &involved_set,
        Rules::Translation_unit,
        context,
        translation_unit,
    );
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |parent: Key, source: &Source, position: u32| {
        ws(user_state, parent, context, source, position)
    };
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
