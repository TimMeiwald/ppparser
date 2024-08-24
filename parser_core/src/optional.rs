use crate::source::Source;
use cache::AST;
pub fn _optional_kernel(
    source: &Source,
    position: u32,
    func: impl Fn(&Source, u32) -> (bool, u32, AST),
) -> (bool, u32, AST) {
    let temp_position = position;
    let (valid, position, ast) = func(source, temp_position);
    if !valid {
        return (true, temp_position, AST::IGNORE);
    }
    (true, position, AST::IGNORE)
}

pub fn _optional(
    func: &impl Fn(&Source, u32) -> (bool, u32, AST),
) -> impl Fn(&Source, u32) -> (bool, u32, AST) + '_ {
    move |source: &Source, position: u32| _optional_kernel(source, position, func)
}

// #[cfg(test)]
// mod tests {
//     use crate::optional::{_optional, _optional_kernel};
//     use crate::source::Source;
//     use crate::terminal::_terminal;
//     fn test_func(source: &Source, position: u32) -> (bool, u32, AST) {
//         let x = _terminal("a".to_string().as_bytes()[0]);
//         x(source, position)
//     }
//     #[test]
//     fn test_optional_kernel() {
//         let s = "aaa".to_string();
//         let s = Source::new(&s);
//         let x = _optional_kernel(&s, 0, test_func);
//         assert_eq!(x, (true, 1));
//     }
//     #[test]
//     fn test_optional() {
//         let s = "baa".to_string();
//         let s = Source::new(&s);
//         let func = _optional(&test_func);
//         let x = func(&s, 0);
//         assert_eq!(x, (true, 0));
//     }
//     #[test]
//     fn test_optional_no_elements() {
//         let s = "".to_string();
//         let s = Source::new(&s);
//         let func = _optional(&test_func);
//         let x = func(&s, 0);
//         assert_eq!(x, (true, 0));
//     }

//     #[test]
//     fn test_optional_nested() {
//         let s = "aaa".to_string();
//         let s = Source::new(&s);
//         let func = _optional(&test_func);
//         let func = _optional(&func);
//         let x = func(&s, 0);
//         assert_eq!(x, (true, 1));
//     }
// }
