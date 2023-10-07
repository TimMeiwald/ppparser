use crate::source::Source;

pub fn _optional_kernel(source: &Source, position: u32, func: impl Fn(&Source, u32) -> (bool, u32)) -> (bool, u32)
{
    let temp_position = position;
    let (valid, position) = func(source, temp_position);
    if !valid{
        return (true, temp_position);
    }
    (true, position)
}

pub fn _optional(func: &impl Fn(&Source, u32) -> (bool, u32)) -> impl Fn(&Source, u32) -> (bool, u32) + '_{
    move |source: &Source, position: u32| _optional_kernel(source, position, func)
}

#[cfg(test)]
mod tests {
    use crate::source::Source;
    use crate::terminal::_terminal;
    use crate::optional::{_optional, _optional_kernel};
    fn test_func(source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        return x(source, position);
    }
    #[test]
    fn test_optional_kernel() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let x = _optional_kernel(&s, 0, &test_func);
        assert_eq!(x, (true, 1));
    }
    #[test]
    fn test_optional() {
        let s = "baa".to_string();
        let s = Source::new(s);
        let func = _optional(&test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 0));
    }
    #[test]
    fn test_optional_no_elements() {
        let s = "".to_string();
        let s = Source::new(s);
        let func = _optional(&test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 0));
    }

    #[test]
    fn test_optional_nested() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let func = _optional(&test_func);
        let func = _optional(&func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }
}
