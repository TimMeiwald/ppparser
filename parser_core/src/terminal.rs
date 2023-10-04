// Given a string, a position and a u8 check whether u8 at string[position] matches the u8
// u8 because ascii could be modified for utf-8 eventually.
use crate::source::Source;
pub fn _terminal_kernel(source: &Source, position: u32, chr: u8) -> (bool, u32) {
    let char = source.get_char(position);
    //let char = char.expect("\nThis error only happens with invalid grammar. E.g **. This should be disallowed at generation time!!\n");
    if char == Some(chr) {
        return (true, position + 1);
    }
    else{
        return (false, position);
    }

}

pub fn _terminal(chr: u8) -> impl Fn(&Source, u32) -> (bool, u32) {
    move |source: &Source, position: u32| _terminal_kernel(source, position, chr)
}

#[cfg(test)]
mod tests {
    use crate::source::Source;
    use crate::terminal::_terminal;

    #[test]
    fn test_terminal_closure() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let c = _terminal("a".to_string().as_bytes()[0]);
        assert_eq!(c(&s, 0), (true, 1));
    }

    #[test]
    fn test_terminal_closure_fail() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let c = _terminal("a".to_string().as_bytes()[0]);
        assert_eq!(c(&s, 3), (false, 3));
    }
}
