use cache::*;
use parser_core::*;
use publisher::*;
use rules::Rules;

pub use parser_core::Context;
pub use parser_core::Source;

pub fn AlphabetUpper<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // We all love commments
    let closure_1 = _terminal(b'A');
    let closure_2 = _terminal(b'B');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'C');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b'D');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _terminal(b'E');
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _terminal(b'F');
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _terminal(b'G');
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _terminal(b'H');
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    let closure_16 = _terminal(b'I');
    let closure_17 = _ordered_choice(&closure_15, &closure_16);
    let closure_18 = _terminal(b'J');
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _terminal(b'K');
    let closure_21 = _ordered_choice(&closure_19, &closure_20);
    let closure_22 = _terminal(b'L');
    let closure_23 = _ordered_choice(&closure_21, &closure_22);
    let closure_24 = _terminal(b'M');
    let closure_25 = _ordered_choice(&closure_23, &closure_24);
    let closure_26 = _terminal(b'N');
    let closure_27 = _ordered_choice(&closure_25, &closure_26);
    let closure_28 = _terminal(b'O');
    let closure_29 = _ordered_choice(&closure_27, &closure_28);
    let closure_30 = _terminal(b'P');
    let closure_31 = _ordered_choice(&closure_29, &closure_30);
    let closure_32 = _terminal(b'Q');
    let closure_33 = _ordered_choice(&closure_31, &closure_32);
    let closure_34 = _terminal(b'R');
    let closure_35 = _ordered_choice(&closure_33, &closure_34);
    let closure_36 = _terminal(b'S');
    let closure_37 = _ordered_choice(&closure_35, &closure_36);
    let closure_38 = _terminal(b'T');
    let closure_39 = _ordered_choice(&closure_37, &closure_38);
    let closure_40 = _terminal(b'U');
    let closure_41 = _ordered_choice(&closure_39, &closure_40);
    let closure_42 = _terminal(b'V');
    let closure_43 = _ordered_choice(&closure_41, &closure_42);
    let closure_44 = _terminal(b'W');
    let closure_45 = _ordered_choice(&closure_43, &closure_44);
    let closure_46 = _terminal(b'X');
    let closure_47 = _ordered_choice(&closure_45, &closure_46);
    let closure_48 = _terminal(b'Y');
    let closure_49 = _ordered_choice(&closure_47, &closure_48);
    let closure_50 = _terminal(b'Z');
    let closure_51 = _ordered_choice(&closure_49, &closure_50);
    closure_51(source, position)
}
pub fn AlphabetLower<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'a');
    let closure_2 = _terminal(b'b');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'c');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b'd');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _terminal(b'e');
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _terminal(b'f');
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _terminal(b'g');
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _terminal(b'h');
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    let closure_16 = _terminal(b'i');
    let closure_17 = _ordered_choice(&closure_15, &closure_16);
    let closure_18 = _terminal(b'j');
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _terminal(b'k');
    let closure_21 = _ordered_choice(&closure_19, &closure_20);
    let closure_22 = _terminal(b'l');
    let closure_23 = _ordered_choice(&closure_21, &closure_22);
    let closure_24 = _terminal(b'm');
    let closure_25 = _ordered_choice(&closure_23, &closure_24);
    let closure_26 = _terminal(b'n');
    let closure_27 = _ordered_choice(&closure_25, &closure_26);
    let closure_28 = _terminal(b'o');
    let closure_29 = _ordered_choice(&closure_27, &closure_28);
    let closure_30 = _terminal(b'p');
    let closure_31 = _ordered_choice(&closure_29, &closure_30);
    let closure_32 = _terminal(b'q');
    let closure_33 = _ordered_choice(&closure_31, &closure_32);
    let closure_34 = _terminal(b'r');
    let closure_35 = _ordered_choice(&closure_33, &closure_34);
    let closure_36 = _terminal(b's');
    let closure_37 = _ordered_choice(&closure_35, &closure_36);
    let closure_38 = _terminal(b't');
    let closure_39 = _ordered_choice(&closure_37, &closure_38);
    let closure_40 = _terminal(b'u');
    let closure_41 = _ordered_choice(&closure_39, &closure_40);
    let closure_42 = _terminal(b'v');
    let closure_43 = _ordered_choice(&closure_41, &closure_42);
    let closure_44 = _terminal(b'w');
    let closure_45 = _ordered_choice(&closure_43, &closure_44);
    let closure_46 = _terminal(b'x');
    let closure_47 = _ordered_choice(&closure_45, &closure_46);
    let closure_48 = _terminal(b'y');
    let closure_49 = _ordered_choice(&closure_47, &closure_48);
    let closure_50 = _terminal(b'z');
    let closure_51 = _ordered_choice(&closure_49, &closure_50);
    closure_51(source, position)
}
pub fn num<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'0');
    let closure_2 = _terminal(b'1');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'2');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b'3');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _terminal(b'4');
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _terminal(b'5');
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _terminal(b'6');
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _terminal(b'7');
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    let closure_16 = _terminal(b'8');
    let closure_17 = _ordered_choice(&closure_15, &closure_16);
    let closure_18 = _terminal(b'9');
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    closure_19(source, position)
}
pub fn spaces<T: Cache, S: Publisher>(
    context: &Context<T, S>,
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
    closure_7(source, position)
}
pub fn specials<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'+');
    let closure_2 = _terminal(b'*');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'-');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b'&');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _terminal(b'!');
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _terminal(b'?');
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _terminal(b'<');
    let closure_13 = _ordered_choice(&closure_11, &closure_12);
    let closure_14 = _terminal(b'>');
    let closure_15 = _ordered_choice(&closure_13, &closure_14);
    let closure_16 = _terminal(b'"');
    let closure_17 = _ordered_choice(&closure_15, &closure_16);
    let closure_18 = _terminal(b'(');
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _terminal(b')');
    let closure_21 = _ordered_choice(&closure_19, &closure_20);
    let closure_22 = _terminal(b'_');
    let closure_23 = _ordered_choice(&closure_21, &closure_22);
    let closure_24 = _terminal(b',');
    let closure_25 = _ordered_choice(&closure_23, &closure_24);
    let closure_26 = _terminal(b'/');
    let closure_27 = _ordered_choice(&closure_25, &closure_26);
    let closure_28 = _terminal(b';');
    let closure_29 = _ordered_choice(&closure_27, &closure_28);
    let closure_30 = _terminal(b'=');
    let closure_31 = _ordered_choice(&closure_29, &closure_30);
    let closure_32 = _terminal(b'\\');
    let closure_33 = _ordered_choice(&closure_31, &closure_32);
    let closure_34 = _terminal(b'#');
    let closure_35 = _ordered_choice(&closure_33, &closure_34);
    let closure_36 = _terminal(b':');
    let closure_37 = _ordered_choice(&closure_35, &closure_36);
    let closure_38 = _terminal(b'|');
    let closure_39 = _ordered_choice(&closure_37, &closure_38);
    let closure_40 = _terminal(b'.');
    let closure_41 = _ordered_choice(&closure_39, &closure_40);
    let closure_42 = _terminal(b'{');
    let closure_43 = _ordered_choice(&closure_41, &closure_42);
    let closure_44 = _terminal(b'}');
    let closure_45 = _ordered_choice(&closure_43, &closure_44);
    let closure_46 = _terminal(b'[');
    let closure_47 = _ordered_choice(&closure_45, &closure_46);
    let closure_48 = _terminal(b']');
    let closure_49 = _ordered_choice(&closure_47, &closure_48);
    let closure_50 = _terminal(b'%');
    let closure_51 = _ordered_choice(&closure_49, &closure_50);
    let closure_52 = _terminal(b'\'');
    let closure_53 = _ordered_choice(&closure_51, &closure_52);
    let closure_54 = _terminal(b'^');
    let closure_55 = _ordered_choice(&closure_53, &closure_54);
    let closure_56 = _terminal(b'~');
    let closure_57 = _ordered_choice(&closure_55, &closure_56);
    let closure_58 = _terminal(b'@');
    let closure_59 = _ordered_choice(&closure_57, &closure_58);
    closure_59(source, position)
}
pub fn ascii<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::AlphabetLower, context, AlphabetLower);
    let closure_2 = _var_name(Rules::AlphabetUpper, context, AlphabetUpper);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Num, context, num);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::Spaces, context, spaces);
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Specials, context, specials);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    closure_9(source, position)
}
pub fn apostrophe<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'"');
    closure_1(source, position)
}
pub fn quotationmark<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\'');
    closure_1(source, position)
}
pub fn LeftAngleBracket<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'<');
    closure_1(source, position)
}
pub fn RightAngleBracket<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'>');
    closure_1(source, position)
}
pub fn LeftBracket<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'(');
    closure_1(source, position)
}
pub fn RightBracket<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b')');
    closure_1(source, position)
}
pub fn assignment<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'=');
    closure_1(source, position)
}
pub fn end_rule<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b';');
    closure_1(source, position)
}
pub fn ampersand<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'&');
    closure_1(source, position)
}
pub fn exclamation_mark<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'!');
    closure_1(source, position)
}
pub fn plus<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'+');
    closure_1(source, position)
}
pub fn star<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'*');
    closure_1(source, position)
}
pub fn question_mark<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'?');
    closure_1(source, position)
}
pub fn comma<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b',');
    closure_1(source, position)
}
pub fn newline<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'\n');
    closure_1(source, position)
}
pub fn backslash<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'/');
    closure_1(source, position)
}

