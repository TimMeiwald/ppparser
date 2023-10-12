use parser_core::Context;
use parser_core::Source;
use parser_core::_sequence;
use parser_core::_terminal;

pub fn apostrophe(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'"')(source, position)
}

pub fn left_angle_bracket(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'<')(source, position)
}

pub fn right_angle_bracket(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'>')(source, position)
}

pub fn left_bracket(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'(')(source, position)
}

pub fn right_bracket(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b')')(source, position)
}

pub fn assignment(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'=')(source, position)
}

pub fn end_rule(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b';')(source, position)
}

pub fn ampersand(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'&')(source, position)
}
pub fn exclamation_mark(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'!')(source, position)
}
pub fn plus(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'+')(source, position)
}
pub fn star(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'*')(source, position)
}
pub fn question_mark(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'?')(source, position)
}
pub fn comma(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b',')(source, position)
}
pub fn backslash(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    _terminal(b'/')(source, position)
}

pub fn epsilon(_context: &Context, source: &Source, position: u32) -> (bool, u32) {
    let t1 = _terminal(b'"');
    let t2 = _terminal(b'"');
    let s1 = _sequence(&t1, &t2);
    s1(source, position)
}
