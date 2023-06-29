mod and_predicate;
mod not_predicate;
mod one_or_more;
mod optional;
mod ordered_choice;
mod sequence;
mod subexpression;
mod terminal;
mod cache;
pub mod utils;
mod var_name;
mod zero_or_more;
pub mod parser;
mod generated_parser_core;
use std::path::Path;

pub use crate::and_predicate::_AndPredicate;
pub use crate::not_predicate::_NotPredicate;
pub use crate::one_or_more::_OneOrMore;
pub use crate::optional::_Optional;
pub use crate::ordered_choice::_OrderedChoice;
pub use crate::sequence::_Sequence;
pub use crate::terminal::Resolvable;
pub use crate::terminal::_Terminal;
pub use crate::var_name::_VarName;
pub use crate::zero_or_more::_ZeroOrMore;
use std::fs;


pub fn parse(grammar_filepath: &Path) -> (bool, u32, usize){
    let source = fs::read_to_string(grammar_filepath).unwrap_or("There is no grammar filepath!".to_string());
    let size_of_source = source.len(); // For Test purposes but yknow prolly should do that differently, User API is still up in the air a bit
    let (bool, position) = parser::Grammar.resolve(0, &source);
    return (bool, position, size_of_source);
}


/* The following is not an integration test. It's an example of the code that need's to be generated for each Rule so that it works with the rest of the codebase. */
#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Copy, Clone)] // Needed because I copy around position which as a u32 is likely faster than passing a reference, Need's profiling.
    struct Example; // Top level functions don't require T since they're defined via the primitives used in a given function.
    #[derive(Copy, Clone)]
    struct Example2;

    impl Resolvable for Example {
        fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
            return example1(position, source); // Define which function to run using impl Resolvable
        }
    }

    impl Resolvable for Example2 {
        fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
            return example2(position, source);
        }
    }

    fn example1(position: u32, source: &str) -> (bool, u32) {
        let t1 = Example2;
        let t2 = _Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let t3 = _Sequence {
            arg_lhs: t1,
            arg_rhs: t2,
        };
        let s = t3.resolve(position, source); // Each Top Level Rule can still call other Top Level Rules as well as primitives.
        return s;
    }

    fn example2(position: u32, source: &str) -> (bool, u32) {
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let s = t.resolve(position, source);
        return s;
    }

    fn example3(position: u32, source: &str) -> (bool, u32) {
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let b = _Sequence {
            arg_lhs: _Sequence {
                arg_lhs: t,
                arg_rhs: t,
            },
            arg_rhs: t,
        };
        let s = b.resolve(position, source);
        return s;
    }

    #[test]
    fn test_example_3() {
        let a = example3(0, "Hello World!");
        assert_eq!(a.0, false);
    }

    #[test]
    fn test_example_true() {
        let source = "Hello World";
        let position: u32 = 0;
        let s = example1(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 2);
    }

    #[test]
    fn test_example_false() {
        let source = "Hfllo World";
        let position: u32 = 0;
        let s = example1(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_example_false2() {
        let source = "fello World";
        let position: u32 = 0;
        let s = example1(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }
}
