use crate::source::Source;
use cache::AST;
pub fn _and_predicate_kernel(
    source: &Source,
    position: u32,
    func: impl Fn(&Source, u32) -> (bool, u32, AST),
) -> (bool, u32, AST) {
    let temp_position = position;
    let (valid, _position, ast) = func(source, temp_position);
    if !valid {
        return (false, temp_position, ast);
    }
    (true, temp_position, ast)
}

pub fn _and_predicate(
    func: &impl Fn(&Source, u32) -> (bool, u32, AST),
) -> impl Fn(&Source, u32) -> (bool, u32, AST) + '_ {
    move |source: &Source, position: u32| _and_predicate_kernel(source, position, func)
}

// #[cfg(test)]
// mod tests {
//     use crate::and_predicate::{_and_predicate, _and_predicate_kernel};
//     use crate::source::Source;
//     use crate::terminal::_terminal;
//     use cache::AST;
//     fn test_func(source: &Source, position: u32) -> (bool, u32, AST) {
//         let x = _terminal("a".to_string().as_bytes()[0]);
//         x(source, position)
//     }
//     #[test]
//     fn test_and_predicate_kernel() {
//         let s = "aaa".to_string();
//         let s = Source::new(&s);
//         let x = _and_predicate_kernel(&s, 0, test_func);
//         assert_eq!(x, (true, 0));
//     }
//     #[test]
//     fn test_and_predicate() {
//         let s = "baa".to_string();
//         let s = Source::new(&s);
//         let func = _and_predicate(&test_func);
//         let x = func(&s, 0);
//         assert_eq!(x, (false, 0));
//     }
//     #[test]
//     fn test_and_predicate_no_elements() {
//         let s = "".to_string();
//         let s = Source::new(&s);
//         let func = _and_predicate(&test_func);
//         let x = func(&s, 0);
//         assert_eq!(x, (false, 0));
//     }

//     #[test]
//     fn test_and_predicate_nested() {
//         let s = "aaa".to_string();
//         let s = Source::new(&s);
//         let func = _and_predicate(&test_func);
//         let func = _and_predicate(&func);
//         let x = func(&s, 0);
//         assert_eq!(x, (true, 0));
//     }
// }
