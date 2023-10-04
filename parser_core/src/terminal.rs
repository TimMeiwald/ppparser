// Given a string, a position and a u8 check whether u8 at string[position] matches the u8
// u8 because ascii could be modified for utf-8 eventually. 
use crate::source::Source;
pub fn _terminal_kernel(source: &Source, position: u32, chr: u8) -> (bool, u32) {
    if source.get_char(position) == Some(chr){
        return (true, position+1)
    }
    else{
        return (false, position)
    }
}

pub fn _terminal(chr: u8) -> impl Fn(&Source, u32) -> (bool, u32) {
    return move |source: &Source, position: u32| _terminal_kernel(source, position, chr);
}

#[cfg(test)]
mod tests {
    use crate::terminal::{_terminal};
    use crate::source::Source;


    #[test]
    fn test_terminal_closure() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let c = _terminal("a".to_string().as_bytes()[0]);
        assert_eq!(c(&s, 0), (true, 1));
    }   






    // #[test]
    // fn test_terminal() {
    //     let s = "aaa".to_string();
    //     let s = Source::new(s);
    //     let mut s = Context{source:s, position:0};
    //     assert_eq!(_terminal(&mut s, "a".to_string().as_bytes()[0]), true);
    //     assert_eq!(s.position, 1);
    //     assert_eq!(_terminal(&mut s, "b".to_string().as_bytes()[0]), false);
    //     assert_eq!(s.position, 1);

    // }   

    // #[test]
    // fn test_terminal_end() {
    //     let s = "aaa".to_string();
    //     let s = Source::new(s);
    //     let mut s = Context{source:s, position:2};
    //     assert_eq!(_terminal(&mut s,"a".to_string().as_bytes()[0]), true);
    //     assert_eq!(s.position, 3);
    // }

    // #[test]
    // fn test_terminal_overrun() {
    //     let s = "aaa".to_string();
    //     let s = Source::new(s);
    //     let mut s = Context{source:s, position:3};
    //     assert_eq!(_terminal(&mut s, "a".to_string().as_bytes()[0]), false);
    //     assert_eq!(s.position, 3);

    // }
}