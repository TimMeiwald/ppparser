mod and_predicate;
pub mod cache;
mod not_predicate;
mod one_or_more;
mod optional;
mod ordered_choice;
pub mod parser;
mod sequence;
mod subexpression;
mod terminal;
pub mod output_stack;
pub mod utils;
mod var_name;
mod zero_or_more;
//mod generated_parser_core;
use std::path::Path;

pub use crate::and_predicate::_AndPredicate;
use crate::cache::Cache;
pub use crate::not_predicate::_NotPredicate;
pub use crate::one_or_more::_OneOrMore;
pub use crate::optional::_Optional;
pub use crate::ordered_choice::_OrderedChoice;
pub use crate::sequence::_Sequence;
pub use crate::terminal::Resolvable;
pub use crate::terminal::_Terminal;
pub use crate::var_name::_VarName;
pub use crate::zero_or_more::_ZeroOrMore;
use crate::output_stack::Stack;
use std::fs;

pub fn parse(grammar_filepath: &Path) -> (bool, i32, usize, String, Stack) {
    let source =
        fs::read_to_string(grammar_filepath).unwrap_or("There is no grammar filepath!".to_string());
    let size_of_source = source.len(); // For Test purposes but yknow prolly should do that differently, User API is still up in the air a bit
    let mut cache = Cache::new(size_of_source as i32 + 1, 46); // Will break for anything with more than 100 chars or 100 rules
    let mut stack = Stack::new(size_of_source as i32,46);

    let (bool, position) = parser::Grammar.resolve(&mut stack, &mut cache, 0, &source);

    if !bool{
        println!("FAILED\n\nFAILED")
    }
    return (bool, position, size_of_source, source, stack);
}

pub fn parse_string(source: String, arg: &dyn Resolvable) -> (bool, i32) {
    let size_of_source = source.len();
    let mut cache = Cache::new(size_of_source as i32 + 1, 46); // Will break for anything with more than 100 chars or 100 rules
    let mut stack = Stack::new(size_of_source as i32,46);
    let (bool, position) = arg.resolve(&mut stack, &mut cache, 0, &source);
    return (bool, position);
}


/* The following is not an integration test. It's an example of the code that need's to be generated for each Rule so that it works with the rest of the codebase. */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::Cache;
    #[derive(Copy, Clone)] // Needed because I copy around position which as a i32 is likely faster than passing a reference, Need's profiling.
    struct Example; // Top level functions don't require T since they're defined via the primitives used in a given function.
    #[derive(Copy, Clone)]
    struct Example2;

    impl Resolvable for Example {
        fn resolve(&self, _stack: &mut Stack,  _cache: &mut Cache, position: i32, source: &str) -> (bool, i32) {
            return example1(position, source); // Define which function to run using impl Resolvable
        }
    }

    impl Resolvable for Example2 {
        fn resolve(&self, _stack: &mut Stack,  _cache: &mut Cache, position: i32, source: &str) -> (bool, i32) {
            return example2(position, source);
        }
    }

    fn example1(position: i32, source: &str) -> (bool, i32) {
        let t1 = Example2;
        let t2 = _Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let t3 = _Sequence {
            arg_lhs: t1,
            arg_rhs: t2,
        };
        let mut cache = Cache::new(100, 1);
        let mut stack = Stack::new(100,100);

        let s = t3.resolve(&mut stack, &mut cache, position, source); // Each Top Level Rule can still call other Top Level Rules as well as primitives.
        return s;
    }

    fn example2(position: i32, source: &str) -> (bool, i32) {
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let mut cache = Cache::new(100, 1);
        let mut stack = Stack::new(100,100);

        let s = t.resolve(&mut stack, &mut cache, position, source);
        return s;
    }

    fn example3(position: i32, source: &str) -> (bool, i32) {
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
        let mut cache = Cache::new(100, 1);
        let mut stack = Stack::new(100,100);

        let s = b.resolve(&mut stack, &mut cache, position, source);
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
        let position: i32 = 0;
        let s = example1(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 2);
    }

    #[test]
    fn test_example_false() {
        let source = "Hfllo World";
        let position: i32 = 0;
        let s = example1(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_example_false2() {
        let source = "fello World";
        let position: i32 = 0;
        let s = example1(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }
}