pub fn hex<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Replace with custom code
    let char = source.get_char(position).unwrap();
    if char != "0".as_bytes()[0] {
        return (false, position);
    }
    let char = source.get_char(position + 1).unwrap();
    if char != "x".as_bytes()[0] {
        return (false, position);
    }
    let mut count = 2;
    let mut end_position: u32 = position + count;
    loop {
        let chr = source.get_char(position + count);
        match chr {
            None => break,
            Some(chr) => {
                if (chr >= 48 && chr <= 57) || (chr >= 65 && chr <= 70) || (chr >= 97 && chr <= 102)
                {
                    count += 1;
                    end_position = position + count;
                } else {
                    break;
                }
            }
        }
    }
    return (true, end_position);
}
pub fn integer<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    println!("Not implemented yet: integer");
    return (false, position);
}
pub fn orderedchoicematchrange<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_2 = _terminal(b'[');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Whitespace, context, whitespace);
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
    let closure_24 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_25 = _sequence(&closure_23, &closure_24);
    let closure_26 = _terminal(b']');
    let closure_27 = _sequence(&closure_25, &closure_26);
    let closure_28 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_29 = _sequence(&closure_27, &closure_28);
    closure_29(source, position)
}

pub fn var_name_decl<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::VarName, context, var_name);
    closure_1(source, position)
}

