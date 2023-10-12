use crate::source::Source;

pub fn _sequence_kernel(
    source: &Source,
    position: u32,
    func_lhs: impl Fn(&Source, u32) -> (bool, u32),
    func_rhs: impl Fn(&Source, u32) -> (bool, u32),
) -> (bool, u32) {
    let temp_position = position;

    let (lhs_bool, position) = func_lhs(source, position);
    if lhs_bool {
        let (rhs_bool, position) = func_rhs(source, position);
        if rhs_bool {
            return (true, position);
        }
    }
    (false, temp_position)
}

pub fn _sequence<'a>(
    func_lhs: &'a impl Fn(&Source, u32) -> (bool, u32),
    func_rhs: &'a impl Fn(&Source, u32) -> (bool, u32),
) -> impl Fn(&Source, u32) -> (bool, u32) + 'a {
    move |source: &Source, position: u32| _sequence_kernel(source, position, func_lhs, func_rhs)
}

#[cfg(test)]
mod tests {
    use crate::sequence::{_sequence, _sequence_kernel};
    use crate::source::Source;
    use crate::terminal::_terminal;
    fn test_func1(source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        return x(source, position);
    }
    fn test_func2(source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("b".to_string().as_bytes()[0]);
        return x(source, position);
    }
    fn test_func3(source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("c".to_string().as_bytes()[0]);
        return x(source, position);
    }
    #[test]
    fn test_sequence_kernel_lhs() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let x = _sequence_kernel(&s, 0, &test_func1, &test_func1);
        assert_eq!(x, (true, 2));
    }
    #[test]
    fn test_sequence_kernel_rhs() {
        let s = "bbb".to_string();
        let s = Source::new(s);
        let x = _sequence_kernel(&s, 0, &test_func2, &test_func2);
        assert_eq!(x, (true, 2));
    }
    #[test]
    fn test_sequence_kernel_neither() {
        let s = "ccc".to_string();
        let s = Source::new(s);
        let x = _sequence_kernel(&s, 0, &test_func1, &test_func1);
        assert_eq!(x, (false, 0));
    }

    #[test]
    fn test_sequence_nested() {
        let s = "abc".to_string();
        let s = Source::new(s);
        let x = _sequence(&test_func1, &test_func2);
        let y = _sequence(&test_func3, &test_func2);
        let z = _sequence(&x, &y);
        assert_eq!(z(&s, 0), (false, 0));
    }
}
