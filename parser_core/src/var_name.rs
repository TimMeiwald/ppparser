use crate::source::Source;
use crate::context::Context;

pub fn _var_name_kernel(
    source: &Source,
    position: u32,
    context: &mut Context,
    func: fn(&Source, u32, &mut Context) -> (bool, u32),
) -> (bool, u32) {
    context.something_mutable.clear();
    func(source, position, context)
}
pub fn _var_name(func: fn(&Source, u32, &mut Context) -> (bool, u32)) -> impl Fn(&Source, u32, &mut Context) -> (bool, u32) {
    move |source: &Source, position: u32, context: &mut Context| _var_name_kernel(source, position, context, func)
}

#[cfg(test)]
mod tests {
    use super::_var_name;
    use crate::source::Source;
    use crate::context::{Context};
    use crate::terminal::_terminal;
    fn test_func(source: &Source, position: u32, context: &mut Context) -> (bool, u32) {
        let x = _terminal("a".to_string().as_bytes()[0]);
        x(source, position)
    }
    #[test]
    fn test_var_name() {
        let s = "aaa".to_string();
        let s = Source::new(s);
        let func = _var_name(test_func);
        let mut c = Context{something_mutable: "thing".to_string()};
        let x = func(&s, 0, &mut c);
        assert_eq!(x, (true, 1));
    }
}
