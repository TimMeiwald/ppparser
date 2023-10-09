use parser_core::_sequence;
use parser_core::_terminal;
use parser_core::Source;
use parser_core::{Context, Rules};

pub fn apostrophe(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('"' as u8)(source, position)
}

pub fn left_angle_bracket(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('<' as u8)(source, position)
}

pub fn right_angle_bracket(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('>' as u8)(source, position)
}

pub fn left_bracket(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('(' as u8)(source, position)
}

pub fn right_bracket(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal(')' as u8)(source, position)
}

pub fn assignment(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('=' as u8)(source, position)
}

pub fn end_rule(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal(';' as u8)(source, position)
}

pub fn ampersand(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('&' as u8)(source, position)
}
pub fn exclamation_mark(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('!' as u8)(source, position)
}
pub fn plus(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('+' as u8)(source, position)
}
pub fn star(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('*' as u8)(source, position)
}
pub fn question_mark(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('?' as u8)(source, position)
}
pub fn comma(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal(',' as u8)(source, position)
}
pub fn backslash(context: &Context,source: &Source, position: u32) -> (bool, u32){
    _terminal('/' as u8)(source, position)
}

pub fn epsilon(context: &Context,source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal('"' as u8);
    let t2 = _terminal('"' as u8);
    let s1 = _sequence(&t1, &t2);
    s1(source, position)
}