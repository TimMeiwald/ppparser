use crate::source::Source;

pub fn _zero_or_more_kernel(source: &Source, position: u32, func: &dyn Fn(&Source, u32) -> (bool, u32)) -> (bool, u32)
{
    let mut temp_position = position;
    loop {
        let (valid, position) = func(source, temp_position);
        if !valid {
            break;
        }
        temp_position = position;
    }
    // Always true but may consume zero positions
    (true, temp_position)
}

pub fn _zero_or_more(func: &dyn Fn(&Source, u32) -> (bool, u32)) -> impl Fn(&Source, u32) -> (bool, u32) + '_{
    move |source: &Source, position: u32| _zero_or_more_kernel(source, position, func)
}

#[cfg(test)]
mod tests {
    use crate::source::Source;
    use crate::terminal::_terminal;
    use crate::zero_or_more::{_zero_or_more, _zero_or_more_kernel};
    fn test_func(source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        return x(source, position);
    }
    #[test]
    fn test_zero_or_more_kernel() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let x = _zero_or_more_kernel(&s, 0, &test_func);
        assert_eq!(x, (true, 3));
    }
    #[test]
    fn test_zero_or_more() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let func = _zero_or_more(&test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 3));
    }
    #[test]
    #[should_panic]
    fn test_zero_or_more_nested() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let func = _zero_or_more(&test_func);
        let func = _zero_or_more(&func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 3));
    }
}
