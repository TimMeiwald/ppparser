#[allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#[warn(unused_variables)] // Generated Code also, since everything passes stuff
use cache::*;
use parser_core::*;
use publisher::*;
use rules::Rules;

pub use parser_core::Context;
pub use parser_core::Source;
pub fn alphabet_upper<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // We all love commments
    let closure_1 = _ordered_choice_match_range(65, 90);
    closure_1(source, position)
}
pub fn alphabet_lower<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(97, 122);
    closure_1(source, position)
}
pub fn num<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    closure_1(source, position)
}
pub fn numnozero<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(49, 57);
    closure_1(source, position)
}
pub fn hexval<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    let closure_2 = _ordered_choice_match_range(65, 70);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    closure_3(source, position)
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
    let closure_1 = _var_name(Rules::Alphabet_Upper, context, alphabet_upper);
    let closure_2 = _var_name(Rules::Alphabet_Lower, context, alphabet_lower);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Num, context, num);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    let closure_6 = _var_name(Rules::Spaces, context, spaces);
    let closure_7 = _ordered_choice(&closure_5, &closure_6);
    let closure_8 = _subexpression(&closure_7);
    let closure_9 = _not_predicate(&closure_8);
    let closure_10 = _ordered_choice_match_range(0, 255);
    let closure_11 = _sequence(&closure_9, &closure_10);
    closure_11(source, position)
}
pub fn ascii<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(0, 255);
    closure_1(source, position)
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
pub fn left_angle_bracket<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'<');
    closure_1(source, position)
}
pub fn right_angle_bracket<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'>');
    closure_1(source, position)
}
pub fn left_bracket<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'(');
    closure_1(source, position)
}
pub fn right_bracket<T: Cache, S: Publisher>(
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
pub fn var_name<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Not whitespace dependent, feel free to use multiple lines for readability
    let closure_1 = _var_name(Rules::Left_Angle_Bracket, context, left_angle_bracket);
    let closure_2 = _var_name(Rules::Alphabet_Lower, context, alphabet_lower);
    let closure_3 = _var_name(Rules::Alphabet_Upper, context, alphabet_upper);
    let closure_4 = _ordered_choice(&closure_2, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    let closure_6 = _sequence(&closure_1, &closure_5);
    let closure_7 = _var_name(Rules::Alphabet_Lower, context, alphabet_lower);
    let closure_8 = _var_name(Rules::Alphabet_Upper, context, alphabet_upper);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _terminal(b'_');
    let closure_11 = _ordered_choice(&closure_9, &closure_10);
    let closure_12 = _subexpression(&closure_11);
    let closure_13 = _zero_or_more(&closure_12);
    let closure_14 = _sequence(&closure_6, &closure_13);
    let closure_15 = _var_name(Rules::Right_Angle_Bracket, context, right_angle_bracket);
    let closure_16 = _sequence(&closure_14, &closure_15);
    closure_16(source, position)
}
pub fn var_name_decl<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Var_Name, context, var_name);
    closure_1(source, position)
}
pub fn hex<T: Cache, S: Publisher>(
    context: &Context<T, S>,
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
    closure_6(source, position)
}
pub fn integer<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // No negative values since that is meaningless in this context
    let closure_1 = _var_name(Rules::NumNoZero, context, numnozero);
    let closure_2 = _var_name(Rules::Num, context, num);
    let closure_3 = _zero_or_more(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    closure_4(source, position)
}
pub fn orderedchoicematchrange<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_2 = _terminal(b'[');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = move |source: &Source, position: u32| whitespace(context, source, position);
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
    let closure_24 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_25 = _sequence(&closure_23, &closure_24);
    let closure_26 = _terminal(b']');
    let closure_27 = _sequence(&closure_25, &closure_26);
    let closure_28 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_29 = _sequence(&closure_27, &closure_28);
    closure_29(source, position)
}
pub fn subexpression<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Left_Bracket, context, left_bracket);
    let closure_2 = _var_name(Rules::RHS, context, rhs);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Right_Bracket, context, right_bracket);
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
    let closure_8 = _var_name(Rules::Var_Name, context, var_name);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_12 = _sequence(&closure_10, &closure_11);
    closure_12(source, position)
}
pub fn atom<T: Cache, S: Publisher>(
    context: &Context<T, S>,
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
    let closure_13 = move |source: &Source, position: u32| whitespace(context, source, position);
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
    let closure_1 = _var_name(Rules::Exclamation_Mark, context, exclamation_mark);
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
    let closure_2 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Comma, context, comma);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Atom, context, atom);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Comma, context, comma);
    let closure_11 = move |source: &Source, position: u32| whitespace(context, source, position);
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
    let closure_2 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Backslash, context, backslash);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::Atom, context, atom);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = _var_name(Rules::Backslash, context, backslash);
    let closure_11 = move |source: &Source, position: u32| whitespace(context, source, position);
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
    let closure_2 = move |source: &Source, position: u32| whitespace(context, source, position);
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
    let closure_2 = move |source: &Source, position: u32| whitespace(context, source, position);
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
    let closure_2 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Question_Mark, context, question_mark);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(source, position)
}
pub fn whitespace<T: Cache, S: Publisher>(
    context: &Context<T, S>,
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
    closure_9(source, position)
}
pub fn rhs<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Sequence, context, sequence);
    let closure_2 = _var_name(Rules::Ordered_Choice, context, ordered_choice);
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
    let closure_1 = _var_name(Rules::Var_Name_Decl, context, var_name_decl);
    let closure_2 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_3 = _var_name(Rules::Semantic_Instructions, context, semantic_instructions);
    let closure_4 = _sequence(&closure_2, &closure_3);
    let closure_5 = move |source: &Source, position: u32| whitespace(context, source, position);
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
    let closure_2 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::Assignment, context, assignment);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_7 = _sequence(&closure_5, &closure_6);
    let closure_8 = _var_name(Rules::RHS, context, rhs);
    let closure_9 = _sequence(&closure_7, &closure_8);
    let closure_10 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_11 = _sequence(&closure_9, &closure_10);
    let closure_12 = _var_name(Rules::End_Rule, context, end_rule);
    let closure_13 = _sequence(&closure_11, &closure_12);
    let closure_14 = move |source: &Source, position: u32| whitespace(context, source, position);
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
    let closure_3 = move |source: &Source, position: u32| whitespace(context, source, position);
    let closure_4 = _optional(&closure_3);
    let closure_5 = _sequence(&closure_2, &closure_4);
    closure_5(source, position)
}
pub fn comment<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = move |source: &Source, position: u32| whitespace(context, source, position);
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
    let closure_13 = move |source: &Source, position: u32| whitespace(context, source, position);
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
    let closure_4 = _var_name(Rules::Inline, context, inline);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(source, position)
}
pub fn delete<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(&[b'D', b'E', b'L', b'E', b'T', b'E']);
    closure_1(source, position)
}
pub fn passthrough<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _string_terminal_opt_ascii(&[
        b'P', b'A', b'S', b'S', b'T', b'H', b'R', b'O', b'U', b'G', b'H',
    ]);
    closure_1(source, position)
}
pub fn inline<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // Comment
    let closure_1 = _string_terminal_opt_ascii(&[b'I', b'n', b'l', b'i', b'n', b'e']);
    closure_1(source, position)
}
pub fn test_lr_num<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::Num, context, num);
    closure_1(source, position)
}
pub fn test_lr_expr<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    //  Should match 0-0-0-0-0-0-0-0 etc
    let closure_1 = _var_name(Rules::test_LR_expr, context, test_lr_expr);
    let closure_2 = _terminal(b'-');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::test_LR_num, context, test_lr_num);
    let closure_5 = _sequence(&closure_3, &closure_4);
    let closure_6 = _subexpression(&closure_5);
    let closure_7 = _var_name(Rules::test_LR_num, context, test_lr_num);
    let closure_8 = _ordered_choice(&closure_6, &closure_7);
    closure_8(source, position)
}
