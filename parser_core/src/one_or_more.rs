use crate::source::Source;

pub fn _one_or_more_kernel(source: &Source, position: u32, func: impl Fn(&Source, u32) -> (bool, u32)) -> (bool, u32)
{
    let mut temp_position = position;
    let (valid, position) = func(source, temp_position);
    if !valid{
        return (false, temp_position);
    }
    temp_position = position;
    loop {
        let (valid, position) = func(source, temp_position);
        if !valid {
            break;
        }
        temp_position = position;
    }
    (true, temp_position)
}

pub fn _one_or_more(func: &impl Fn(&Source, u32) -> (bool, u32)) -> impl Fn(&Source, u32) -> (bool, u32) + '_{
    move |source: &Source, position: u32| _one_or_more_kernel(source, position, func)
}

#[cfg(test)]
mod tests {
    use crate::source::Source;
    use crate::terminal::_terminal;
    use crate::one_or_more::{_one_or_more, _one_or_more_kernel};
    fn test_func(source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        return x(source, position);
    }
    #[test]
    fn test_one_or_more_kernel() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let x = _one_or_more_kernel(&s, 0, &test_func);
        assert_eq!(x, (true, 3));
    }
    #[test]
    fn test_one_or_more() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let func = _one_or_more(&test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 3));
    }
    #[test]
    fn test_one_or_more_no_elements() {
        let s = "".to_string();
        let s = Source::new(s);
        let func = _one_or_more(&test_func);
        let x = func(&s, 0);
        assert_eq!(x, (false, 0));
    }
    #[test]
    fn test_one_or_more_one_elements() {
        let s = "a".to_string();
        let s = Source::new(s);
        let func = _one_or_more(&test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }
    // #[test]
    // Will cause an infinite loop, short of littering Options throughout the code base at the cost of performance
    // there isn't much I can do about it. It can be prevented by simply prohibiting doing directly nested zero or mores/one or mores in the grammar
    // which isn't allowed in peg anyway. 
    // fn test_one_or_more_nested() {
    //     let s = "aaa".to_string();
    //     let s = Source::new(s);
    //     let func = _one_or_more(&test_func);
    //     let func = _one_or_more(&func);
    //     let x = func(&s, 0);
    //     assert_eq!(x, (true, 3));
    // }
}
