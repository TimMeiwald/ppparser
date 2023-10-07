use crate::source::Source;

pub fn _not_predicate_kernel(source: &Source, position: u32, func: impl Fn(&Source, u32) -> (bool, u32)) -> (bool, u32)
{
    let temp_position = position;
    let (valid, _position) = func(source, temp_position);
    if !valid{
        return (true, temp_position);
    }
    (false, temp_position)
}

pub fn _not_predicate(func: &dyn Fn(&Source, u32) -> (bool, u32)) -> impl Fn(&Source, u32) -> (bool, u32) + '_{
    move |source: &Source, position: u32| _not_predicate_kernel(source, position, func)
}

#[cfg(test)]
mod tests {
    use crate::source::Source;
    use crate::terminal::_terminal;
    use crate::not_predicate::{_not_predicate, _not_predicate_kernel};
    fn test_func(source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("a".as_bytes()[0]);
        return x(source, position);
    }
    #[test]
    fn test_not_predicate_kernel() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let x = _not_predicate_kernel(&s, 0, &test_func);
        assert_eq!(x, (false, 0));
    }
    #[test]
    fn test_not_predicate() {
        let s = "baa".to_string();
        let s = Source::new(s);
        let func = _not_predicate(&test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 0));
    }
    #[test]
    fn test_not_predicate_no_elements() {
        let s = "".to_string();
        let s = Source::new(s);
        let func = _not_predicate(&test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 0));
    }

    #[test]
    fn test_not_predicate_nested() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let func = _not_predicate(&test_func);
        let func = _not_predicate(&func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 0)); // Not Not True is True
    }
}
