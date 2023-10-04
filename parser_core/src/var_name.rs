use crate::source::Source;

pub fn _var_name_kernel(
    source: &Source,
    position: u32,
    func: fn(&Source, u32) -> (bool, u32),
) -> (bool, u32) {
    func(source, position)
}
pub fn _var_name(func: fn(&Source, u32) -> (bool, u32)) -> impl Fn(&Source, u32) -> (bool, u32) {
    move |source: &Source, position: u32| _var_name_kernel(source, position, func)
}

#[cfg(test)]
mod tests {
    use super::_var_name;
    use crate::source::Source;
    use crate::terminal::_terminal;
    fn test_func(source: &Source, position: u32) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        x(source, position)
    }
    #[test]
    fn test_var_name() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let func = _var_name(test_func);
        let x = func(&s, 0);
        assert_eq!(x, (true, 1));
    }
}