pub fn var_name<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Not whitespace dependent, feel free to use multiple lines for readability
    let closure_1 = _var_name(Rules::LeftAngleBracket, context, LeftAngleBracket);
    let closure_2 = _var_name(Rules::AlphabetLower, context, AlphabetLower);
    let closure_3 = _var_name(Rules::AlphabetUpper, context, AlphabetUpper);
    let closure_4 = _ordered_choice(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _sequence(&closure_1, &closure_5);
    let closure_7 = _var_name(Rules::AlphabetLower, context, AlphabetLower);
    let closure_8 = _var_name(Rules::AlphabetUpper, context, AlphabetUpper);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _terminal(b'_');
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _zero_or_more(&closure_12);
    let closure_14 = _sequence(&closure_6, &closure_13);
    let closure_15 = _var_name(Rules::RightAngleBracket, context, RightAngleBracket);
    let closure_16 = _sequence(&closure_14, &closure_15);
    closure_16(source, position)
}
pub fn subexpression<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::LeftBracket, context, LeftBracket);
    let closure_2 = _var_name(Rules::RHS, context, rhs);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::RightBracket, context, RightBracket);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(source, position)
}
pub fn epsilon<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::QuotationMark, context, quotationmark);
    let closure_2 = _var_name(Rules::QuotationMark, context, quotationmark);
    let closure_3 = _sequence(&closure_1, &closure_2);
    closure_3(source, position)
}
pub fn stringterminal<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Multibyte matches essentially
    let closure_1 = _var_name(Rules::Apostrophe, context, apostrophe);
    let closure_2 = _var_name(Rules::Apostrophe, context, apostrophe);
    let closure_3 = _not_predicate(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _var_name(Rules::ASCII, context, ascii);
    let closure_6 = _var_name(Rules::Apostrophe, context, apostrophe);
    let closure_7 = _not_predicate(&closure_6);
    let closure_8 = _var_name(Rules::ASCII, context, ascii);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _one_or_more(&closure_10);
    let closure_12 = _sequence(&closure_5, &closure_11);
    let closure_13 = _subexpression(&closure_12);
    let closure_14 = _sequence(&closure_4, &closure_13);
    let closure_15 = _var_name(Rules::Apostrophe, context, apostrophe);
    let closure_16 = _sequence(&closure_14, &closure_15);
    let closure_17 = _subexpression(&closure_16);
    let closure_18 = _var_name(Rules::Hex, context, hex);
    let closure_19 = _ordered_choice(&closure_17, &closure_18);
    let closure_20 = _var_name(Rules::Integer, context, integer);
    let closure_21 = _ordered_choice(&closure_19, &closure_20);
    closure_21(source, position)
}
pub fn terminal<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::QuotationMark, context, quotationmark);
    let closure_2 = _var_name(Rules::ASCII, context, ascii);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::QuotationMark, context, quotationmark);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name(Rules::QuotationMark, context, quotationmark);
    let closure_8 = _terminal(b'\\');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b'n');
    let closure_11 = _terminal(b'r');
    let closure_12 = _ordered_choice(&closure_10, &closure_11);
    let closure_13 = _terminal(b't');
    let closure_14 = _ordered_choice(&closure_12, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _sequence(&closure_9, &closure_15);
    let closure_17 = _var_name(Rules::QuotationMark, context, quotationmark);
    let closure_18 = _sequence(&closure_16, &closure_17);
    let closure_19 = _subexpression(&closure_18);
    let closure_20 = _ordered_choice(&closure_6, &closure_19);
    let closure_21 = _var_name(Rules::Epsilon, context, epsilon);
    let closure_22 = _ordered_choice(&closure_20, &closure_21);
    closure_22(source, position)
}
pub fn nucleus<T: Cache, S: Publisher>(
    context: &Context<T, S>,
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
    let closure_8 = _var_name(Rules::VarName, context, var_name);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_12 = _sequence(&closure_10, &closure_11);
    closure_12(source, position)
}
pub fn atom<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::AndPredicate, context, and_predicate);
    let closure_2 = _var_name(Rules::NotPredicate, context, not_predicate);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::OneOrMore, context, one_or_more);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::ZeroOrMore, context, zero_or_more);
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Optional, context, optional);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_14 = _sequence(&closure_12, &closure_13);
    closure_14(source, position)
}
pub fn and_predicate<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Ampersand, context, ampersand);
    let closure_2 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_3 = _sequence(&closure_1, &closure_2);
    closure_3(source, position)
}
pub fn not_predicate<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::ExclamationMark, context, exclamation_mark);
    let closure_2 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_3 = _sequence(&closure_1, &closure_2);
    closure_3(source, position)
}
pub fn sequence<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Atom, context, atom);
    let closure_2 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Comma, context, comma);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Atom, context, atom);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Comma, context, comma);
    let closure_11 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_12 = _sequence(&closure_10, &closure_11);
    let closure_13 = _var_name(Rules::Atom, context, atom);
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _zero_or_more(&closure_15);
    let closure_17 = _sequence(&closure_9, &closure_16);
    closure_17(source, position)
}
pub fn ordered_choice<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Atom, context, atom);
    let closure_2 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Backslash, context, backslash);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Atom, context, atom);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Backslash, context, backslash);
    let closure_11 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_12 = _sequence(&closure_10, &closure_11);
    let closure_13 = _var_name(Rules::Atom, context, atom);
    let closure_14 = _sequence(&closure_12, &closure_13);
    let closure_15 = _subexpression(&closure_14);
    let closure_16 = _zero_or_more(&closure_15);
    let closure_17 = _sequence(&closure_9, &closure_16);
    closure_17(source, position)
}
pub fn one_or_more<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_2 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Plus, context, plus);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(source, position)
}
pub fn zero_or_more<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_2 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Star, context, star);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(source, position)
}
pub fn optional<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Nucleus, context, nucleus);
    let closure_2 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::QuestionMark, context, question_mark);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(source, position)
}
pub fn whitespace<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b' ');
    let closure_2 = _var_name(Rules::Newline, context, newline);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _terminal(b'\r');
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _terminal(b'\t');
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _zero_or_more(&closure_8);
    closure_9(source, position)
}
pub fn rhs<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Sequence, context, sequence);
    let closure_2 = _var_name(Rules::OrderedChoice, context, ordered_choice);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Atom, context, atom);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(source, position)
}
pub fn lhs<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::VarNameDecl, context, var_name_decl);
    let closure_2 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_3 = _var_name(Rules::SemanticInstructions, context, semantic_instructions);
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 = _optional(&closure_7);
    let closure_9 = _sequence(&closure_1, &closure_8);
    closure_9(source, position)
}
pub fn rule<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::LHS, context, lhs);
    let closure_2 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Assignment, context, assignment);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::RHS, context, rhs);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _var_name(Rules::EndRule, context, end_rule);
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _var_name(Rules::Comment, context, comment);
    let closure_17 = _zero_or_more(&closure_16);
    let closure_18 = _sequence(&closure_15, &closure_17);
    closure_18(source, position)
}
pub fn grammar<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Double up dem comments
    let closure_1 = _var_name(Rules::Rule, context, rule);
    let closure_2 = _one_or_more(&closure_1);
    let closure_3 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_4 = _optional(&closure_3);
    let closure_5 = _sequence(&closure_2, &closure_4);
    closure_5(source, position)
}
pub fn comment<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Whitespace, context, whitespace);
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
    let closure_13 = _var_name(Rules::Whitespace, context, whitespace);
    let closure_14 = _sequence(&closure_12, &closure_13);
    closure_14(source, position)
}
pub fn semantic_instructions<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Delete, context, delete);
    let closure_2 = _var_name(Rules::Passthrough, context, passthrough);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Collect, context, collect);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(source, position)
}
pub fn delete<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'D');
    let closure_2 = _terminal(b'E');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b'L');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b'E');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _terminal(b'T');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b'E');
    let closure_11 = _sequence(&closure_9, &closure_10);
    closure_11(source, position)
}
pub fn passthrough<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'P');
    let closure_2 = _terminal(b'A');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b'S');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b'S');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _terminal(b'T');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b'H');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _terminal(b'R');
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = _terminal(b'O');
    let closure_15 = _sequence(&closure_13, &closure_14);
    let closure_16 = _terminal(b'U');
    let closure_17 = _sequence(&closure_15, &closure_16);
    let closure_18 = _terminal(b'G');
    let closure_19 = _sequence(&closure_17, &closure_18);
    let closure_20 = _terminal(b'H');
    let closure_21 = _sequence(&closure_19, &closure_20);
    closure_21(source, position)
}
pub fn collect<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Comment
    let closure_1 = _terminal(b'C');
    let closure_2 = _terminal(b'O');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _terminal(b'L');
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _terminal(b'L');
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _terminal(b'E');
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _terminal(b'C');
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _terminal(b'T');
    let closure_13 = _sequence(&closure_11, &closure_12);
    closure_13(source, position)
}
