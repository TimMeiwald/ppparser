use crate::source::Source;

pub fn _subexpression_kernel(source: &Source, position: u32, func: &dyn Fn(&Source, u32) -> (bool, u32)) -> (bool, u32)
{
    let temp_position = position;
    let (valid, position) = func(source, temp_position);
    if !valid{
        return (false, temp_position);
    }
    (true, position)
}

pub fn _subexpression(func: &dyn Fn(&Source, u32) -> (bool, u32)) -> impl Fn(&Source, u32) -> (bool, u32) + '_{
    move |source: &Source, position: u32| _subexpression_kernel(source, position, func)
}

#[cfg(test)]
mod tests {
    use crate::source::Source;
    use crate::terminal::_terminal;
    use crate::subexpression::{_subexpression, _subexpression_kernel};
    fn test_func(source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        return x(source, position);
    }
    #[test]
    fn test_subexpression_kernel() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let x = _subexpression_kernel(&s, 0, &test_func);
        assert_eq!(x, (true, 1));
    }
    #[test]
    fn test_subexpression() {
        let s = "baa".to_string();
        let s = Source::new(s);
        let func = _subexpression(&test_func);
        let x = func(&s, 0);
        assert_eq!(x, (false, 0));
    }
    #[test]
    fn test_subexpression_no_elements() {
        let s = "".to_string();
        let s = Source::new(s);
        let func = _subexpression(&test_func);
        let x = func(&s, 0);
        assert_eq!(x, (false, 0));
    }

    #[test]
    fn test_subexpression_nested() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let func = _subexpression(&test_func);
        let func = _subexpression(&func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }
}
