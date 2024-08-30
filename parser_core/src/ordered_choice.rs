use crate::source::Source;
use cache::AST;
use std::result;

pub fn _ordered_choice_kernel(
    source: &Source,
    position: u32,
    func_lhs: impl Fn(&Source, u32) -> (bool, u32, AST),
    func_rhs: impl Fn(&Source, u32) -> (bool, u32, AST),
) -> (bool, u32, AST) {
    let temp_position = position;
    let (valid, position, ast) = func_lhs(source, position);
    // println!("Func LHS: {:?}", (valid, position));
    if valid {
        // println!("Should end here");
        return (true, position, ast);
    }
    let position = temp_position;
    let (valid, position, ast) = func_rhs(source, position);
    if valid {
        // println!("Wtf why");
        (true, position, ast)
    } else {
        (false, temp_position, AST::FAIL)
    }
}

pub fn _ordered_choice<'a>(
    func_lhs: &'a impl Fn(&Source, u32) -> (bool, u32, AST),
    func_rhs: &'a impl Fn(&Source, u32) -> (bool, u32, AST),
) -> impl Fn(&Source, u32) -> (bool, u32, AST) + 'a {
    move |source: &Source, position: u32| {
        _ordered_choice_kernel(source, position, func_lhs, func_rhs)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::ordered_choice::{_ordered_choice, _ordered_choice_kernel};
//     use crate::source::Source;
//     use crate::terminal::_terminal;
//     fn test_func1(source: &Source, position: u32) -> (bool, u32, AST) {
//         let x = _terminal("a".to_string().as_bytes()[0]);
//         x(source, position)
//     }
//     fn test_func2(source: &Source, position: u32) -> (bool, u32, AST) {
//         let x = _terminal("b".to_string().as_bytes()[0]);
//         x(source, position)
//     }
//     fn test_func3(source: &Source, position: u32) -> (bool, u32, AST) {
//         let x = _terminal("c".to_string().as_bytes()[0]);
//         x(source, position)
//     }
//     #[test]
//     fn test_ordered_choice_kernel_lhs() {
//         let s = "aaa".to_string();
//         let s = Source::new(&s);
//         let x = _ordered_choice_kernel(&s, 0, test_func1, test_func2);
//         assert_eq!(x, (true, 1));
//     }
//     #[test]
//     fn test_ordered_choice_kernel_rhs() {
//         let s = "bbb".to_string();
//         let s = Source::new(&s);
//         let x = _ordered_choice_kernel(&s, 0, test_func1, test_func2);
//         assert_eq!(x, (true, 1));
//     }
//     #[test]
//     fn test_ordered_choice_kernel_neither() {
//         let s = "ccc".to_string();
//         let s = Source::new(&s);
//         let x = _ordered_choice_kernel(&s, 0, test_func1, test_func2);
//         assert_eq!(x, (false, 0));
//     }

//     #[test]
//     fn test_ordered_choice_nested() {
//         let s = "ccc".to_string();
//         let s = Source::new(&s);
//         let x = _ordered_choice(&test_func1, &test_func2);
//         let y = _ordered_choice(&test_func3, &test_func2);
//         let z = _ordered_choice(&x, &y);
//         assert_eq!(z(&s, 0), (true, 1));
//     }
// }
