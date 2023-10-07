use parser_core::_sequence;
use parser_core::_terminal;
use parser_core::Source;

pub fn apostrophe(source: &Source, position: u32) -> (bool, u32){
    _terminal('"' as u8)(source, position)
}

pub fn left_angle_bracket(source: &Source, position: u32) -> (bool, u32){
    _terminal('<' as u8)(source, position)
}

pub fn right_angle_bracket(source: &Source, position: u32) -> (bool, u32){
    _terminal('>' as u8)(source, position)
}

pub fn left_bracket(source: &Source, position: u32) -> (bool, u32){
    _terminal('(' as u8)(source, position)
}

pub fn right_bracket(source: &Source, position: u32) -> (bool, u32){
    _terminal(')' as u8)(source, position)
}

pub fn assignment(source: &Source, position: u32) -> (bool, u32){
    _terminal('=' as u8)(source, position)
}

pub fn end_rule(source: &Source, position: u32) -> (bool, u32){
    _terminal(';' as u8)(source, position)
}

pub fn ampersand(source: &Source, position: u32) -> (bool, u32){
    _terminal('&' as u8)(source, position)
}
pub fn exclamation_mark(source: &Source, position: u32) -> (bool, u32){
    _terminal('!' as u8)(source, position)
}
pub fn plus(source: &Source, position: u32) -> (bool, u32){
    _terminal('+' as u8)(source, position)
}
pub fn star(source: &Source, position: u32) -> (bool, u32){
    _terminal('*' as u8)(source, position)
}
pub fn question_mark(source: &Source, position: u32) -> (bool, u32){
    _terminal('?' as u8)(source, position)
}
pub fn comma(source: &Source, position: u32) -> (bool, u32){
    _terminal(',' as u8)(source, position)
}
pub fn backslash(source: &Source, position: u32) -> (bool, u32){
    _terminal('/' as u8)(source, position)
}

pub fn epsilon(source: &Source, position: u32) -> (bool, u32){
    let t1 = _terminal('"' as u8);
    let t2 = _terminal('"' as u8);
    let s1 = _sequence(&t1, &t2);
    s1(source, position)
}