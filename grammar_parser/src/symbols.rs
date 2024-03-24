use cache::Cache;
use parser_core::Context;
use parser_core::Source;
use parser_core::_sequence;
use parser_core::_terminal;
use publisher::Publisher;

pub fn apostrophe<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'"')(source, position)
}

pub fn left_angle_bracket<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'<')(source, position)
}

pub fn right_angle_bracket<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'>')(source, position)
}

pub fn left_bracket<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'(')(source, position)
}

pub fn right_bracket<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b')')(source, position)
}

pub fn assignment<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'=')(source, position)
}

pub fn end_rule<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b';')(source, position)
}

pub fn ampersand<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'&')(source, position)
}
pub fn exclamation_mark<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'!')(source, position)
}
pub fn plus<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'+')(source, position)
}
pub fn star<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'*')(source, position)
}
pub fn question_mark<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'?')(source, position)
}
pub fn comma<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b',')(source, position)
}
pub fn backslash<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    _terminal(b'/')(source, position)
}

pub fn epsilon<T: Cache, S: Publisher>(
    _context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let t1 = _terminal(b'"');
    let t2 = _terminal(b'"');
    let s1 = _sequence(&t1, &t2);
    s1(source, position)
}
